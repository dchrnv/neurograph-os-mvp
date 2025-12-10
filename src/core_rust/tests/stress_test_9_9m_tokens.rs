// NeuroGraph OS - 9.9M Tokens Stress Test
// Tests scalability with large token counts (near 10M limit)

use neurograph_core::Token;
use std::time::Instant;

/// Test creating 9.9M tokens
#[test]
#[ignore] // Run with: cargo test --test stress_test_9_9m_tokens -- --ignored --nocapture
fn test_9_9m_tokens_creation() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          9.9M Tokens Creation Stress Test                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let count = 9_900_000;
    let start = Instant::now();
    let mut tokens = Vec::with_capacity(count);

    // Create 9.9M tokens
    for i in 0..count as u32 {
        let token = Token::new(i);
        tokens.push(token);
    }

    let duration = start.elapsed();

    println!("✓ Created {} tokens", tokens.len());
    println!("  Duration: {:?}", duration);
    println!("  Rate: {:.2} tokens/sec", count as f64 / duration.as_secs_f64());
    println!("  Avg: {:.2} ns/token", duration.as_nanos() as f64 / count as f64);

    // Memory check
    let token_size = std::mem::size_of::<Token>();
    let total_memory = token_size * tokens.len();
    println!("  Token size: {} bytes", token_size);
    println!("  Total memory: {:.2} MB", total_memory as f64 / 1_048_576.0);
    println!("  Total memory: {:.2} GB", total_memory as f64 / 1_073_741_824.0);
    println!();
}

/// Test memory patterns with 9.9M tokens
#[test]
#[ignore]
fn test_9_9m_tokens_memory_patterns() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          9.9M Tokens Memory Pattern Test                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let count = 9_900_000;

    println!("Phase 1: Vec without pre-allocated capacity...");
    let start = Instant::now();
    let tokens1: Vec<Token> = (0..count as u32)
        .map(|i| Token::new(i))
        .collect();
    let phase1 = start.elapsed();
    println!("✓ Phase 1: {:?} ({} tokens)", phase1, tokens1.len());

    println!("\nPhase 2: Vec with pre-allocated capacity...");
    let start = Instant::now();
    let mut tokens2 = Vec::with_capacity(count);
    for i in 0..count as u32 {
        tokens2.push(Token::new(i));
    }
    let phase2 = start.elapsed();
    println!("✓ Phase 2: {:?} ({} tokens)", phase2, tokens2.len());

    println!("\n=== Performance Comparison ===");
    println!("Without capacity: {:?}", phase1);
    println!("With capacity:    {:?} ({:.2}x faster)", phase2, phase1.as_secs_f64() / phase2.as_secs_f64());
    println!();
}

/// Test parallel token creation (9.9M total)
#[test]
#[ignore]
fn test_parallel_token_operations_9_9m() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          Parallel Token Operations (9.9M total)             ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    use std::thread;

    let start = Instant::now();

    // Create 9.9M tokens across 8 threads
    let threads = 8;
    let per_thread = 9_900_000 / threads;

    let handles: Vec<_> = (0..threads)
        .map(|thread_id| {
            thread::spawn(move || {
                let offset = thread_id * per_thread;
                let mut local_tokens = Vec::with_capacity(per_thread);

                for i in 0..per_thread as u32 {
                    local_tokens.push(Token::new(offset as u32 + i));
                }

                local_tokens
            })
        })
        .collect();

    let mut total = 0;
    for handle in handles {
        let tokens = handle.join().unwrap();
        total += tokens.len();
    }

    let duration = start.elapsed();

    println!("✓ Created {} tokens across {} threads", total, threads);
    println!("  Duration: {:?}", duration);
    println!("  Rate: {:.2} tokens/sec", total as f64 / duration.as_secs_f64());
    println!("  Throughput: {:.2} M tokens/sec", total as f64 / duration.as_secs_f64() / 1_000_000.0);
    println!();
}

