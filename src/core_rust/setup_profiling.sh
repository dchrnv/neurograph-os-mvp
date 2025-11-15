#!/bin/bash
# Profiling Setup for v0.27.0
# Installs flamegraph and generates performance profiles

set -e

echo "=== NeuroGraph OS v0.27.0 Profiling Setup ==="
echo ""

# Check if flamegraph is installed
if ! command -v cargo-flamegraph &> /dev/null; then
    echo "📦 Installing flamegraph..."
    cargo install flamegraph
    echo "✅ flamegraph installed"
else
    echo "✅ flamegraph already installed"
fi

echo ""
echo "🔥 Generating flamegraphs for demo binaries..."
echo ""

DEMOS=(
    "learning-loop-demo"
    "action-controller-demo"
)

mkdir -p flamegraphs

for demo in "${DEMOS[@]}"; do
    echo "Profiling: $demo"
    echo "---"

    # Check if demo exists
    if ! grep -q "name = \"$demo\"" Cargo.toml; then
        echo "⚠️  Demo $demo not found in Cargo.toml, skipping..."
        continue
    fi

    # Run flamegraph
    cargo flamegraph \
        --bin "$demo" \
        --features demo-tokio \
        --output "flamegraphs/${demo}.svg" \
        2>&1 | tail -10 || {
        echo "⚠️  Failed to profile $demo (may require sudo for perf)"
    }

    echo ""
done

echo "✅ Profiling complete!"
echo ""
echo "📂 Flamegraphs available at:"
for demo in "${DEMOS[@]}"; do
    if [ -f "flamegraphs/${demo}.svg" ]; then
        echo "   - flamegraphs/${demo}.svg"
    fi
done

echo ""
echo "To view flamegraphs:"
echo "   open flamegraphs/*.svg"
echo "   # or use a browser"
echo ""
echo "Note: Flamegraph profiling may require:"
echo "   - sudo access for 'perf' tool"
echo "   - kernel.perf_event_paranoid setting adjustment"
echo "   - Run: echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid"