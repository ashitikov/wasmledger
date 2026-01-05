#!/bin/bash
# Example: Execute a WASM function via HTTP API

# Usage:
#   ./execute_function.sh path/to/function.wasm
#   ./execute_function.sh path/to/function.wasm '{"key": "value"}'

WASM_FILE="${1:-function.wasm}"
INPUT="${2:-}"

if [ ! -f "$WASM_FILE" ]; then
    echo "Error: WASM file not found: $WASM_FILE"
    exit 1
fi

echo "Executing function: $WASM_FILE"

# Build curl command
if [ -z "$INPUT" ]; then
    # Without input
    curl -X POST http://localhost:3000/execute \
        -F "wasm=@$WASM_FILE" \
        -H "Accept: application/json" \
        | jq .
else
    # With input
    curl -X POST http://localhost:3000/execute \
        -F "wasm=@$WASM_FILE" \
        -F "input=$INPUT" \
        -H "Accept: application/json" \
        | jq .
fi
