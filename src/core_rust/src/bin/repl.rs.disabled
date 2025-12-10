use neurograph_core::{
    adapters::console::{ConsoleConfig, ConsoleInputAdapter, ConsoleOutputAdapter},
    adapters::{OutputAdapter, OutputContext},
    adna::{AppraiserConfig, InMemoryADNAReader},
    bootstrap::{BootstrapConfig, BootstrapLibrary},
    experience_stream::ExperienceStream,
    feedback::{DetailedFeedbackType, FeedbackProcessor, FeedbackSignal},
    gateway::Gateway,
    intuition_engine::IntuitionEngine,
    curiosity::{CuriosityDrive, CuriosityConfig},
    install_panic_hook,
    GatewayConfig,
    ProcessedSignal,
};
use std::io::{self, Write};
use std::sync::RwLock;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::mpsc;
use tokio::time::timeout;

/// Print welcome banner
fn print_welcome() {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║           NeuroGraph OS v0.38.0 - REPL                   ║");
    println!("║   Когнитивная архитектура + Curiosity Drive              ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    println!("Type /help for commands, /quit to exit\n");
}

/// Print help message
fn print_help() {
    println!("\n📚 Available Commands:\n");
    println!("  /help       - Show this help message");
    println!("  /status     - Show system status");
    println!("  /stats      - Show Gateway statistics");
    println!("  /curiosity  - Show curiosity drive statistics");
    println!("  /explore    - Manually explore uncertain regions");
    println!("  /quit       - Exit REPL");
    println!("  /exit       - Exit REPL (alias for /quit)");
    println!("\nOr just type any text to query the system!");
    println!("\n💬 After each response, provide feedback:");
    println!("  y  - Positive (helpful)");
    println!("  n  - Negative (not helpful)");
    println!("  c  - Correction (provide correct answer)\n");
}

/// Print system status
async fn print_status(gateway: &Arc<Gateway>) {
    println!("\n📊 System Status:\n");
    println!("  Gateway:");
    println!("    - Pending requests: {}", gateway.pending_count());
    println!("\n  Status: ✅ Running\n");
}

/// Print Gateway statistics
async fn print_stats(gateway: &Arc<Gateway>) {
    let stats = gateway.stats();

    println!("\n📈 Gateway Statistics:\n");
    println!("  Total signals: {}", stats.total_signals);
    println!("    - Text:     {}", stats.text_signals);
    println!("    - Ticks:    {}", stats.tick_signals);
    println!("    - Commands: {}", stats.command_signals);
    println!("    - Feedback: {}", stats.feedback_signals);
    println!();
    println!("  Unknown words: {}", stats.unknown_words);
    println!("  Queue overflows: {}", stats.queue_overflows);
    println!("  Timeouts: {}", stats.timeouts);
    println!("  Errors: {}", stats.errors);
    println!();
    println!(
        "  Avg processing time: {:.2} μs",
        stats.avg_processing_time_us()
    );
    println!("  Success rate: {:.1}%", stats.success_rate() * 100.0);
    println!();
}

/// Print curiosity drive statistics
async fn print_curiosity_stats(curiosity: &Arc<CuriosityDrive>) {
    let stats = curiosity.stats();

    println!("\n🧠 Curiosity Drive Statistics:\n");

    println!("  Uncertainty Tracking:");
    println!("    - Total cells explored: {}", stats.uncertainty.total_cells);
    println!("    - Total visits: {}", stats.uncertainty.total_visits);
    println!("    - Average confidence: {:.3}", stats.uncertainty.avg_confidence);
    println!("    - Average visits per cell: {:.1}", stats.uncertainty.avg_visits);

    println!("\n  Surprise Detection:");
    println!("    - Current surprise: {:.3}", stats.surprise.current_surprise);
    println!("    - Average surprise: {:.3}", stats.surprise.avg_surprise);
    println!("    - Max recent surprise: {:.3}", stats.surprise.max_recent_surprise);
    println!("    - History size: {}", stats.surprise.history_size);
    println!("    - Total events: {}", stats.surprise.total_events);

    println!("\n  Novelty Tracking:");
    println!("    - Unique states seen: {}", stats.novelty.unique_states);
    println!("    - Total observations: {}", stats.novelty.total_observations);
    println!("    - Total unique ever: {}", stats.novelty.total_unique_seen);

    println!("\n  Exploration Queue:");
    println!("    - Queue size: {}", stats.exploration.queue_size);
    println!("    - Total added: {}", stats.exploration.total_added);
    println!("    - Total explored: {}", stats.exploration.total_explored);

    println!("\n  Status:");
    println!("    - Autonomous exploration: {}", if stats.autonomous_enabled { "✅ enabled" } else { "❌ disabled" });

    println!();
}

/// Manually trigger exploration
async fn trigger_exploration(curiosity: &Arc<CuriosityDrive>) {
    println!("\n🔍 Exploring uncertain regions...\n");

    // Find most uncertain regions
    let uncertain_regions = curiosity.find_uncertain_regions(5);

    if uncertain_regions.is_empty() {
        println!("  No uncertain regions found. System is fully confident!\n");
        return;
    }

    println!("  Top {} uncertain regions:\n", uncertain_regions.len());
    for (i, (state, uncertainty)) in uncertain_regions.iter().enumerate() {
        println!("  {}. Uncertainty: {:.3}", i + 1, uncertainty);
        println!("      State: [{:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}]",
            state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7]);
    }

    // Add top region to exploration queue if available
    if let Some((state, score)) = uncertain_regions.first() {
        let target = neurograph_core::curiosity::ExplorationTarget::new(
            *state,
            *score,
            neurograph_core::curiosity::ExplorationReason::Manual,
        );
        curiosity.add_exploration_target(target);
        println!("\n  ✅ Top region added to exploration queue");
    }

    println!();
}

