// NeuroGraph OS - Async Write-Ahead Log (WAL) v0.44.2
// Copyright (C) 2024-2025 Chernov Denys
//
// High-performance async WAL with batching to eliminate fsync bottleneck.
//
// # Architecture
//
// ```
// Application Thread           WAL Writer Thread
//       |                            |
//       | -- send(entry) -----> MPSC Channel (bounded)
//       |                            |
//       |                       Batch Buffer
//       |                       (1000 entries or 10ms)
//       |                            |
//       |                       write_all() + fsync()
//       |                            |
//       | <--- ack (optional) -------
// ```
//
// ## Key Improvements
//
// - **Batching**: 1000 entries per fsync (vs 1 in sync WAL)
// - **Async**: Non-blocking writes via MPSC channel
// - **Backpressure**: Bounded channel prevents memory exhaustion
// - **Configurable**: Tunable batch size and timeout
//
// ## Expected Performance
//
// - Sync WAL: ~44ms per write (fsync bottleneck) = 22 writes/sec
// - Async WAL: ~44ms per 1000 writes = 22,000 writes/sec (1000x improvement)
//
// ## Trade-offs
//
// - **Durability window**: Up to 10ms of data loss on crash (configurable)
// - **Memory overhead**: Bounded channel holds max 10,000 entries in queue
// - **Complexity**: Requires graceful shutdown to flush pending writes

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;
use tracing::{debug, error, info, warn};

use crate::wal::{WalEntry, WalEntryType, WalError, WalStats};

/// Configuration for async WAL writer
#[derive(Debug, Clone)]
pub struct AsyncWalConfig {
    /// Maximum entries to batch before fsync (default: 1000)
    pub batch_size: usize,
    /// Maximum time to wait before flushing batch (default: 10ms)
    pub batch_timeout: Duration,
    /// MPSC channel capacity (default: 10000)
    pub channel_capacity: usize,
    /// Enable fsync on every batch (default: true for durability)
    pub enable_fsync: bool,
}

impl Default for AsyncWalConfig {
    fn default() -> Self {
        Self {
            batch_size: 1000,
            batch_timeout: Duration::from_millis(10),
            channel_capacity: 10_000,
            enable_fsync: true,
        }
    }
}

/// Async WAL writer handle (cheap to clone)
#[derive(Clone)]
pub struct AsyncWalWriter {
    sender: mpsc::Sender<WalCommand>,
    stats: Arc<AsyncWalStats>,
}

impl AsyncWalWriter {
    /// Create new async WAL writer with default config
    pub fn new<P: AsRef<Path>>(path: P) -> Result<(Self, AsyncWalWriterHandle), WalError> {
        Self::with_config(path, AsyncWalConfig::default())
    }

    /// Create new async WAL writer with custom config
    pub fn with_config<P: AsRef<Path>>(
        path: P,
        config: AsyncWalConfig,
    ) -> Result<(Self, AsyncWalWriterHandle), WalError> {
        let path = path.as_ref().to_path_buf();

        // Open file in append mode
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(WalError::IoError)?;

        info!("Async WAL writer opened: {} (batch_size={}, timeout={}ms)",
            path.display(), config.batch_size, config.batch_timeout.as_millis());

        // Create MPSC channel for commands
        let (sender, receiver) = mpsc::channel(config.channel_capacity);

        // Shared statistics
        let stats = Arc::new(AsyncWalStats {
            entries_written: AtomicU64::new(0),
            entries_dropped: AtomicU64::new(0),
            bytes_written: AtomicU64::new(0),
            batches_flushed: AtomicU64::new(0),
            path: path.clone(),
        });

        // Spawn dedicated writer task
        let task_stats = Arc::clone(&stats);
        let batch_size = config.batch_size;  // Clone batch_size before move
        let task_handle = tokio::spawn(async move {
            let mut task = WalWriterTask {
                file,
                config,
                receiver,
                stats: task_stats,
                batch_buffer: Vec::with_capacity(batch_size),
            };
            task.run().await
        });

        let writer = Self { sender, stats: Arc::clone(&stats) };
        let handle = AsyncWalWriterHandle { task_handle, stats };

        Ok((writer, handle))
    }

