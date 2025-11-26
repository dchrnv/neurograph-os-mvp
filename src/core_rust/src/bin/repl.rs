use neurograph_core::{
    adapters::console::{ConsoleConfig, ConsoleInputAdapter, ConsoleOutputAdapter},
    adapters::{OutputAdapter, OutputContext},
    bootstrap::{BootstrapConfig, BootstrapLibrary},
    gateway::Gateway,
    GatewayConfig,
    ProcessedSignal,
};
use std::sync::RwLock;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::timeout;

/// Print welcome banner
fn print_welcome() {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║           NeuroGraph OS v0.36.0 - REPL                   ║");
    println!("║     Когнитивная архитектура с Gateway v1.0               ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    println!("Type /help for commands, /quit to exit\n");
}

/// Print help message
fn print_help() {
    println!("\n📚 Available Commands:\n");
    println!("  /help       - Show this help message");
    println!("  /status     - Show system status");
    println!("  /stats      - Show Gateway statistics");
    println!("  /quit       - Exit REPL");
    println!("  /exit       - Exit REPL (alias for /quit)");
    println!("\nOr just type any text to query the system!\n");
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

/// Process a command
async fn process_command(
    cmd: &str,
    _args: Vec<&str>,
    gateway: &Arc<Gateway>,
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

            match process_command(cmd, args, &gateway).await {
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
    // Initialize Bootstrap Library
    let bootstrap_config = BootstrapConfig::default();
    let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(bootstrap_config)));

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

    // Run REPL
    run_repl(gateway, output_adapter, signal_rx).await?;

    Ok(())
}