/// Ask for feedback on a response
async fn ask_for_feedback(
    signal_id: u64,
    feedback_processor: &Arc<FeedbackProcessor>,
) -> Result<(), String> {
    print!("\n[y/n/c] Was this helpful? ");
    io::stdout().flush().unwrap();

    let mut feedback_input = String::new();
    io::stdin()
        .read_line(&mut feedback_input)
        .map_err(|e| format!("Failed to read feedback: {}", e))?;

    let feedback_input = feedback_input.trim().to_lowercase();

    if feedback_input.is_empty() {
        // Skip feedback if user just pressed Enter
        return Ok(());
    }

    let feedback_type = match feedback_input.as_str() {
        "y" | "yes" => DetailedFeedbackType::Positive { strength: 1.0 },
        "n" | "no" => DetailedFeedbackType::Negative { strength: 1.0 },
        "c" | "correct" | "correction" => {
            print!("Enter correct answer: ");
            io::stdout().flush().unwrap();

            let mut correction = String::new();
            io::stdin()
                .read_line(&mut correction)
                .map_err(|e| format!("Failed to read correction: {}", e))?;

            DetailedFeedbackType::Correction {
                correct_value: correction.trim().to_string(),
            }
        }
        _ => {
            println!("⚠️  Invalid feedback option. Use y/n/c or press Enter to skip.");
            return Ok(());
        }
    };

    let feedback_signal = FeedbackSignal {
        reference_id: signal_id,
        feedback_type,
        timestamp: SystemTime::now(),
        explanation: None,
    };

    match feedback_processor.process(feedback_signal).await {
        Ok(result) => {
            if result.success {
                println!("✅ Feedback recorded: {}", result.changes_made.join(", "));
            } else {
                println!("⚠️  Feedback partially applied:");
                for change in result.changes_made {
                    println!("  ✅ {}", change);
                }
                for error in result.errors {
                    println!("  ❌ {}", error);
                }
            }
        }
        Err(e) => {
            println!("❌ Feedback error: {}", e);
        }
    }

    Ok(())
}

/// Process a command
async fn process_command(
    cmd: &str,
    _args: Vec<&str>,
    gateway: &Arc<Gateway>,
    curiosity: &Arc<CuriosityDrive>,
) -> Result<bool, String> {
    match cmd {
        "/help" | "/h" => {
            print_help();
            Ok(false)
        }
        "/status" => {
            print_status(gateway).await;
            Ok(false)
        }
        "/stats" => {
            print_stats(gateway).await;
            Ok(false)
        }
        "/curiosity" | "/c" => {
            print_curiosity_stats(curiosity).await;
            Ok(false)
        }
        "/explore" | "/e" => {
            trigger_exploration(curiosity).await;
            Ok(false)
        }
        "/quit" | "/exit" | "/q" => {
            println!("\n👋 Goodbye!\n");
            Ok(true)
        }
        _ => Err(format!("Unknown command: {}", cmd)),
    }
}