    /// Append entry to WAL (non-blocking, returns immediately)
    pub async fn append(&self, entry: WalEntry) -> Result<(), WalError> {
        self.sender
            .send(WalCommand::Append(entry))
            .await
            .map_err(|_| WalError::WriterClosed)?;
        Ok(())
    }

    /// Flush all pending writes (blocks until complete)
    pub async fn flush(&self) -> Result<(), WalError> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.sender
            .send(WalCommand::Flush(tx))
            .await
            .map_err(|_| WalError::WriterClosed)?;
        rx.await.map_err(|_| WalError::WriterClosed)??;
        Ok(())
    }

    /// Get current statistics (lock-free)
    pub fn stats(&self) -> WalStats {
        WalStats {
            entries_written: self.stats.entries_written.load(Ordering::Relaxed),
            bytes_written: self.stats.bytes_written.load(Ordering::Relaxed),
            path: self.stats.path.clone(),
        }
    }

    /// Check if writer is still alive
    pub fn is_alive(&self) -> bool {
        !self.sender.is_closed()
    }
}

/// Handle to async WAL writer task (for graceful shutdown)
pub struct AsyncWalWriterHandle {
    task_handle: tokio::task::JoinHandle<Result<(), WalError>>,
    stats: Arc<AsyncWalStats>,
}

impl AsyncWalWriterHandle {
    /// Wait for writer task to complete (blocks until shutdown)
    pub async fn join(self) -> Result<(), WalError> {
        match self.task_handle.await {
            Ok(result) => result,
            Err(e) => {
                error!("WAL writer task panicked: {}", e);
                Err(WalError::WriterClosed)
            }
        }
    }

    /// Get final statistics
    pub fn stats(&self) -> AsyncWalStatsSnapshot {
        AsyncWalStatsSnapshot {
            entries_written: self.stats.entries_written.load(Ordering::Relaxed),
            entries_dropped: self.stats.entries_dropped.load(Ordering::Relaxed),
            bytes_written: self.stats.bytes_written.load(Ordering::Relaxed),
            batches_flushed: self.stats.batches_flushed.load(Ordering::Relaxed),
            path: self.stats.path.clone(),
        }
    }
}

/// Commands sent to WAL writer task
enum WalCommand {
    Append(WalEntry),
    Flush(tokio::sync::oneshot::Sender<Result<(), WalError>>),
}

/// Dedicated task for batched WAL writes
struct WalWriterTask {
    file: File,
    config: AsyncWalConfig,
    receiver: mpsc::Receiver<WalCommand>,
    stats: Arc<AsyncWalStats>,
    batch_buffer: Vec<WalEntry>,
}

impl WalWriterTask {
    async fn run(&mut self) -> Result<(), WalError> {
        info!("WAL writer task started");

        loop {
            // Wait for next command or timeout
            match timeout(self.config.batch_timeout, self.receiver.recv()).await {
                Ok(Some(cmd)) => {
                    match cmd {
                        WalCommand::Append(entry) => {
                            self.batch_buffer.push(entry);

                            // Flush if batch is full
                            if self.batch_buffer.len() >= self.config.batch_size {
                                self.flush_batch()?;
                            }
                        }
                        WalCommand::Flush(respond) => {
                            // Flush pending batch
                            self.flush_batch()?;
                            let _ = respond.send(Ok(()));
                        }
                    }
                }
                Ok(None) => {
                    // Channel closed, flush and exit
                    info!("WAL writer channel closed, flushing final batch");
                    self.flush_batch()?;
                    break;
                }
                Err(_) => {
                    // Timeout - flush partial batch
                    if !self.batch_buffer.is_empty() {
                        debug!("Batch timeout reached, flushing {} entries", self.batch_buffer.len());
                        self.flush_batch()?;
                    }
                }
            }
        }

        info!("WAL writer task stopped");
        Ok(())
    }

