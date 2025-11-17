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

/// Archive module - long-term storage and compression
///
/// This module contains structures and utilities for archiving
/// experiences in a compressed format for long-term storage
/// and later replay/analysis.

pub mod experience_token;

pub use experience_token::{
    ExperienceToken,
    ExperienceFlags,
    InfoFlags,
    EXPERIENCE_TOKEN_MAGIC,
};
