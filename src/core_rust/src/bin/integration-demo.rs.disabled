/// Integration Demo: Token + Connection + Grid in Rust
///
/// Demonstrates how all three core components work together.
///
/// Run with: cargo run --release --bin integration-demo

use neurograph_core::{
    Token, Connection, Grid, GridConfig,
    CoordinateSpace, EntityType, ConnectionType,
    token_flags, connection_flags, active_levels,
};
use std::collections::HashMap;

fn print_section(title: &str) {
    println!("\n{}", "=".repeat(70));
    println!("  {}", title);
    println!("{}\n", "=".repeat(70));
}

fn demo_1_semantic_network() {
    print_section("DEMO 1: Semantic Network with Spatial Indexing");

    // Create grid
    let config = GridConfig {
        bucket_size: 5.0,
        density_threshold: 0.5,
        min_field_nodes: 3,
    };
    let mut grid = Grid::with_config(config);

    // Define concepts
    let concepts = vec![
        ("dog", 1, 0.0, 0.0, 0.0),
        ("cat", 2, 3.0, 2.0, 0.0),
        ("animal", 3, 1.5, 5.0, 0.0),
        ("pet", 4, 1.5, 1.0, 0.0),
        ("mammal", 5, 1.5, 3.5, 0.0),
    ];

    println!("Creating concept tokens:");
    let mut token_map = HashMap::new();

    for (name, id, x, y, z) in &concepts {
        let mut token = Token::new(*id);
        token.set_coordinates(CoordinateSpace::L8Abstract, *x, *y, *z);
        token.set_entity_type(EntityType::Concept);
        token.weight = 100; // 1.0 * 100
        token.set_flag(token_flags::ACTIVE);

        println!("  [{}] {:8} at ({:4.1}, {:4.1}, {:4.1})",
                 id, name, x, y, z);

        grid.add(token).unwrap();
        token_map.insert(*id, *name);
    }

    println!("\nGrid contains {} tokens", grid.len());

    // Create semantic connections
    println!("\nCreating semantic relationships:");
    let mut connections = Vec::new();

    let hypernym_pairs = vec![
        (1, 3, 230), // dog -> animal (0.90)
        (2, 3, 230), // cat -> animal
        (1, 5, 242), // dog -> mammal (0.95)
        (2, 5, 242), // cat -> mammal
        (1, 4, 217), // dog -> pet (0.85)
        (2, 4, 217), // cat -> pet
    ];

    for (source_id, target_id, strength) in hypernym_pairs {
        let mut conn = Connection::new(source_id, target_id);
        conn.set_connection_type(ConnectionType::Hypernym);
        conn.pull_strength = strength;
        conn.rigidity = 200; // 0.80
        conn.set_flag(connection_flags::ACTIVE);

        let source_name = token_map.get(&source_id).unwrap_or(&"?");
        let target_name = token_map.get(&target_id).unwrap_or(&"?");

        println!("  {:8} --[Hypernym]-> {:8} (strength: {:.2})",
                 source_name, target_name, strength as f32 / 255.0);

        connections.push(conn);
    }

    // Similar connection
    let mut conn = Connection::new(1, 2); // dog <-> cat
    conn.set_connection_type(ConnectionType::Similar);
    conn.pull_strength = 178; // 0.70
    conn.rigidity = 153; // 0.60
    conn.set_flag(connection_flags::ACTIVE);
    conn.set_flag(connection_flags::BIDIRECTIONAL);

    println!("  dog      <-[Similar]-> cat      (strength: 0.70)");
    connections.push(conn);

    println!("\nCreated {} semantic connections", connections.len());

    // Spatial queries
    println!("\nSpatial Queries in Abstract Space (L8):");

    // Find neighbors of "dog" (id=1)
    let neighbors = grid.find_neighbors(1, CoordinateSpace::L8Abstract, 5.0, 10);

    println!("\n  Neighbors of 'dog' within radius 5.0:");
    for (token_id, distance) in neighbors {
        if token_id != 1 {
            let name = token_map.get(&token_id).unwrap_or(&"?");
            println!("    - {:8}: distance = {:.2}", name, distance);
        }
    }

    // Range query around "animal"
    let animal_token = grid.get(3).unwrap();
    let coords = animal_token.get_coordinates(CoordinateSpace::L8Abstract);
    let nearby = grid.range_query(CoordinateSpace::L8Abstract, coords.0, coords.1, coords.2, 3.0);

    println!("\n  Concepts within 3.0 units of 'animal':");
    for (token_id, distance) in nearby {
        let name = token_map.get(&token_id).unwrap_or(&"?");
        println!("    - {:8}: distance = {:.2}", name, distance);
    }

    println!("\n✓ Demo 1 complete");
}