    fn flush_batch(&mut self) -> Result<(), WalError> {
        if self.batch_buffer.is_empty() {
            return Ok(());
        }

        let start = std::time::Instant::now();
        let batch_size = self.batch_buffer.len();
        let mut total_bytes = 0;

        // Write all entries in batch
        for entry in self.batch_buffer.drain(..) {
            let bytes = entry.to_bytes();
            total_bytes += bytes.len();
            self.file.write_all(&bytes).map_err(WalError::IoError)?;
        }

        // Single fsync for entire batch
        if self.config.enable_fsync {
            self.file.sync_all().map_err(WalError::IoError)?;
        }

        // Update stats
        self.stats.entries_written.fetch_add(batch_size as u64, Ordering::Relaxed);
        self.stats.bytes_written.fetch_add(total_bytes as u64, Ordering::Relaxed);
        self.stats.batches_flushed.fetch_add(1, Ordering::Relaxed);

        // Update Prometheus metrics (v0.42.0)
        crate::metrics::WAL_ENTRIES_WRITTEN.inc_by(batch_size as u64);
        crate::metrics::WAL_WRITE_DURATION.observe(start.elapsed().as_secs_f64());

        debug!(
            batch_size = batch_size,
            bytes = total_bytes,
            duration_ms = start.elapsed().as_millis(),
            "WAL batch flushed"
        );

        Ok(())
    }
}

/// Shared statistics (lock-free atomics)
struct AsyncWalStats {
    entries_written: AtomicU64,
    entries_dropped: AtomicU64,
    bytes_written: AtomicU64,
    batches_flushed: AtomicU64,
    path: PathBuf,
}

/// Statistics snapshot
#[derive(Debug, Clone)]
pub struct AsyncWalStatsSnapshot {
    pub entries_written: u64,
    pub entries_dropped: u64,
    pub bytes_written: u64,
    pub batches_flushed: u64,
    pub path: PathBuf,
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_async_wal_basic() {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("async_test.wal");

        // Create writer
        let (writer, handle) = AsyncWalWriter::new(&wal_path).unwrap();

        // Write single entry
        let entry = WalEntry::new(WalEntryType::TokenCreated, vec![1, 2, 3, 4, 5]);
        writer.append(entry).await.unwrap();

        // Flush and verify
        writer.flush().await.unwrap();
        let stats = writer.stats();
        assert_eq!(stats.entries_written, 1);

        // Cleanup
        drop(writer);
        handle.join().await.unwrap();
    }

    #[tokio::test]
    async fn test_async_wal_batching() {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("async_batch_test.wal");

        let config = AsyncWalConfig {
            batch_size: 100,
            batch_timeout: Duration::from_millis(50),
            ..Default::default()
        };

        let (writer, handle) = AsyncWalWriter::with_config(&wal_path, config).unwrap();

        // Write 250 entries (should trigger 2 batches + 1 partial)
        for i in 0..250 {
            let entry = WalEntry::new(
                WalEntryType::ExperienceAdded,
                vec![i as u8; 10],
            );
            writer.append(entry).await.unwrap();
        }

        // Flush remaining
        writer.flush().await.unwrap();
        let stats = writer.stats();
        assert_eq!(stats.entries_written, 250);

        // Cleanup
        drop(writer);
        let final_stats = handle.join().await.unwrap();
    }

    #[tokio::test]
    async fn test_async_wal_graceful_shutdown() {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("async_shutdown_test.wal");

        let (writer, handle) = AsyncWalWriter::new(&wal_path).unwrap();

        // Write entries
        for i in 0..50 {
            let entry = WalEntry::new(WalEntryType::TokenCreated, vec![i; 5]);
            writer.append(entry).await.unwrap();
        }

        // Drop writer (closes channel)
        drop(writer);

        // Task should flush pending entries before exit
        handle.join().await.unwrap();

        // Verify all entries were written
        use crate::wal::WalReader;
        let mut reader = WalReader::new(&wal_path).unwrap();
        let mut count = 0;
        reader.replay(|_| {
            count += 1;
            Ok(())
        }).unwrap();
        assert_eq!(count, 50);
    }
}
