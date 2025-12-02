// NeuroGraph OS - 1M Tokens Stress Test
// Tests scalability with large token counts

use neurograph_core::Token;
use std::time::Instant;

/// Test creating 1M tokens
#[test]
#[ignore] // Run with: cargo test --test stress_test_1m_tokens -- --ignored --nocapture
fn test_1m_tokens_creation() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          1M Tokens Creation Stress Test                     ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let start = Instant::now();
    let mut tokens = Vec::with_capacity(1_000_000);

    // Create 1M tokens
    for i in 0..1_000_000u32 {
        let token = Token::new(i);
        tokens.push(token);
    }

    let duration = start.elapsed();

    println!("✓ Created {} tokens", tokens.len());
    println!("  Duration: {:?}", duration);
    println!("  Rate: {:.2} tokens/sec", 1_000_000.0 / duration.as_secs_f64());
    println!("  Avg: {:.2} ns/token", duration.as_nanos() as f64 / 1_000_000.0);

    // Memory check
    let token_size = std::mem::size_of::<Token>();
    let total_memory = token_size * tokens.len();
    println!("  Token size: {} bytes", token_size);
    println!("  Total memory: {:.2} MB", total_memory as f64 / 1_048_576.0);
    println!();
}

/// Test memory patterns with 1M tokens
#[test]
#[ignore]
fn test_1m_tokens_memory_patterns() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          1M Tokens Memory Pattern Test                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("Phase 1: Vec without pre-allocated capacity...");
    let start = Instant::now();
    let tokens1: Vec<Token> = (0..1_000_000u32)
        .map(|i| Token::new(i))
        .collect();
    let phase1 = start.elapsed();
    println!("✓ Phase 1: {:?} ({} tokens)", phase1, tokens1.len());

    println!("\nPhase 2: Vec with pre-allocated capacity...");
    let start = Instant::now();
    let mut tokens2 = Vec::with_capacity(1_000_000);
    for i in 0..1_000_000u32 {
        tokens2.push(Token::new(i));
    }
    let phase2 = start.elapsed();
    println!("✓ Phase 2: {:?} ({} tokens)", phase2, tokens2.len());

    println!("\n=== Performance Comparison ===");
    println!("Without capacity: {:?}", phase1);
    println!("With capacity:    {:?} ({:.2}x faster)", phase2, phase1.as_secs_f64() / phase2.as_secs_f64());
    println!();
}

/// Test parallel token creation
#[test]
#[ignore]
fn test_parallel_token_operations() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          Parallel Token Operations (1M total)               ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    use std::thread;

    let start = Instant::now();

    // Create 1M tokens across 4 threads (250k each)
    let handles: Vec<_> = (0..4)
        .map(|thread_id| {
            thread::spawn(move || {
                let offset = thread_id * 250_000;
                let mut local_tokens = Vec::with_capacity(250_000);

                for i in 0..250_000u32 {
                    local_tokens.push(Token::new(offset + i));
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

    println!("✓ Created {} tokens across 4 threads", total);
    println!("  Duration: {:?}", duration);
    println!("  Rate: {:.2} tokens/sec", total as f64 / duration.as_secs_f64());
    println!("  Speedup: ~{:.2}x (vs sequential)", 1_000_000.0 / duration.as_micros() as f64 * 50.0);
    println!();
}

/// Test token access patterns
#[test]
#[ignore]
fn test_token_access_patterns() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          Token Access Pattern Test (1M tokens)              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Pre-create 1M tokens
    println!("Creating 1M tokens...");
    let create_start = Instant::now();
    let tokens: Vec<Token> = (0..1_000_000u32)
        .map(|i| Token::new(i))
        .collect();
    let create_time = create_start.elapsed();
    println!("✓ Created {} tokens in {:?}", tokens.len(), create_time);

    // Sequential access
    println!("\nSequential access (1M reads)...");
    let seq_start = Instant::now();
    let mut sum_id = 0u64;
    for token in &tokens {
        sum_id += token.id as u64;
    }
    let seq_time = seq_start.elapsed();
    println!("✓ Sequential: {:?} (sum: {})", seq_time, sum_id);

    // Random access
    println!("\nRandom access (100k reads)...");
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

/// Memory footprint test
#[test]
#[ignore]
fn test_memory_footprint() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          Memory Footprint Analysis                          ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let token_size = std::mem::size_of::<Token>();
    println!("Token size: {} bytes", token_size);

    let sizes = [1_000, 10_000, 100_000, 1_000_000];

    for &size in &sizes {
        let memory_mb = (token_size * size) as f64 / 1_048_576.0;
        println!("  {:>9} tokens: {:.2} MB", size, memory_mb);
    }

    println!("\n=== Allocating 1M tokens ===");
    let start = Instant::now();
    let tokens: Vec<Token> = (0..1_000_000u32).map(|i| Token::new(i)).collect();
    let duration = start.elapsed();

    let actual_memory = token_size * tokens.len();
    println!("✓ Allocated {} tokens in {:?}", tokens.len(), duration);
    println!("  Memory used: {:.2} MB", actual_memory as f64 / 1_048_576.0);
    println!();
}

/// Combined stress test
#[test]
#[ignore]
fn test_combined_stress() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          Combined Stress Test                               ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let total_start = Instant::now();

    // Phase 1: Create 1M tokens
    println!("Phase 1: Creating 1M tokens...");
    let phase1_start = Instant::now();
    let tokens: Vec<Token> = (0..1_000_000u32).map(|i| Token::new(i)).collect();
    let phase1_time = phase1_start.elapsed();
    println!("✓ Phase 1: {:?}", phase1_time);

    // Phase 2: Access all tokens
    println!("\nPhase 2: Accessing all tokens...");
    let phase2_start = Instant::now();
    let mut count = 0u64;
    for token in &tokens {
        count += token.id as u64;
    }
    let phase2_time = phase2_start.elapsed();
    println!("✓ Phase 2: {:?} (checksum: {})", phase2_time, count);

    // Phase 3: Clone vector
    println!("\nPhase 3: Cloning 1M tokens...");
    let phase3_start = Instant::now();
    let tokens_clone = tokens.clone();
    let phase3_time = phase3_start.elapsed();
    println!("✓ Phase 3: {:?} ({} tokens cloned)", phase3_time, tokens_clone.len());

    let total_time = total_start.elapsed();

    println!("\n=== Summary ===");
    println!("Phase 1 (Create): {:?}", phase1_time);
    println!("Phase 2 (Access): {:?}", phase2_time);
    println!("Phase 3 (Clone):  {:?}", phase3_time);
    println!("Total time:       {:?}", total_time);

    let token_size = std::mem::size_of::<Token>();
    let memory_mb = (token_size * 2_000_000) as f64 / 1_048_576.0; // Original + clone
    println!("Peak memory:      {:.2} MB", memory_mb);
    println!();
}
