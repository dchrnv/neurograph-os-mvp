use neurograph_core::{ADNA, ExperienceToken, ADNAHeader, EvolutionMetrics, PolicyPointer, StateMapping};

fn main() {
    println!("ADNA size: {}", std::mem::size_of::<ADNA>());
    println!("ADNAHeader size: {}", std::mem::size_of::<ADNAHeader>());
    println!("EvolutionMetrics size: {}", std::mem::size_of::<EvolutionMetrics>());
    println!("PolicyPointer size: {}", std::mem::size_of::<PolicyPointer>());
    println!("StateMapping size: {}", std::mem::size_of::<StateMapping>());
    println!("\nExperienceToken size: {}", std::mem::size_of::<ExperienceToken>());
}
