// NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Reward Appraisers for Intuition Module v2.2
//!
//! This module implements the 4 core appraisers that evaluate experience events
//! and assign partial rewards based on different criteria:
//!
//! 1. **HomeostasisAppraiser** - Penalizes deviations from target ranges
//! 2. **CuriosityAppraiser** - Rewards novelty and exploration
//! 3. **EfficiencyAppraiser** - Penalizes resource usage
//! 4. **GoalDirectedAppraiser** - Rewards progress toward goals
//!
//! Each appraiser runs as an independent async task, subscribing to the
//! ExperienceStream and writing rewards to dedicated slots in ExperienceEvent.

use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;

use crate::adna::{
    ADNAReader, ADNAError,
    HomeostasisParams, CuriosityParams, EfficiencyParams, GoalDirectedParams,
};
use crate::experience_stream::{ExperienceEvent, ExperienceWriter, AppraiserType};
use crate::coordinates::CoordinateExt;

// ============================================================================
// HomeostasisAppraiser
// ============================================================================

/// Homeostasis Appraiser
///
/// Evaluates deviations from target ranges for L5 (Cognitive Load),
/// L6 (Certainty), and L8 (Coherence). Penalizes states outside healthy ranges.
pub struct HomeostasisAppraiser {
    dna_reader: Arc<dyn ADNAReader>,
    experience_writer: Arc<dyn ExperienceWriter>,
    event_receiver: broadcast::Receiver<ExperienceEvent>,
}

impl HomeostasisAppraiser {
    pub fn new(
        dna_reader: Arc<dyn ADNAReader>,
        experience_writer: Arc<dyn ExperienceWriter>,
        event_receiver: broadcast::Receiver<ExperienceEvent>,
    ) -> Self {
        Self {
            dna_reader,
            experience_writer,
            event_receiver,
        }
    }

    /// Main run loop - processes events until channel closes
    pub async fn run(mut self) {
        loop {
            match self.event_receiver.recv().await {
                Ok(event) => {
                    if let Err(e) = self.process_event(event).await {
                        eprintln!("[HomeostasisAppraiser] Error processing event: {}", e);
                    }
                }
                Err(broadcast::error::RecvError::Closed) => {
                    println!("[HomeostasisAppraiser] Channel closed, shutting down");
                    break;
                }
                Err(broadcast::error::RecvError::Lagged(skipped)) => {
                    eprintln!("[HomeostasisAppraiser] Lagged by {} events", skipped);
                    continue;
                }
            }
        }
    }

    async fn process_event(&self, event: ExperienceEvent) -> Result<(), ADNAError> {
        // Load parameters from ADNA
        let params = self.dna_reader.get_homeostasis_params().await?;

        // Calculate reward
        let reward = self.calculate_reward(&event, &params);

        // Write reward if significant
        if reward.abs() > 1e-6 {
            let _ = self.experience_writer
                .set_appraiser_reward(event.sequence_number as u64, AppraiserType::Homeostasis, reward);
        }

        Ok(())
    }

    fn calculate_reward(&self, event: &ExperienceEvent, params: &HomeostasisParams) -> f32 {
        let mut total_penalty = 0.0;

        // Penalty for L5 Cognitive Load deviation
        let cognitive_load = event.l5_cognitive_load();
        let (cl_min, cl_max) = params.cognitive_load_range;
        if cognitive_load < cl_min {
            total_penalty += (cl_min - cognitive_load).powi(2);
        } else if cognitive_load > cl_max {
            total_penalty += (cognitive_load - cl_max).powi(2);
        }

        // Penalty for L6 Certainty deviation
        let certainty = event.l6_certainty();
        let (cert_min, cert_max) = params.certainty_range;
        if certainty < cert_min {
            total_penalty += (cert_min - certainty).powi(2);
        } else if certainty > cert_max {
            total_penalty += (certainty - cert_max).powi(2);
        }

        // Penalty for L8 Coherence deviation
        let coherence = event.l8_coherence();
        let (coh_min, coh_max) = params.coherence_range;
        if coherence < coh_min {
            total_penalty += (coh_min - coherence).powi(2);
        } else if coherence > coh_max {
            total_penalty += (coherence - coh_max).powi(2);
        }

        // Return negative weighted penalty
        -params.weight * params.penalty_multiplier * total_penalty
    }
}

