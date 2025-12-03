use super::{FormattedOutput, OutputAdapter, OutputContext, OutputError};
use crate::action_executor::ActionResult;
use crate::gateway::Gateway;
use crate::{InputSignal, SignalSource};
use std::io::{self, Write};
use std::sync::Arc;

/// Configuration for console output
#[derive(Debug, Clone)]
pub struct ConsoleConfig {
    /// Show confidence scores
    pub show_confidence: bool,

    /// Maximum results to display
    pub max_results: usize,

    /// Use colored output (ANSI codes)
    pub colorize: bool,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        Self {
            show_confidence: true,
            max_results: 10,
            colorize: true,
        }
    }
}

/// Console output adapter
pub struct ConsoleOutputAdapter {
    config: ConsoleConfig,
}

impl ConsoleOutputAdapter {
    pub fn new(config: ConsoleConfig) -> Self {
        Self { config }
    }
}

#[async_trait::async_trait]
impl OutputAdapter for ConsoleOutputAdapter {
    fn name(&self) -> &str {
        "console"
    }

    async fn format_output(
        &self,
        result: &ActionResult,
        context: &OutputContext,
    ) -> Result<FormattedOutput, OutputError> {
        let mut output = String::new();

        // Check for errors first
        if let Some(error) = &result.error {
            output.push_str(&format!("\n❌ Error: {}\n", error));
            return Ok(FormattedOutput::text(output));
        }

        // Format successful result
        output.push_str(&format!(
            "\n🔍 Query: {}\n",
            context
                .original_input
                .as_deref()
                .unwrap_or("<direct state>")
        ));

        output.push_str(&format!(
            "   Completed in {:.2}ms\n",
            result.duration_ms
        ));

        if result.success {
            output.push_str("   ✅ Success\n");
        } else {
            output.push_str("   ⚠️  Partial success\n");
        }

        // Show output data if available
        if !result.output.is_null() {
            output.push_str(&format!(
                "   Output: {}\n",
                serde_json::to_string_pretty(&result.output)
                    .unwrap_or_else(|_| result.output.to_string())
            ));
        }

        output.push_str("\n");

        Ok(FormattedOutput::text(output))
    }

    async fn send(&self, output: FormattedOutput) -> Result<(), OutputError> {
        if let Some(text) = output.text {
            print!("{}", text);
            io::stdout()
                .flush()
                .map_err(|e| OutputError::IoError(e.to_string()))?;
        }
        Ok(())
    }
}

/// Console input adapter
pub struct ConsoleInputAdapter {
    gateway: Arc<Gateway>,
}

impl ConsoleInputAdapter {
    pub fn new(gateway: Arc<Gateway>) -> Self {
        Self { gateway }
    }

    /// Read a line from stdin
    pub fn read_line(&self) -> Result<String, io::Error> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    /// Process input line and inject into Gateway
    pub async fn process_input(&self, input: String) -> Result<u64, String> {
        if input.is_empty() {
            return Err("Empty input".to_string());
        }

        let signal = InputSignal::Text {
            content: input,
            source: SignalSource::Console,
            metadata: None,
        };

        let (receipt, _receiver) = self
            .gateway
            .inject(signal)
            .await
            .map_err(|e| format!("Gateway error: {}", e))?;

        Ok(receipt.signal_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap::BootstrapConfig;
    use crate::bootstrap::BootstrapLibrary;
    use crate::GatewayConfig; // Import directly from lib.rs
    use parking_lot::RwLock;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_console_output_format() {
        let adapter = ConsoleOutputAdapter::new(ConsoleConfig::default());

        let result = ActionResult {
            success: true,
            output: serde_json::json!({}),
            duration_ms: 1,
            error: None,
        };

        let context = OutputContext::new(
            1,
            Some("test query".to_string()),
            crate::SignalType::SemanticQuery, // Import directly from lib.rs
            SignalSource::Console,
        );

        let formatted = adapter.format_output(&result, &context).await.unwrap();
        assert!(formatted.text.is_some());
        let text = formatted.text.unwrap();
        assert!(text.contains("test query"));
    }

    #[tokio::test]
    async fn test_console_input_adapter() {
        let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(
            BootstrapConfig::default(),
        )));
        let (tx, _rx) = mpsc::channel(100);
        let gateway = Arc::new(Gateway::new(tx, bootstrap, GatewayConfig::default()));

        let adapter = ConsoleInputAdapter::new(gateway);

        // Test empty input
        let result = adapter.process_input("".to_string()).await;
        assert!(result.is_err());
    }
}