/// Test token access patterns (9.9M tokens)
#[test]
#[ignore]
fn test_token_access_patterns_9_9m() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          Token Access Pattern Test (9.9M tokens)            ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let count = 9_900_000;

    // Pre-create 9.9M tokens
    println!("Creating {} tokens...", count);
    let create_start = Instant::now();
    let tokens: Vec<Token> = (0..count as u32)
        .map(|i| Token::new(i))
        .collect();
    let create_time = create_start.elapsed();
    println!("✓ Created {} tokens in {:?}", tokens.len(), create_time);

    // Sequential access
    println!("\nSequential access ({} reads)...", count);
    let seq_start = Instant::now();
    let mut sum_id = 0u64;
    for token in &tokens {
        sum_id += token.id as u64;
    }
    let seq_time = seq_start.elapsed();
    println!("✓ Sequential: {:?} (sum: {})", seq_time, sum_id);

    // Random access (every 10th token = 990k reads)
    println!("\nRandom access (990k reads)...");
    let rand_start = Instant::now();
    let mut sum_id2 = 0u64;
    for i in (0..tokens.len()).step_by(10) {
        sum_id2 += tokens[i].id as u64;
    }
    let rand_time = rand_start.elapsed();
    println!("✓ Random: {:?} (sum: {})", rand_time, sum_id2);

    println!("\n=== Summary ===");
    println!("Creation: {:?}", create_time);
    println!("Sequential: {:?}", seq_time);
    println!("Random: {:?}", rand_time);
    println!();
}

/// Memory footprint test for 9.9M
#[test]
#[ignore]
fn test_memory_footprint_9_9m() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          Memory Footprint Analysis (9.9M)                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let token_size = std::mem::size_of::<Token>();
    println!("Token size: {} bytes", token_size);

    let sizes = [1_000_000, 5_000_000, 9_900_000, 10_000_000];

    println!("\n=== Memory estimates ===");
    for &size in &sizes {
        let memory_mb = (token_size * size) as f64 / 1_048_576.0;
        let memory_gb = memory_mb / 1024.0;
        println!("  {:>10} tokens: {:.2} MB ({:.3} GB)", size, memory_mb, memory_gb);
    }

    println!("\n=== Allocating 9.9M tokens ===");
    let count = 9_900_000;
    let start = Instant::now();
    let tokens: Vec<Token> = (0..count as u32).map(|i| Token::new(i)).collect();
    let duration = start.elapsed();

    let actual_memory = token_size * tokens.len();
    println!("✓ Allocated {} tokens in {:?}", tokens.len(), duration);
    println!("  Memory used: {:.2} MB", actual_memory as f64 / 1_048_576.0);
    println!("  Memory used: {:.3} GB", actual_memory as f64 / 1_073_741_824.0);
    println!();
}

/// Combined stress test for 9.9M
#[test]
#[ignore]
fn test_combined_stress_9_9m() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          Combined Stress Test (9.9M)                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let count = 9_900_000;
    let total_start = Instant::now();

    // Phase 1: Create 9.9M tokens
    println!("Phase 1: Creating {} tokens...", count);
    let phase1_start = Instant::now();
    let tokens: Vec<Token> = (0..count as u32).map(|i| Token::new(i)).collect();
    let phase1_time = phase1_start.elapsed();
    println!("✓ Phase 1: {:?}", phase1_time);

    // Phase 2: Access all tokens
    println!("\nPhase 2: Accessing all tokens...");
    let phase2_start = Instant::now();
    let mut checksum = 0u64;
    for token in &tokens {
        checksum += token.id as u64;
    }
    let phase2_time = phase2_start.elapsed();
    println!("✓ Phase 2: {:?} (checksum: {})", phase2_time, checksum);

    // Phase 3: Partial clone (first 1M to avoid OOM)
    println!("\nPhase 3: Cloning first 1M tokens...");
    let phase3_start = Instant::now();
    let tokens_clone = tokens[..1_000_000].to_vec();
    let phase3_time = phase3_start.elapsed();
    println!("✓ Phase 3: {:?} ({} tokens cloned)", phase3_time, tokens_clone.len());

    let total_time = total_start.elapsed();

    println!("\n=== Summary ===");
    println!("Phase 1 (Create): {:?}", phase1_time);
    println!("Phase 2 (Access): {:?}", phase2_time);
    println!("Phase 3 (Clone):  {:?}", phase3_time);
    println!("Total time:       {:?}", total_time);

    let token_size = std::mem::size_of::<Token>();
    let memory_mb = (token_size * (count + 1_000_000)) as f64 / 1_048_576.0;
    println!("Peak memory:      {:.2} MB ({:.3} GB)", memory_mb, memory_mb / 1024.0);
    println!();
}