// ============================================================================
// CuriosityAppraiser
// ============================================================================

/// Curiosity Appraiser
///
/// Rewards novelty (L2) to encourage exploration of unknown states.
pub struct CuriosityAppraiser {
    dna_reader: Arc<dyn ADNAReader>,
    experience_writer: Arc<dyn ExperienceWriter>,
    event_receiver: broadcast::Receiver<ExperienceEvent>,
}

impl CuriosityAppraiser {
    pub fn new(
        dna_reader: Arc<dyn ADNAReader>,
        experience_writer: Arc<dyn ExperienceWriter>,
        event_receiver: broadcast::Receiver<ExperienceEvent>,
    ) -> Self {
        Self {
            dna_reader,
            experience_writer,
            event_receiver,
        }
    }

    pub async fn run(mut self) {
        loop {
            match self.event_receiver.recv().await {
                Ok(event) => {
                    if let Err(e) = self.process_event(event).await {
                        eprintln!("[CuriosityAppraiser] Error processing event: {}", e);
                    }
                }
                Err(broadcast::error::RecvError::Closed) => {
                    println!("[CuriosityAppraiser] Channel closed, shutting down");
                    break;
                }
                Err(broadcast::error::RecvError::Lagged(skipped)) => {
                    eprintln!("[CuriosityAppraiser] Lagged by {} events", skipped);
                    continue;
                }
            }
        }
    }

    async fn process_event(&self, event: ExperienceEvent) -> Result<(), ADNAError> {
        let params = self.dna_reader.get_curiosity_params().await?;
        let reward = self.calculate_reward(&event, &params);

        if reward.abs() > 1e-6 {
            let _ = self.experience_writer
                .set_appraiser_reward(event.sequence_number as u64, AppraiserType::Curiosity, reward);
        }

        Ok(())
    }

    fn calculate_reward(&self, event: &ExperienceEvent, params: &CuriosityParams) -> f32 {
        let novelty = event.l2_novelty();

        // Only reward novelty above threshold
        if novelty > params.novelty_threshold {
            let novelty_excess = novelty - params.novelty_threshold;
            params.weight * params.reward_multiplier * novelty_excess
        } else {
            0.0
        }
    }
}

// ============================================================================
// EfficiencyAppraiser
// ============================================================================

/// Efficiency Appraiser
///
/// Penalizes resource usage: motor activity (L3), cognitive load (L5),
/// and creation of new entities.
pub struct EfficiencyAppraiser {
    dna_reader: Arc<dyn ADNAReader>,
    experience_writer: Arc<dyn ExperienceWriter>,
    event_receiver: broadcast::Receiver<ExperienceEvent>,
}

impl EfficiencyAppraiser {
    pub fn new(
        dna_reader: Arc<dyn ADNAReader>,
        experience_writer: Arc<dyn ExperienceWriter>,
        event_receiver: broadcast::Receiver<ExperienceEvent>,
    ) -> Self {
        Self {
            dna_reader,
            experience_writer,
            event_receiver,
        }
    }

    pub async fn run(mut self) {
        loop {
            match self.event_receiver.recv().await {
                Ok(event) => {
                    if let Err(e) = self.process_event(event).await {
                        eprintln!("[EfficiencyAppraiser] Error processing event: {}", e);
                    }
                }
                Err(broadcast::error::RecvError::Closed) => {
                    println!("[EfficiencyAppraiser] Channel closed, shutting down");
                    break;
                }
                Err(broadcast::error::RecvError::Lagged(skipped)) => {
                    eprintln!("[EfficiencyAppraiser] Lagged by {} events", skipped);
                    continue;
                }
            }
        }
    }

