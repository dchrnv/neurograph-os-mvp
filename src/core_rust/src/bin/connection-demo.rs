/// Connection V1.0 Demo Application
///
/// Demonstrates the core functionality of NeuroGraph OS Connection V1.0

use neurograph_core::{Connection, ConnectionType, active_levels, connection_flags};

fn main() {
    println!("=== NeuroGraph OS Connection V1.0 Demo ===\n");

    // Create a connection between two tokens
    println!("1. Creating connection between tokens...");
    let mut conn = Connection::new(1001, 1002);

    println!("   Connection: {} → {}", conn.token_a_id, conn.token_b_id);
    println!("   Size: {} bytes", std::mem::size_of::<Connection>());
    println!("   Type: {:?}", conn.get_connection_type());
    println!("   Rigidity: {:.2}\n", conn.get_rigidity());

    // Set connection type
    println!("2. Setting connection type...");
    conn.set_connection_type(ConnectionType::Cause);
    println!("   Type: {:?} (Causal relationship)", conn.get_connection_type());
    println!("   Token 1001 CAUSES Token 1002\n");

    // Configure rigidity and strength
    println!("3. Configuring physical parameters...");
    conn.set_rigidity(0.85);
    conn.pull_strength = 0.70;
    conn.preferred_distance = 1.50;

    println!("   Rigidity: {:.2} (how firm the connection is)", conn.get_rigidity());
    println!("   Pull strength: {:.2} (attraction force)", conn.pull_strength);
    println!("   Preferred distance: {:.2} (desired spacing)\n", conn.preferred_distance);

    // Set active levels
    println!("4. Setting active semantic spaces...");
    conn.active_levels = active_levels::NONE;
    conn.activate_level(active_levels::L5_COGNITIVE);
    conn.activate_level(active_levels::L8_ABSTRACT);

    println!("   Active on L5 (Cognitive): {}", conn.is_level_active(active_levels::L5_COGNITIVE));
    println!("   Active on L8 (Abstract): {}", conn.is_level_active(active_levels::L8_ABSTRACT));
    println!("   Active on L1 (Physical): {}\n", conn.is_level_active(active_levels::L1_PHYSICAL));

    // Set flags
    println!("5. Setting connection flags...");
    conn.set_flag(connection_flags::PERSISTENT);
    conn.set_flag(connection_flags::REINFORCED);

    println!("   Active: {}", conn.has_flag(connection_flags::ACTIVE));
    println!("   Bidirectional: {}", conn.has_flag(connection_flags::BIDIRECTIONAL));
    println!("   Persistent: {}", conn.has_flag(connection_flags::PERSISTENT));
    println!("   Reinforced: {}\n", conn.has_flag(connection_flags::REINFORCED));

    // Activate the connection
    println!("6. Activating connection multiple times...");
    println!("   Initial activation count: {}", conn.activation_count);

    conn.activate();
    conn.activate();
    conn.activate();

    println!("   After 3 activations: {}", conn.activation_count);
    println!("   Last activation: {} seconds ago\n", conn.time_since_activation());

    // Validate
    println!("7. Validating connection...");
    match conn.validate() {
        Ok(_) => println!("   ✓ Connection is valid\n"),
        Err(e) => println!("   ✗ Connection validation failed: {}\n", e),
    }

    // Serialize and deserialize
    println!("8. Testing serialization...");
    let bytes = conn.to_bytes();
    println!("   Serialized to {} bytes", bytes.len());

    let conn_copy = Connection::from_bytes(&bytes);
    println!("   Deserialized connection:");
    println!("   - Tokens: {} → {}", conn_copy.token_a_id, conn_copy.token_b_id);
    println!("   - Type: {:?}", conn_copy.get_connection_type());
    println!("   - Activation count: {}\n", conn_copy.activation_count);

    // Debug representation
    println!("9. Debug representation:");
    println!("{:#?}\n", conn);

    // Demonstrate different connection types
    println!("10. Creating connections with different types...");

    let examples = [
        (ConnectionType::Synonym, 1, 2, "Words with same meaning"),
        (ConnectionType::Cause, 3, 4, "Causal relationship"),
        (ConnectionType::Before, 5, 6, "Temporal sequence"),
        (ConnectionType::Near, 7, 8, "Spatial proximity"),
        (ConnectionType::Implies, 9, 10, "Logical implication"),
    ];

    for (i, (conn_type, a, b, desc)) in examples.iter().enumerate() {
        let mut c = Connection::new(*a, *b);
        c.set_connection_type(*conn_type);
        println!("   {}: {:?} - {}", i + 1, conn_type, desc);
        println!("      {} → {} (type: {})", c.token_a_id, c.token_b_id, c.connection_type);
    }

    // Demonstrate attraction vs repulsion
    println!("\n11. Attraction vs Repulsion...");

    let mut attraction = Connection::new(100, 101);
    attraction.set_connection_type(ConnectionType::Synonym);
    attraction.pull_strength = 0.80;  // Positive = attraction
    attraction.preferred_distance = 0.10;
    println!("   Synonym (attraction):");
    println!("   - Pull strength: {:.2} (positive = attract)", attraction.pull_strength);
    println!("   - Preferred distance: {:.2} (very close)", attraction.preferred_distance);

    let mut repulsion = Connection::new(200, 201);
    repulsion.set_connection_type(ConnectionType::Antonym);
    repulsion.pull_strength = -0.60;  // Negative = repulsion
    repulsion.preferred_distance = 3.00;
    println!("\n   Antonym (repulsion):");
    println!("   - Pull strength: {:.2} (negative = repel)", repulsion.pull_strength);
    println!("   - Preferred distance: {:.2} (far apart)", repulsion.preferred_distance);

    // Connection aging
    println!("\n12. Connection lifecycle...");
    let test_conn = Connection::new(500, 501);
    println!("   Age: {} seconds", test_conn.age());
    println!("   Time since last activation: {} seconds", test_conn.time_since_activation());
    println!("   Created at: {}", test_conn.created_at);
    println!("   Last activation: {}", test_conn.last_activation);

    println!("\n=== Demo Complete ===");
}
