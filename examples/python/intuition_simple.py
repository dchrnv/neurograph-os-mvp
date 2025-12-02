#!/usr/bin/env python3
"""
NeuroGraph OS - IntuitionEngine Simple Demo

Shows the simplest possible usage of IntuitionEngine v3.0
(Hybrid Reflex System).

Features:
- System 1 (Fast Path): ~30-50ns reflex lookup
- System 2 (Slow Path): Pattern analysis from experience
"""

import neurograph


def main():
    print("""
╔══════════════════════════════════════════════════════════════╗
║    NeuroGraph OS - IntuitionEngine Simple Demo              ║
╚══════════════════════════════════════════════════════════════╝

IntuitionEngine v3.0: Hybrid Reflex System
- Fast Path (System 1): ~30-50ns reflexive responses
- Slow Path (System 2): Experience-based pattern analysis
""")

    # =============================
    # ONE-LINE INITIALIZATION
    # =============================
    print("=" * 60)
    print("CREATING INTUITION ENGINE")
    print("=" * 60)
    print()
    print("Code:")
    print("  intuition = neurograph.IntuitionEngine.with_defaults()")
    print()

    intuition = neurograph.IntuitionEngine.with_defaults()
    print("✓ Created IntuitionEngine with default configuration")
    print()

    # =============================
    # GET STATISTICS
    # =============================
    print("=" * 60)
    print("STATISTICS")
    print("=" * 60)
    print()

    stats = intuition.stats()

    print(f"Reflexes created:     {stats['reflexes_created']:,}")
    print(f"Total reflexes:       {stats['total_reflexes']:,}")
    print(f"Fast path hits:       {stats['fast_path_hits']:,}")
    print(f"Avg fast path time:   {stats['avg_fast_path_time_ns']} ns")
    print()

    # =============================
    # CUSTOM CONFIGURATION
    # =============================
    print("=" * 60)
    print("CUSTOM CONFIGURATION")
    print("=" * 60)
    print()

    print("Code:")
    print("  config = neurograph.IntuitionConfig(")
    print("      analysis_interval_secs=30,")
    print("      min_confidence=0.8,")
    print("      enable_fast_path=True")
    print("  )")
    print("  intuition = neurograph.IntuitionEngine.create(config=config)")
    print()

    config = neurograph.IntuitionConfig(
        analysis_interval_secs=30,
        min_confidence=0.8,
        enable_fast_path=True
    )

    print(f"✓ Created config: {config}")
    print()

    intuition_custom = neurograph.IntuitionEngine.create(config=config)
    print("✓ Created IntuitionEngine with custom configuration")
    print()

    # =============================
    # BUILDER PATTERN
    # =============================
    print("=" * 60)
    print("BUILDER PATTERN (Optional Parameters)")
    print("=" * 60)
    print()

    print("Code:")
    print("  intuition = neurograph.IntuitionEngine.create(")
    print("      capacity=50_000,")
    print("      channel_size=5_000")
    print("  )")
    print()

    intuition_builder = neurograph.IntuitionEngine.create(
        capacity=50_000,
        channel_size=5_000
    )

    print("✓ Created IntuitionEngine with custom capacity")
    print()

    stats_builder = intuition_builder.stats()
    print(f"Stats: {stats_builder}")
    print()

    # =============================
    # SUMMARY
    # =============================
    print("=" * 60)
    print("SUMMARY")
    print("=" * 60)
    print()

    print("✓ IntuitionEngine is READY to use")
    print()
    print("Next steps:")
    print("  1. Connect to ExperienceStream (for learning)")
    print("  2. Register fast-path reflexes")
    print("  3. Start analysis cycles")
    print()
    print("Performance:")
    print("  - Fast path lookup: ~30-50ns")
    print("  - Memory efficient: scales to millions of experiences")
    print()


if __name__ == "__main__":
    main()