    async fn process_event(&self, event: ExperienceEvent) -> Result<(), ADNAError> {
        let params = self.dna_reader.get_efficiency_params().await?;
        let reward = self.calculate_reward(&event, &params);

        if reward.abs() > 1e-6 {
            let _ = self.experience_writer
                .set_appraiser_reward(event.sequence_number as u64, AppraiserType::Efficiency, reward);
        }

        Ok(())
    }

    fn calculate_reward(&self, event: &ExperienceEvent, params: &EfficiencyParams) -> f32 {
        let mut total_cost = 0.0;

        // Cost for motor activity (L3 velocity and acceleration)
        let velocity = event.l3_velocity();
        let acceleration = event.l3_acceleration();
        total_cost += params.motor_cost_factor * (velocity.powi(2) + acceleration.powi(2));

        // Cost for cognitive load
        let cognitive_load = event.l5_cognitive_load();
        total_cost += params.cognitive_cost_factor * cognitive_load;

        // Cost for creation events (event_type is u16, not string)
        // For MVP, we can add creation cost based on flags or skip this check
        // Future: define event_type constants for token_created, connection_created

        // Return negative weighted cost
        -params.weight * total_cost
    }
}

// ============================================================================
// GoalDirectedAppraiser (MVP)
// ============================================================================

/// Goal-Directed Appraiser (MVP version)
///
/// Simplified version that provides immediate rewards for goal-related events.
/// Full retroactive trajectory-based rewards deferred for future implementation.
pub struct GoalDirectedAppraiser {
    dna_reader: Arc<dyn ADNAReader>,
    experience_writer: Arc<dyn ExperienceWriter>,
    event_receiver: broadcast::Receiver<ExperienceEvent>,
}

impl GoalDirectedAppraiser {
    pub fn new(
        dna_reader: Arc<dyn ADNAReader>,
        experience_writer: Arc<dyn ExperienceWriter>,
        event_receiver: broadcast::Receiver<ExperienceEvent>,
    ) -> Self {
        Self {
            dna_reader,
            experience_writer,
            event_receiver,
        }
    }

    pub async fn run(mut self) {
        loop {
            match self.event_receiver.recv().await {
                Ok(event) => {
                    if let Err(e) = self.process_event(event).await {
                        eprintln!("[GoalDirectedAppraiser] Error processing event: {}", e);
                    }
                }
                Err(broadcast::error::RecvError::Closed) => {
                    println!("[GoalDirectedAppraiser] Channel closed, shutting down");
                    break;
                }
                Err(broadcast::error::RecvError::Lagged(skipped)) => {
                    eprintln!("[GoalDirectedAppraiser] Lagged by {} events", skipped);
                    continue;
                }
            }
        }
    }

    async fn process_event(&self, event: ExperienceEvent) -> Result<(), ADNAError> {
        let params = self.dna_reader.get_goal_directed_params().await?;
        let reward = self.calculate_reward(&event, &params);

        if reward.abs() > 1e-6 {
            let _ = self.experience_writer
                .set_appraiser_reward(event.sequence_number as u64, AppraiserType::Goal, reward);
        }

        Ok(())
    }

    fn calculate_reward(&self, event: &ExperienceEvent, params: &GoalDirectedParams) -> f32 {
        // MVP: Simplified immediate rewards
        // Full retroactive trajectory-based rewards deferred for future implementation

        // For MVP, use L7 Valence as a proxy for goal achievement
        // Positive valence indicates progress toward goals
        let valence = event.l7_valence();

        if valence > 0.5 {
            // High positive valence → likely goal achievement
            params.weight * valence
        } else if valence > 0.0 {
            // Moderate positive valence → goal progress
            params.weight * valence * 0.5
        } else {
            // Negative or zero valence → no goal reward
            0.0
        }
    }
}

// ============================================================================
// AppraiserSet - Coordinator for all appraisers
// ============================================================================

/// Coordinator for all 4 appraisers
///
/// Manages the lifecycle of HomeostasisAppraiser, CuriosityAppraiser,
/// EfficiencyAppraiser, and GoalDirectedAppraiser, running them concurrently.
pub struct AppraiserSet {
    homeostasis_handle: Option<JoinHandle<()>>,
    curiosity_handle: Option<JoinHandle<()>>,
    efficiency_handle: Option<JoinHandle<()>>,
    goal_handle: Option<JoinHandle<()>>,
}

