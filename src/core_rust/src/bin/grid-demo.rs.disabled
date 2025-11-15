//! Grid V2.0 Demo
//!
//! Demonstrates the Grid spatial indexing and field calculations.
//!
//! Usage:
//!   cargo run --bin grid-demo --release

use neurograph_core::{Token, Grid, GridConfig, CoordinateSpace, EntityType};

fn main() {
    println!("NeuroGraph Grid V2.0 - Rust Demo");
    println!("=================================\n");

    demo_basic_grid();
    demo_neighbor_search();
    demo_range_query();
    demo_field_influence();
    demo_density();
    demo_performance();

    println!("\nAll demonstrations completed successfully!");
}

fn demo_basic_grid() {
    println!("=== Basic Grid Operations ===");

    // Create grid with default config
    let mut grid = Grid::new();
    println!("Created empty grid");

    // Add tokens
    let mut token1 = Token::new(1);
    token1.set_coordinates(CoordinateSpace::L1Physical, 0.00, 0.00, 0.00);
    grid.add(token1).unwrap();

    let mut token2 = Token::new(2);
    token2.set_coordinates(CoordinateSpace::L1Physical, 10.00, 0.00, 0.00);
    grid.add(token2).unwrap();

    println!("Added 2 tokens, grid size: {}", grid.len());

    // Retrieve token
    if let Some(token) = grid.get(1) {
        let (x, y, z) = token.get_coordinates(CoordinateSpace::L1Physical);
        println!("Retrieved token 1 at ({:.2}, {:.2}, {:.2})", x, y, z);
    }

    // Remove token
    if let Some(_) = grid.remove(2) {
        println!("Removed token 2, grid size: {}", grid.len());
    }

    println!();
}

fn demo_neighbor_search() {
    println!("=== Neighbor Search ===");

    let mut grid = Grid::new();

    // Add tokens in a line
    for i in 0..10 {
        let mut token = Token::new(i);
        token.set_coordinates(CoordinateSpace::L1Physical, i as f32, 0.00, 0.00);
        grid.add(token).unwrap();
    }

    println!("Added 10 tokens in a line (0-9 on X axis)");

    // Find neighbors of token 5 within radius 3
    let neighbors = grid.find_neighbors(5, CoordinateSpace::L1Physical, 3.00, 10);

    println!("Neighbors of token 5 within radius 3.0:");
    for (id, distance) in neighbors {
        println!("  Token {}: distance = {:.2}", id, distance);
    }

    println!();
}

fn demo_range_query() {
    println!("=== Range Query ===");

    let mut grid = Grid::new();

    // Add tokens in a grid pattern
    let mut id = 0;
    for x in 0..5 {
        for y in 0..5 {
            let mut token = Token::new(id);
            token.set_coordinates(
                CoordinateSpace::L1Physical,
                x as f32 * 10.0,
                y as f32 * 10.0,
                0.00
            );
            grid.add(token).unwrap();
            id += 1;
        }
    }

    println!("Created 5x5 grid of tokens (spacing 10.0)");

    // Query around center point (20, 20, 0) with radius 15
    let results = grid.range_query(CoordinateSpace::L1Physical, 20.00, 20.00, 0.00, 15.00);

    println!("Tokens within radius 15.0 of (20, 20, 0):");
    println!("  Found {} tokens:", results.len());
    for (id, distance) in results.iter().take(5) {
        println!("    Token {}: distance = {:.2}", id, distance);
    }

    println!();
}

fn demo_field_influence() {
    println!("=== Field Influence ===");

    let mut grid = Grid::new();

    // Add token with strong field
    let mut token1 = Token::new(1);
    token1.set_coordinates(CoordinateSpace::L1Physical, 0.00, 0.00, 0.00);
    token1.field_radius = 200; // 2.0 decoded
    token1.field_strength = 255; // 1.0 decoded
    grid.add(token1).unwrap();

    // Add token with weak field
    let mut token2 = Token::new(2);
    token2.set_coordinates(CoordinateSpace::L1Physical, 5.00, 0.00, 0.00);
    token2.field_radius = 100; // 1.0 decoded
    token2.field_strength = 128; // 0.5 decoded
    grid.add(token2).unwrap();

    println!("Added 2 tokens with field properties");

    // Calculate influence at different points
    let points = vec![
        (0.00, 0.00, 0.00, "Origin (on strong field)"),
        (1.00, 0.00, 0.00, "1m from origin"),
        (5.00, 0.00, 0.00, "On weak field"),
        (10.00, 0.00, 0.00, "Far from both"),
    ];

    for (x, y, z, desc) in points {
        let influence = grid.calculate_field_influence(
            CoordinateSpace::L1Physical,
            x, y, z,
            10.00
        );
        println!("  {}: influence = {:.3}", desc, influence);
    }

    println!();
}

fn demo_density() {
    println!("=== Density Calculation ===");

    let mut grid = Grid::new();

    // Create dense cluster at origin
    for i in 0..20 {
        let mut token = Token::new(i);
        let x = (i as f32 % 5.0) * 0.5;
        let y = (i as f32 / 5.0).floor() * 0.5;
        token.set_coordinates(CoordinateSpace::L1Physical, x, y, 0.00);
        grid.add(token).unwrap();
    }

    // Create sparse area far away
    for i in 20..23 {
        let mut token = Token::new(i);
        let x = 50.00 + ((i - 20) as f32 * 10.0);
        token.set_coordinates(CoordinateSpace::L1Physical, x, 0.00, 0.00);
        grid.add(token).unwrap();
    }

    println!("Created clusters: 20 tokens in dense cluster, 3 in sparse area");

    // Calculate density in different areas
    let areas = vec![
        (1.00, 1.00, 0.00, 2.00, "Dense cluster center"),
        (50.00, 0.00, 0.00, 5.00, "Sparse area"),
        (25.00, 0.00, 0.00, 5.00, "Empty space"),
    ];

    for (x, y, z, radius, desc) in areas {
        let density = grid.calculate_density(
            CoordinateSpace::L1Physical,
            x, y, z,
            radius
        );
        println!("  {}: density = {:.6} tokens/unit³", desc, density);
    }

    println!();
}

fn demo_performance() {
    println!("=== Performance Test ===");

    let mut grid = Grid::new();

    // Add 1000 tokens
    use std::time::Instant;

    let start = Instant::now();
    for i in 0..1000 {
        let mut token = Token::new(i);
        let x = ((i as f32 * 1.234567).sin() * 100.0);
        let y = ((i as f32 * 2.345678).cos() * 100.0);
        let z = ((i as f32 * 3.456789).sin() * 100.0);
        token.set_coordinates(CoordinateSpace::L1Physical, x, y, z);
        grid.add(token).unwrap();
    }
    let duration = start.elapsed();

    println!("Added 1000 tokens in {:?}", duration);
    println!("  ~{:.0} tokens/sec", 1000.0 / duration.as_secs_f64());

    // Perform 1000 neighbor searches
    let start = Instant::now();
    for i in 0..1000 {
        let _ = grid.find_neighbors(i % 1000, CoordinateSpace::L1Physical, 20.00, 10);
    }
    let duration = start.elapsed();

    println!("Performed 1000 neighbor searches in {:?}", duration);
    println!("  ~{:.0} searches/sec", 1000.0 / duration.as_secs_f64());

    println!();
}