fn demo_2_emotional_landscape() {
    print_section("DEMO 2: Emotional Landscape with Field Physics");

    let mut grid = Grid::new();

    // Create emotional tokens (VAD model)
    let emotions = vec![
        ("joy", 1, 0.8, 0.7, 0.6, 1.5, 0.8),
        ("sadness", 2, -0.7, 0.3, 0.3, 1.2, 0.6),
        ("anger", 3, -0.5, 0.8, 0.7, 1.0, 0.7),
        ("fear", 4, -0.6, 0.7, 0.2, 1.3, 0.5),
        ("calm", 5, 0.3, 0.2, 0.6, 2.0, 0.4),
    ];

    println!("Creating emotional tokens with fields:");
    let mut emotion_map = HashMap::new();

    for (name, id, v, a, d, field_r, field_s) in &emotions {
        let mut token = Token::new(*id);
        token.set_coordinates(CoordinateSpace::L4Emotional, *v, *a, *d);
        token.set_entity_type(EntityType::Event);
        token.field_radius = (field_r * 100.0) as u8;
        token.field_strength = (field_s * 255.0) as u8;
        token.set_flag(token_flags::ACTIVE);

        println!("  [{}] {:8} VAD=({:5.2}, {:5.2}, {:5.2}) field_radius={:.2} field_strength={:.2}",
                 id, name, v, a, d, field_r, field_s);

        grid.add(token).unwrap();
        emotion_map.insert(*id, *name);
    }

    // Field influence analysis
    println!("\nField Influence Analysis:");

    let test_points = vec![
        ("Near joy", 0.8, 0.7, 0.6),
        ("Near sadness", -0.7, 0.3, 0.3),
        ("Center", 0.0, 0.0, 0.0),
        ("High arousal", 0.0, 0.8, 0.5),
    ];

    for (name, x, y, z) in test_points {
        let influence = grid.calculate_field_influence(
            CoordinateSpace::L4Emotional,
            x, y, z,
            3.0
        );

        let density = grid.calculate_density(
            CoordinateSpace::L4Emotional,
            x, y, z,
            1.0
        );

        println!("  {:15} ({:5.2}, {:5.2}, {:5.2}): influence={:.3}, density={:.4}",
                 name, x, y, z, influence, density);
    }

    // Find emotionally similar states
    println!("\nEmotionally Similar States (neighbors of 'joy'):");
    let joy_neighbors = grid.find_neighbors(1, CoordinateSpace::L4Emotional, 2.0, 5);

    for (token_id, distance) in joy_neighbors {
        if token_id != 1 {
            let name = emotion_map.get(&token_id).unwrap_or(&"?");
            println!("    - {:8}: emotional distance = {:.3}", name, distance);
        }
    }

    println!("\n✓ Demo 2 complete");
}

fn demo_3_physical_network() {
    print_section("DEMO 3: Physical Network with Force Model");

    let config = GridConfig {
        bucket_size: 10.0,
        density_threshold: 0.5,
        min_field_nodes: 3,
    };
    let mut grid = Grid::with_config(config);

    // Create tokens in physical space
    println!("Creating physical nodes:");
    let positions = vec![
        (1, 0.0, 0.0, 0.0),
        (2, 5.0, 0.0, 0.0),
        (3, 10.0, 0.0, 0.0),
        (4, 2.5, 4.3, 0.0),
        (5, 7.5, 4.3, 0.0),
    ];

    for (id, x, y, z) in &positions {
        let mut token = Token::new(*id);
        token.set_coordinates(CoordinateSpace::L1Physical, *x, *y, *z);
        token.set_entity_type(EntityType::Object);
        token.weight = 100; // 1.0
        token.set_flag(token_flags::ACTIVE);

        println!("  Node {:2}: ({:6.2}, {:6.2}, {:6.2})", id, x, y, z);

        grid.add(token).unwrap();
    }

    // Create proximity-based connections
    println!("\nCreating proximity connections:");
    let mut connections = Vec::new();

    for (id, _, _, _) in &positions {
        let neighbors = grid.find_neighbors(*id, CoordinateSpace::L1Physical, 7.0, 3);

        for (neighbor_id, distance) in neighbors {
            if neighbor_id != *id && *id < neighbor_id {
                let mut conn = Connection::new(*id, neighbor_id);
                conn.set_connection_type(ConnectionType::Proximity);
                conn.preferred_distance = (distance * 100.0) as u16;
                conn.pull_strength = 127; // 0.5
                conn.rigidity = 178; // 0.7
                conn.set_flag(connection_flags::ACTIVE);

                println!("  Node {} <-> Node {} (distance={:.2}, preferred={:.2})",
                         id, neighbor_id, distance, distance);

                connections.push(conn);
            }
        }
    }

    println!("\nCreated {} proximity connections", connections.len());

    // Force analysis
    println!("\nForce Analysis:");

    if let Some(conn) = connections.first() {
        let preferred = conn.preferred_distance as f32 / 100.0;
        println!("\n  Sample connection (preferred distance: {:.2}m):", preferred);

        for d in &[3.0, 5.0, 7.0, 10.0] {
            let force = conn.calculate_force(*d);
            let force_f = force as f32 / 255.0;

            let direction = if force_f > 0.0 {
                "pull"
            } else if force_f < 0.0 {
                "push"
            } else {
                "equilibrium"
            };

            println!("    At {:5.2}m: force = {:+.3} ({})", d, force_f, direction);
        }
    }

    // Density analysis
    println!("\nDensity Analysis:");

    // Center of mass
    let center_x = positions.iter().map(|(_, x, _, _)| x).sum::<f32>() / positions.len() as f32;
    let center_y = positions.iter().map(|(_, _, y, _)| y).sum::<f32>() / positions.len() as f32;
    let center_z = positions.iter().map(|(_, _, _, z)| z).sum::<f32>() / positions.len() as f32;

    println!("  Center of mass: ({:.2}, {:.2}, {:.2})", center_x, center_y, center_z);

    for radius in &[5.0, 10.0, 15.0] {
        let density = grid.calculate_density(
            CoordinateSpace::L1Physical,
            center_x, center_y, center_z,
            *radius
        );
        println!("    Density within {:5.2}m radius: {:.6} nodes/m³", radius, density);
    }

    println!("\n✓ Demo 3 complete");
}