impl AppraiserSet {
    /// Start all 4 appraisers in parallel
    ///
    /// # Arguments
    ///
    /// * `dna_reader` - Shared ADNA configuration reader
    /// * `experience_writer` - Shared ExperienceStream writer
    /// * `homeostasis_rx` - Event receiver for HomeostasisAppraiser
    /// * `curiosity_rx` - Event receiver for CuriosityAppraiser
    /// * `efficiency_rx` - Event receiver for EfficiencyAppraiser
    /// * `goal_rx` - Event receiver for GoalDirectedAppraiser
    pub fn start(
        dna_reader: Arc<dyn ADNAReader>,
        experience_writer: Arc<dyn ExperienceWriter>,
        homeostasis_rx: broadcast::Receiver<ExperienceEvent>,
        curiosity_rx: broadcast::Receiver<ExperienceEvent>,
        efficiency_rx: broadcast::Receiver<ExperienceEvent>,
        goal_rx: broadcast::Receiver<ExperienceEvent>,
    ) -> Self {
        // Launch HomeostasisAppraiser
        let homeostasis_appraiser = HomeostasisAppraiser::new(
            dna_reader.clone(),
            experience_writer.clone(),
            homeostasis_rx,
        );
        let homeostasis_handle = tokio::spawn(async move {
            homeostasis_appraiser.run().await;
        });

        // Launch CuriosityAppraiser
        let curiosity_appraiser = CuriosityAppraiser::new(
            dna_reader.clone(),
            experience_writer.clone(),
            curiosity_rx,
        );
        let curiosity_handle = tokio::spawn(async move {
            curiosity_appraiser.run().await;
        });

        // Launch EfficiencyAppraiser
        let efficiency_appraiser = EfficiencyAppraiser::new(
            dna_reader.clone(),
            experience_writer.clone(),
            efficiency_rx,
        );
        let efficiency_handle = tokio::spawn(async move {
            efficiency_appraiser.run().await;
        });

        // Launch GoalDirectedAppraiser
        let goal_appraiser = GoalDirectedAppraiser::new(
            dna_reader.clone(),
            experience_writer.clone(),
            goal_rx,
        );
        let goal_handle = tokio::spawn(async move {
            goal_appraiser.run().await;
        });

        Self {
            homeostasis_handle: Some(homeostasis_handle),
            curiosity_handle: Some(curiosity_handle),
            efficiency_handle: Some(efficiency_handle),
            goal_handle: Some(goal_handle),
        }
    }

    /// Wait for all appraisers to complete
    ///
    /// This will block until all 4 appraisers finish (usually when the
    /// ExperienceStream channel is closed).
    pub async fn wait_all(mut self) {
        if let Some(handle) = self.homeostasis_handle.take() {
            let _ = handle.await;
        }
        if let Some(handle) = self.curiosity_handle.take() {
            let _ = handle.await;
        }
        if let Some(handle) = self.efficiency_handle.take() {
            let _ = handle.await;
        }
        if let Some(handle) = self.goal_handle.take() {
            let _ = handle.await;
        }
        println!("[AppraiserSet] All appraisers completed");
    }

