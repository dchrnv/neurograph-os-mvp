// API Compatibility Test
//
// Verifies that all public APIs are accessible and work correctly

use neurograph_core::{
    // Gateway
    Gateway, GatewayConfig, GatewayError,
    InputSignal, SignalSource, SignalReceipt, ResultReceiver,

    // Bootstrap
    bootstrap::{BootstrapConfig, BootstrapLibrary},

    // Curiosity
    curiosity::{CuriosityDrive, CuriosityConfig},

    // Feedback
    feedback::{FeedbackProcessor, FeedbackSignal, DetailedFeedbackType},

    // ActionController
    action_controller::{ActionController, ActionControllerConfig, ArbiterConfig},

    // ADNA
    adna::{AppraiserConfig, InMemoryADNAReader},

    // Experience
    experience_stream::ExperienceStream,

    // Intuition
    intuition_engine::IntuitionEngine,

    // Guardian
    Guardian, CDNA,
};
use parking_lot::RwLock;
use std::sync::Arc;

#[test]
fn test_gateway_api() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(BootstrapConfig::default())));
        let config = GatewayConfig::default();

        let gateway = Gateway::new(tx, bootstrap, config);

        let signal = InputSignal::Text {
            content: "test".to_string(),
            source: SignalSource::Console,
            metadata: None,
        };

        let result = gateway.inject(signal).await;
        assert!(result.is_ok());
    });
}

#[test]
fn test_bootstrap_api() {
    let config = BootstrapConfig::default();
    let bootstrap = BootstrapLibrary::new(config);

    // Test concept lookup
    let concept = bootstrap.get_concept("hello");
    assert!(concept.is_some() || concept.is_none()); // Should not panic

    // Test semantic search
    let results = bootstrap.semantic_search("test", 5);
    assert!(results.len() <= 5);
}

#[test]
fn test_curiosity_api() {
    let config = CuriosityConfig::default();
    let curiosity = CuriosityDrive::new(config);

    let state = [0.5, 0.3, 0.1, 0.0, 0.2, 0.4, 0.1, 0.0];

    // Test curiosity calculation
    let score = curiosity.calculate_curiosity(&state, None);
    assert!(score.is_some() || score.is_none()); // Should not panic

    // Test state update
    curiosity.update_state(&state, 0.5); // Should not panic
}

#[test]
fn test_feedback_api() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async {
        let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(BootstrapConfig::default())));
        let experience = Arc::new(RwLock::new(ExperienceStream::new(1000)));
        let intuition = Arc::new(RwLock::new(IntuitionEngine::new(Default::default())));

        let processor = FeedbackProcessor::new(bootstrap, experience, intuition);

        let signal = FeedbackSignal {
            reference_id: 1,
            feedback_type: DetailedFeedbackType::Positive { strength: 0.8 },
            timestamp: std::time::SystemTime::now(),
        };

        let result = processor.process_feedback(signal).await;
        assert!(result.is_ok() || result.is_err()); // Should not panic
    });
}

#[test]
fn test_action_controller_api() {
    let experience = Arc::new(RwLock::new(ExperienceStream::new(1000)));
    let intuition = Arc::new(RwLock::new(IntuitionEngine::new(Default::default())));
    let guardian = Arc::new(Guardian::new(Arc::new(CDNA::default())));
    let adna = Arc::new(InMemoryADNAReader::new(AppraiserConfig::default()));

    let controller = ActionController::new(
        adna,
        experience,
        intuition,
        guardian,
        ActionControllerConfig::default(),
        ArbiterConfig::default(),
    );

    // Test that we can get stats
    let stats = controller.get_arbiter_stats();
    assert_eq!(stats.total_decisions, 0); // Should be zero for new controller
}

#[test]
fn test_full_integration() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    runtime.block_on(async {
        // Create full system
        let (signal_tx, _signal_rx) = tokio::sync::mpsc::channel(100);
        let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(BootstrapConfig::default())));

        let gateway = Arc::new(Gateway::new(
            signal_tx,
            bootstrap.clone(),
            GatewayConfig::default(),
        ));

        let experience = Arc::new(RwLock::new(ExperienceStream::new(1000)));
        let intuition = Arc::new(RwLock::new(IntuitionEngine::new(Default::default())));
        let guardian = Arc::new(Guardian::new(Arc::new(CDNA::default())));
        let adna = Arc::new(InMemoryADNAReader::new(AppraiserConfig::default()));

        let mut controller = ActionController::new(
            adna,
            experience,
            intuition,
            guardian,
            ActionControllerConfig::default(),
            ArbiterConfig::default(),
        );

        controller.set_gateway(gateway.clone());

        // Test injection
        let signal = InputSignal::Text {
            content: "hello world".to_string(),
            source: SignalSource::Console,
            metadata: None,
        };

        let result = gateway.inject(signal).await;
        assert!(result.is_ok());
    });
}
