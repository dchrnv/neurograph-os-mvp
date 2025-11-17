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

/// Token V2.0 Demo Application
///
/// Demonstrates the core functionality of NeuroGraph OS Token V2.0

use neurograph_core::{Token, CoordinateSpace, EntityType};
use neurograph_core::token::flags;

fn main() {
    println!("=== NeuroGraph OS Token V2.0 Demo ===\n");

    // Create a new token
    println!("1. Creating new token...");
    let mut token = Token::new(Token::create_id(12345, 2, 1));

    println!("   Token ID: {}", token.id);
    println!("   Local ID: {}", token.local_id());
    println!("   Domain: {}", token.domain());
    println!("   Size: {} bytes\n", std::mem::size_of::<Token>());

    // Set coordinates in different spaces
    println!("2. Setting coordinates in semantic spaces...");
    println!("   (Note: Using precision x.xx for proper encoding)\n");

    // L1: Physical space (meters, scale 100 → precision 0.01)
    token.set_coordinates(CoordinateSpace::L1Physical, 10.50, 20.30, 5.20);
    println!("   L1 Physical: {:?}", token.get_coordinates(CoordinateSpace::L1Physical));

    // L4: Emotional space (VAD model, scale 10000 → precision 0.0001)
    token.set_coordinates(CoordinateSpace::L4Emotional, 0.80, 0.60, 0.50);
    println!("   L4 Emotional (VAD): {:?}", token.get_coordinates(CoordinateSpace::L4Emotional));

    // L8: Abstract space (scale 10000 → precision 0.0001)
    token.set_coordinates(CoordinateSpace::L8Abstract, 0.90, 0.30, 0.70);
    println!("   L8 Abstract: {:?}\n", token.get_coordinates(CoordinateSpace::L8Abstract));

    // Set entity type and flags
    println!("3. Setting entity type and flags...");
    token.set_entity_type(EntityType::Concept);
    token.set_flag(flags::PERSISTENT);
    token.set_flag(flags::MUTABLE);

    println!("   Entity type: {:?}", token.get_entity_type());
    println!("   Active: {}", token.has_flag(flags::ACTIVE));
    println!("   Persistent: {}", token.has_flag(flags::PERSISTENT));
    println!("   Mutable: {}\n", token.has_flag(flags::MUTABLE));

    // Set weight and field properties
    println!("4. Setting weight and field properties...");
    token.weight = 0.75;
    token.set_field_radius(1.5);
    token.set_field_strength(0.85);

    println!("   Weight: {}", token.weight);
    println!("   Field radius: {}", token.get_field_radius());
    println!("   Field strength: {}\n", token.get_field_strength());

    // Validate token
    println!("5. Validating token...");
    match token.validate() {
        Ok(_) => println!("   ✓ Token is valid\n"),
        Err(e) => println!("   ✗ Token validation failed: {}\n", e),
    }

    // Serialize and deserialize
    println!("6. Testing serialization...");
    let bytes = token.to_bytes();
    println!("   Serialized to {} bytes", bytes.len());

    let token_copy = Token::from_bytes(&bytes);
    println!("   Deserialized token ID: {}", token_copy.id);
    println!("   Weight preserved: {}", token_copy.weight);

    let coords_copy = token_copy.get_coordinates(CoordinateSpace::L1Physical);
    println!("   Coordinates preserved: {:?}\n", coords_copy);

    // Debug output
    println!("7. Debug representation:");
    println!("{:?}\n", token);

    // Demonstrate multiple tokens
    println!("8. Creating multiple tokens with different types...");
    let types = [
        (EntityType::Object, "Physical object"),
        (EntityType::Event, "Temporal event"),
        (EntityType::Process, "Running process"),
        (EntityType::Memory, "Stored memory"),
    ];

    for (i, (entity_type, desc)) in types.iter().enumerate() {
        let mut t = Token::new(Token::create_id(1000 + i as u32, 0, 0));
        t.set_entity_type(*entity_type);
        println!("   Token {}: {:?} - {}", i + 1, entity_type, desc);
    }

    println!("\n=== Demo Complete ===");
}