fn demo_4_multi_level_activation() {
    print_section("DEMO 4: Multi-Level Connection Activation");

    let mut grid = Grid::new();

    // Create tokens in different spaces
    let mut token1 = Token::new(1);
    token1.set_coordinates(CoordinateSpace::L1Physical, 0.0, 0.0, 0.0);
    token1.set_coordinates(CoordinateSpace::L5Cognitive, 0.5, 0.7, 0.3);
    token1.set_entity_type(EntityType::Agent);
    token1.set_flag(token_flags::ACTIVE);

    let mut token2 = Token::new(2);
    token2.set_coordinates(CoordinateSpace::L1Physical, 5.0, 0.0, 0.0);
    token2.set_coordinates(CoordinateSpace::L5Cognitive, 0.6, 0.8, 0.4);
    token2.set_entity_type(EntityType::Agent);
    token2.set_flag(token_flags::ACTIVE);

    grid.add(token1).unwrap();
    grid.add(token2).unwrap();

    println!("Created 2 multi-dimensional tokens");

    // Create connection with selective level activation
    let mut conn = Connection::new(1, 2);
    conn.set_connection_type(ConnectionType::Related);
    conn.pull_strength = 178; // 0.7
    conn.rigidity = 204; // 0.8

    // Activate only on specific levels
    conn.active_levels = active_levels::PHYSICAL | active_levels::COGNITIVE;
    conn.set_flag(connection_flags::ACTIVE);

    println!("\nConnection Configuration:");
    println!("  Type: Related");
    println!("  Active on levels: L1 Physical, L5 Cognitive");

    // Check which levels are active
    println!("\nLevel Activation Status:");
    let levels = vec![
        ("L1 Physical", active_levels::PHYSICAL),
        ("L2 Sensory", active_levels::SENSORY),
        ("L3 Motor", active_levels::MOTOR),
        ("L4 Emotional", active_levels::EMOTIONAL),
        ("L5 Cognitive", active_levels::COGNITIVE),
        ("L6 Social", active_levels::SOCIAL),
        ("L7 Temporal", active_levels::TEMPORAL),
        ("L8 Abstract", active_levels::ABSTRACT),
    ];

    for (name, level) in levels {
        let is_active = conn.is_level_active(level);
        let status = if is_active { "✓ ACTIVE" } else { "  inactive" };
        println!("  {:12}: {}", name, status);
    }

    // Activate connection
    for _ in 0..5 {
        conn.activate();
    }

    println!("\nConnection Stats:");
    println!("  Activation count: {}", conn.activation_count);
    println!("  Active: {}", conn.is_active());

    // Distance in different spaces
    println!("\nDistances in Active Spaces:");

    let t1 = grid.get(1).unwrap();
    let t2 = grid.get(2).unwrap();

    let d_physical = t1.distance_to(t2, CoordinateSpace::L1Physical);
    let d_cognitive = t1.distance_to(t2, CoordinateSpace::L5Cognitive);

    println!("  L1 Physical:  {:.3}m", d_physical);
    println!("  L5 Cognitive: {:.3}", d_cognitive);

    println!("\n✓ Demo 4 complete");
}

fn main() {
    println!("\n{}", "=".repeat(70));
    println!("  NeuroGraph OS - Integration Demo (Rust)");
    println!("  Token + Connection + Grid Working Together");
    println!("{}", "=".repeat(70));

    demo_1_semantic_network();
    demo_2_emotional_landscape();
    demo_3_physical_network();
    demo_4_multi_level_activation();

    // Summary
    print_section("Summary: All Integration Demos Complete");
    println!("Demonstrated capabilities:");
    println!("  ✓ Token creation with multi-dimensional coordinates");
    println!("  ✓ Connection creation with semantic types and force models");
    println!("  ✓ Grid spatial indexing for fast neighbor queries");
    println!("  ✓ Field influence and density calculations");
    println!("  ✓ Selective level activation");
    println!("  ✓ Integration of all three core components");
    println!("\nNeuroGraph OS v0.15.0 - Production Ready");
    println!("{}\n", "=".repeat(70));
}
