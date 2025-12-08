// NeuroGraph OS - Write-Ahead Log (WAL) v0.41.0
// Copyright (C) 2024-2025 Chernov Denys
//
// Provides crash-safe persistence through append-only logging.
//
// # Architecture
//
// WAL ensures data durability by writing operations to disk BEFORE they're applied.
// On crash, the system replays the log to restore state.
//
// ## File Format
//
// ```
// [Entry Header (24 bytes)] [Payload (variable)] [Checksum (4 bytes)]
// ```
//
// Entry Header:
// - timestamp: u64 (8 bytes) - Unix timestamp in microseconds
// - entry_type: u8 (1 byte) - Type of operation
// - payload_size: u32 (4 bytes) - Size of payload in bytes
// - reserved: [u8; 11] (11 bytes) - Reserved for future use
//
// ## Entry Types
//
// - 0x01: TokenCreated
// - 0x02: ExperienceAdded
// - 0x03: ConnectionUpdated
// - 0x04: Snapshot (full state dump)
//
// ## Recovery Process
//
// 1. Open WAL file
// 2. Find last valid Snapshot entry
// 3. Replay all entries after snapshot
// 4. Validate checksums
// 5. Apply operations to system state

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info, warn};

/// WAL entry type identifiers
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalEntryType {
    TokenCreated = 0x01,
    ExperienceAdded = 0x02,
    ConnectionUpdated = 0x03,
    Snapshot = 0x04,
}

impl TryFrom<u8> for WalEntryType {
    type Error = WalError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(WalEntryType::TokenCreated),
            0x02 => Ok(WalEntryType::ExperienceAdded),
            0x03 => Ok(WalEntryType::ConnectionUpdated),
            0x04 => Ok(WalEntryType::Snapshot),
            _ => Err(WalError::InvalidEntryType(value)),
        }
    }
}

/// WAL entry header (24 bytes)
#[derive(Debug, Clone)]
pub struct WalEntryHeader {
    /// Timestamp in microseconds since UNIX epoch
    pub timestamp: u64,
    /// Type of entry
    pub entry_type: WalEntryType,
    /// Size of payload in bytes
    pub payload_size: u32,
    /// Reserved for future use
    pub reserved: [u8; 11],
}

impl WalEntryHeader {
    const SIZE: usize = 24;

    pub fn new(entry_type: WalEntryType, payload_size: u32) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        Self {
            timestamp,
            entry_type,
            payload_size,
            reserved: [0; 11],
        }
    }

    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        let mut bytes = [0u8; Self::SIZE];
        bytes[0..8].copy_from_slice(&self.timestamp.to_le_bytes());
        bytes[8] = self.entry_type as u8;
        bytes[9..13].copy_from_slice(&self.payload_size.to_le_bytes());
        bytes[13..24].copy_from_slice(&self.reserved);
        bytes
    }

    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Result<Self, WalError> {
        let timestamp = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let entry_type = WalEntryType::try_from(bytes[8])?;
        let payload_size = u32::from_le_bytes(bytes[9..13].try_into().unwrap());
        let mut reserved = [0u8; 11];
        reserved.copy_from_slice(&bytes[13..24]);

        Ok(Self {
            timestamp,
            entry_type,
            payload_size,
            reserved,
        })
    }
}

/// Complete WAL entry
#[derive(Debug, Clone)]
pub struct WalEntry {
    pub header: WalEntryHeader,
    pub payload: Vec<u8>,
    pub checksum: u32,
}

impl WalEntry {
    pub fn new(entry_type: WalEntryType, payload: Vec<u8>) -> Self {
        let header = WalEntryHeader::new(entry_type, payload.len() as u32);
        let checksum = Self::calculate_checksum(&header, &payload);

        Self {
            header,
            payload,
            checksum,
        }
    }

    /// Calculate CRC32 checksum
    fn calculate_checksum(header: &WalEntryHeader, payload: &[u8]) -> u32 {
        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&header.to_bytes());
        hasher.update(payload);
        hasher.finalize()
    }

    /// Verify checksum integrity
    pub fn verify(&self) -> bool {
        let expected = Self::calculate_checksum(&self.header, &self.payload);
        self.checksum == expected
    }

    /// Serialize entry to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(
            WalEntryHeader::SIZE + self.payload.len() + 4
        );
        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes.extend_from_slice(&self.checksum.to_le_bytes());
        bytes
    }
}

/// WAL writer - append-only log writer
pub struct WalWriter {
    file: File,
    path: PathBuf,
    entries_written: u64,
    bytes_written: u64,
}

impl WalWriter {
    /// Create new WAL writer
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, WalError> {
        let path = path.as_ref().to_path_buf();

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(WalError::IoError)?;

        info!("WAL writer opened: {}", path.display());

        Ok(Self {
            file,
            path,
            entries_written: 0,
            bytes_written: 0,
        })
    }

    /// Append entry to WAL
    pub fn append(&mut self, entry: &WalEntry) -> Result<(), WalError> {
        let start = std::time::Instant::now();
        let bytes = entry.to_bytes();

        self.file.write_all(&bytes).map_err(WalError::IoError)?;

        // fsync for durability (critical operations only)
        if matches!(entry.header.entry_type, WalEntryType::Snapshot) {
            self.file.sync_all().map_err(WalError::IoError)?;
        }

        self.entries_written += 1;
        self.bytes_written += bytes.len() as u64;

        // Update Prometheus metrics (v0.42.0)
        crate::metrics::WAL_ENTRIES_WRITTEN.inc();
        crate::metrics::WAL_WRITE_DURATION.observe(start.elapsed().as_secs_f64());

        debug!(
            entry_type = ?entry.header.entry_type,
            payload_size = entry.payload.len(),
            "WAL entry written"
        );

        Ok(())
    }

