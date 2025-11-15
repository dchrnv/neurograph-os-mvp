#!/bin/bash
# Coverage Setup for v0.27.0
# Installs cargo-tarpaulin and generates coverage report

set -e

echo "=== NeuroGraph OS v0.27.0 Coverage Setup ==="
echo ""

# Check if tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null; then
    echo "📦 Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin
    echo "✅ cargo-tarpaulin installed"
else
    echo "✅ cargo-tarpaulin already installed"
fi

echo ""
echo "📊 Generating coverage report..."
echo "   (This may take a few minutes)"
echo ""

# Run tarpaulin with HTML output
# --skip-clean: Don't clean before running
# --ignore-tests: Don't include test code in coverage
# --out Html: Generate HTML report
# --output-dir coverage: Output directory

cargo tarpaulin \
    --skip-clean \
    --ignore-tests \
    --out Html \
    --output-dir coverage \
    --exclude-files "src/bin/*" "src/ffi/*" "benches/*" "tests/*" \
    --timeout 300 \
    2>&1 | tee coverage/tarpaulin.log

echo ""
echo "✅ Coverage report generated!"
echo ""
echo "📂 Reports available at:"
echo "   - HTML: coverage/index.html"
echo "   - Log:  coverage/tarpaulin.log"
echo ""
echo "To view the report:"
echo "   open coverage/index.html"
echo "   # or"
echo "   xdg-open coverage/index.html"