    /// Graceful shutdown - abort all appraiser tasks
    pub fn shutdown(mut self) {
        println!("[AppraiserSet] Shutting down all appraisers...");
        if let Some(handle) = self.homeostasis_handle.take() {
            handle.abort();
        }
        if let Some(handle) = self.curiosity_handle.take() {
            handle.abort();
        }
        if let Some(handle) = self.efficiency_handle.take() {
            handle.abort();
        }
        if let Some(handle) = self.goal_handle.take() {
            handle.abort();
        }
        println!("[AppraiserSet] All appraisers shut down");
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adna::{InMemoryADNAReader, AppraiserConfig};
    use crate::experience_stream::ExperienceStream;

    #[test]
    fn test_homeostasis_reward_calculation() {
        let params = HomeostasisParams::default();
        let appraiser = create_test_homeostasis_appraiser();

        // Event within all ranges - no penalty
        let mut event = ExperienceEvent::default();
        event.state[4] = 0.5; // L5 Cognitive Load in range [0.2, 0.7]
        event.state[5] = 0.6; // L6 Certainty in range [0.4, 0.9]
        event.state[7] = 0.8; // L8 Coherence in range [0.5, 1.0]

        let reward = appraiser.calculate_reward(&event, &params);
        assert_eq!(reward, 0.0);

        // Event with cognitive load too high
        event.state[4] = 0.9; // Above max 0.7
        let reward = appraiser.calculate_reward(&event, &params);
        assert!(reward < 0.0); // Should be penalized
    }

    #[test]
    fn test_curiosity_reward_calculation() {
        let params = CuriosityParams::default();
        let appraiser = create_test_curiosity_appraiser();

        // Low novelty - no reward
        let mut event = ExperienceEvent::default();
        event.state[1] = 0.2; // L2 Novelty below threshold 0.3

        let reward = appraiser.calculate_reward(&event, &params);
        assert_eq!(reward, 0.0);

        // High novelty - should reward
        event.state[1] = 0.8; // L2 Novelty above threshold
        let reward = appraiser.calculate_reward(&event, &params);
        assert!(reward > 0.0);
    }

    #[test]
    fn test_efficiency_reward_calculation() {
        let params = EfficiencyParams::default();
        let appraiser = create_test_efficiency_appraiser();

        // Event with motor activity
        let mut event = ExperienceEvent::default();
        event.state[2] = 0.5; // L3 Velocity
        event.action[2] = 0.3; // L3 Acceleration

        let reward = appraiser.calculate_reward(&event, &params);
        assert!(reward < 0.0); // Should penalize resource usage

        // Event with higher activity should have more penalty
        event.state[2] = 0.9;
        event.action[2] = 0.7;
        let reward2 = appraiser.calculate_reward(&event, &params);
        assert!(reward2 < reward); // More penalty with higher activity
    }

    #[test]
    fn test_goal_directed_reward_calculation() {
        let params = GoalDirectedParams::default();
        let appraiser = create_test_goal_directed_appraiser();

        // High positive valence (goal achievement)
        let mut event = ExperienceEvent::default();
        event.state[6] = 0.8; // L7 Valence

        let reward = appraiser.calculate_reward(&event, &params);
        assert!(reward > 0.0);
        assert!(reward > params.weight * 0.5); // Should be significant

        // Moderate positive valence (goal progress)
        event.state[6] = 0.3;
        let reward2 = appraiser.calculate_reward(&event, &params);
        assert!(reward2 > 0.0);
        assert!(reward2 < reward); // Less than high valence

        // Negative valence (no goal reward)
        event.state[6] = -0.5;
        let reward3 = appraiser.calculate_reward(&event, &params);
        assert_eq!(reward3, 0.0);
    }

    // Helper functions to create test appraisers
    fn create_test_homeostasis_appraiser() -> HomeostasisAppraiser {
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let stream = Arc::new(ExperienceStream::new(100, 10));
        let receiver = stream.subscribe();
        HomeostasisAppraiser::new(dna_reader, stream.clone(), receiver)
    }

    fn create_test_curiosity_appraiser() -> CuriosityAppraiser {
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let stream = Arc::new(ExperienceStream::new(100, 10));
        let receiver = stream.subscribe();
        CuriosityAppraiser::new(dna_reader, stream.clone(), receiver)
    }

    fn create_test_efficiency_appraiser() -> EfficiencyAppraiser {
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let stream = Arc::new(ExperienceStream::new(100, 10));
        let receiver = stream.subscribe();
        EfficiencyAppraiser::new(dna_reader, stream.clone(), receiver)
    }

    fn create_test_goal_directed_appraiser() -> GoalDirectedAppraiser {
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let stream = Arc::new(ExperienceStream::new(100, 10));
        let receiver = stream.subscribe();
        GoalDirectedAppraiser::new(dna_reader, stream.clone(), receiver)
    }
}