/// Main REPL loop
async fn run_repl(
    gateway: Arc<Gateway>,
    output_adapter: Arc<ConsoleOutputAdapter>,
    feedback_processor: Arc<FeedbackProcessor>,
    curiosity: Arc<CuriosityDrive>,
    mut signal_receiver: mpsc::Receiver<ProcessedSignal>,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_adapter = ConsoleInputAdapter::new(gateway.clone());

    print_welcome();

    loop {
        // Print prompt
        print!("> ");
        std::io::Write::flush(&mut std::io::stdout())?;

        // Read input
        let input = match input_adapter.read_line() {
            Ok(line) => line,
            Err(e) => {
                eprintln!("❌ Error reading input: {}", e);
                continue;
            }
        };

        if input.is_empty() {
            continue;
        }

        // Check if it's a command
        if input.starts_with('/') {
            let parts: Vec<&str> = input.split_whitespace().collect();
            let cmd = parts[0];
            let args = parts[1..].to_vec();

            match process_command(cmd, args, &gateway, &curiosity).await {
                Ok(should_quit) => {
                    if should_quit {
                        break;
                    }
                }
                Err(e) => {
                    println!("❌ {}", e);
                    println!("   Type /help for available commands\n");
                }
            }
            continue;
        }

        // Process as text query
        match input_adapter.process_input(input.clone()).await {
            Ok(signal_id) => {
                // Wait for result with timeout
                match timeout(Duration::from_millis(5000), signal_receiver.recv()).await {
                    Ok(Some(processed_signal)) => {
                        if processed_signal.signal_id == signal_id {
                            // For now, just show that we received it
                            // In a full implementation, we'd wait for ActionResult
                            let context = OutputContext::new(
                                signal_id,
                                Some(input),
                                processed_signal.signal_type,
                                processed_signal.source,
                            );

                            // Create mock result for now
                            use neurograph_core::action_executor::ActionResult;

                            let result = ActionResult {
                                success: true,
                                output: serde_json::json!({
                                    "signal_id": signal_id,
                                    "state": processed_signal.state,
                                    "confidence": processed_signal.interpretation_confidence,
                                }),
                                duration_ms: 0,
                                error: None,
                            };

                            match output_adapter.format_output(&result, &context).await {
                                Ok(formatted) => {
                                    if let Err(e) = output_adapter.send(formatted).await {
                                        eprintln!("❌ Output error: {}", e);
                                    } else {
                                        // Ask for feedback after successful output
                                        if let Err(e) = ask_for_feedback(signal_id, &feedback_processor).await {
                                            eprintln!("⚠️  Feedback error: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("❌ Format error: {}", e);
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        println!("❌ Gateway closed\n");
                        break;
                    }
                    Err(_) => {
                        println!("⏱️  Request timed out (5s)\n");
                    }
                }
            }
            Err(e) => {
                println!("❌ {}\n", e);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Install panic hook for production (v0.41.0)
    install_panic_hook();

    // Initialize Bootstrap Library
    let bootstrap_config = BootstrapConfig::default();
    let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(bootstrap_config)));

    // Initialize ExperienceStream
    let experience_stream = Arc::new(RwLock::new(ExperienceStream::new(100_000, 1000)));

    // Initialize IntuitionEngine with builder (v0.39.2)
    let intuition_engine = Arc::new(RwLock::new(
        IntuitionEngine::builder()
            .with_capacity(100_000)
            .with_channel_size(1000)
            .build()
            .expect("Failed to build IntuitionEngine")
    ));

    // Create signal queue
    let (signal_tx, signal_rx) = mpsc::channel::<ProcessedSignal>(1000);

    // Initialize Gateway
    let gateway_config = GatewayConfig::default();
    let gateway = Arc::new(Gateway::new(
        signal_tx,
        bootstrap.clone(),
        gateway_config,
    ));

    // Create output adapter
    let console_config = ConsoleConfig::default();
    let output_adapter = Arc::new(ConsoleOutputAdapter::new(console_config));

    // Create feedback processor
    let feedback_processor = Arc::new(FeedbackProcessor::new(
        bootstrap.clone(),
        experience_stream,
        intuition_engine,
    ));

    // Initialize Curiosity Drive (v0.38.0)
    let curiosity_config = CuriosityConfig::default();
    let curiosity = Arc::new(CuriosityDrive::new(curiosity_config));

    // Run REPL
    run_repl(gateway, output_adapter, feedback_processor, curiosity, signal_rx).await?;

    Ok(())
}