    /// Sync all pending writes to disk
    pub fn sync(&mut self) -> Result<(), WalError> {
        self.file.sync_all().map_err(WalError::IoError)
    }

    /// Get statistics
    pub fn stats(&self) -> WalStats {
        WalStats {
            entries_written: self.entries_written,
            bytes_written: self.bytes_written,
            path: self.path.clone(),
        }
    }
}

/// WAL reader - sequential log reader for recovery
pub struct WalReader {
    file: File,
    path: PathBuf,
}

impl WalReader {
    /// Open WAL for reading
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, WalError> {
        let path = path.as_ref().to_path_buf();

        let file = OpenOptions::new()
            .read(true)
            .open(&path)
            .map_err(WalError::IoError)?;

        info!("WAL reader opened: {}", path.display());

        Ok(Self { file, path })
    }

    /// Read next entry from WAL
    pub fn read_entry(&mut self) -> Result<Option<WalEntry>, WalError> {
        // Read header
        let mut header_bytes = [0u8; WalEntryHeader::SIZE];
        match self.file.read_exact(&mut header_bytes) {
            Ok(()) => {}
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                return Ok(None); // End of file
            }
            Err(e) => return Err(WalError::IoError(e)),
        }

        let header = WalEntryHeader::from_bytes(&header_bytes)?;

        // Read payload
        let mut payload = vec![0u8; header.payload_size as usize];
        self.file.read_exact(&mut payload).map_err(WalError::IoError)?;

        // Read checksum
        let mut checksum_bytes = [0u8; 4];
        self.file.read_exact(&mut checksum_bytes).map_err(WalError::IoError)?;
        let checksum = u32::from_le_bytes(checksum_bytes);

        let entry = WalEntry {
            header,
            payload,
            checksum,
        };

        // Verify integrity
        if !entry.verify() {
            error!("WAL entry checksum mismatch");
            return Err(WalError::ChecksumMismatch);
        }

        debug!(
            entry_type = ?entry.header.entry_type,
            payload_size = entry.payload.len(),
            "WAL entry read"
        );

        Ok(Some(entry))
    }

    /// Replay all entries with callback
    pub fn replay<F>(&mut self, mut callback: F) -> Result<usize, WalError>
    where
        F: FnMut(&WalEntry) -> Result<(), WalError>,
    {
        self.file.seek(SeekFrom::Start(0)).map_err(WalError::IoError)?;

        let mut count = 0;
        while let Some(entry) = self.read_entry()? {
            callback(&entry)?;
            count += 1;
            // Update Prometheus metrics (v0.42.0)
            crate::metrics::WAL_ENTRIES_REPLAYED.inc();
        }

        info!("WAL replay complete: {} entries", count);
        Ok(count)
    }
}

/// WAL statistics
#[derive(Debug, Clone)]
pub struct WalStats {
    pub entries_written: u64,
    pub bytes_written: u64,
    pub path: PathBuf,
}

/// WAL errors
#[derive(Debug, thiserror::Error)]
pub enum WalError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Invalid entry type: {0}")]
    InvalidEntryType(u8),

    #[error("Checksum mismatch")]
    ChecksumMismatch,

    #[error("Corrupted WAL file")]
    CorruptedFile,

    #[error("WAL writer closed")]
    WriterClosed,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_wal_entry_header() {
        let header = WalEntryHeader::new(WalEntryType::TokenCreated, 100);
        let bytes = header.to_bytes();
        let decoded = WalEntryHeader::from_bytes(&bytes).unwrap();

        assert_eq!(header.entry_type as u8, decoded.entry_type as u8);
        assert_eq!(header.payload_size, decoded.payload_size);
    }

    #[test]
    fn test_wal_write_read() {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("test.wal");

        // Write
        {
            let mut writer = WalWriter::new(&wal_path).unwrap();
            let entry = WalEntry::new(
                WalEntryType::TokenCreated,
                vec![1, 2, 3, 4, 5],
            );
            writer.append(&entry).unwrap();
            writer.sync().unwrap();
        }

        // Read
        {
            let mut reader = WalReader::new(&wal_path).unwrap();
            let entry = reader.read_entry().unwrap().unwrap();
            assert_eq!(entry.header.entry_type, WalEntryType::TokenCreated);
            assert_eq!(entry.payload, vec![1, 2, 3, 4, 5]);
            assert!(entry.verify());
        }
    }

    #[test]
    fn test_wal_replay() {
        let dir = tempdir().unwrap();
        let wal_path = dir.path().join("test.wal");

        // Write multiple entries
        {
            let mut writer = WalWriter::new(&wal_path).unwrap();
            for i in 0..10 {
                let entry = WalEntry::new(
                    WalEntryType::ExperienceAdded,
                    vec![i; 10],
                );
                writer.append(&entry).unwrap();
            }
            writer.sync().unwrap();
        }

        // Replay
        {
            let mut reader = WalReader::new(&wal_path).unwrap();
            let mut count = 0;
            reader.replay(|entry| {
                assert_eq!(entry.header.entry_type, WalEntryType::ExperienceAdded);
                count += 1;
                Ok(())
            }).unwrap();
            assert_eq!(count, 10);
        }
    }
}
