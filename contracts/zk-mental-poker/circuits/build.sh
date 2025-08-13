#!/bin/bash
set -e

BUILD_DIR="build"

COMMON_DIR="common"

CIRCUIT_FILES=(
 "shuffle_encrypt/shuffle_encrypt"
  "decrypt/decrypt"
  "test/test_elgamal_encrypt"
  "test/test_elgamal_decrypt"
  "test/test_bandersnatch_add"
  "test/test_bandersnatch_double"
  "test/test_bandersnatch_scalar_mul"
  "test/test_column_permutation"
  "test/test_is_permutation"
  "common/bandersnatch"
)

POT_FILE="pot21_final.ptau"

function compile() {
    echo "Compiling circuits..."
    mkdir -p $BUILD_DIR

    for CIRCUIT_PATH in "${CIRCUIT_FILES[@]}"; do
        CIRCUIT_NAME="$CIRCUIT_PATH.circom"
        echo "➡️  Compiling $CIRCUIT_NAME..."
        mkdir -p $BUILD_DIR/$(dirname "$CIRCUIT_PATH")
        circom "$CIRCUIT_NAME" \
          --r1cs --wasm --sym --O2 --prime bls12381\
          -l $COMMON_DIR \
          -l node_modules/circomlib/circuits \
          -o "$BUILD_DIR/$(dirname "$CIRCUIT_PATH")"
          

        echo "✅ Compiled: $BUILD_DIR/$CIRCUIT_NAME"
    done
}

function setup() {
    echo "Setting up keys..."

    if [ ! -f "$POT_FILE" ]; then
      echo "⚡ Powers of Tau not found. Generating new..."
      snarkjs powersoftau new bls12-381 21 pot21_0000.ptau -v
      snarkjs powersoftau contribute pot21_0000.ptau pot21_0001.ptau --name="First contribution" -v
      snarkjs powersoftau prepare phase2 pot21_0001.ptau "$POT_FILE" -v

    else
      echo "✅ Existing Powers of Tau found."
    fi

    for CIRCUIT_PATH in "${CIRCUIT_FILES[@]}"; do

        CIRCUIT_NAME=$(basename "$CIRCUIT_PATH")
        CIRCUIT_DIR=$(dirname "$CIRCUIT_PATH")
        echo "➡️  Setting up $CIRCUIT_NAME..."

        snarkjs groth16 setup "$BUILD_DIR/$CIRCUIT_DIR/$CIRCUIT_NAME.r1cs" "$POT_FILE" "$BUILD_DIR/$CIRCUIT_DIR/$CIRCUIT_NAME.zkey"
        snarkjs zkey export verificationkey "$BUILD_DIR/$CIRCUIT_DIR/$CIRCUIT_NAME.zkey" "$BUILD_DIR/$CIRCUIT_DIR/$CIRCUIT_NAME.vkey.json"
        echo "✅ Setup done: $BUILD_DIR/$CIRCUIT_DIR/$CIRCUIT_NAME"
    done
}

function all() {
    compile
    setup
}

function help() {
    echo ""
    echo "Usage: ./build.sh [command]"
    echo ""
    echo "Commands:"
    echo "  compile      Compile circom circuit (.r1cs, .wasm, .sym)"
    echo "  setup        Generate proving and verifying keys (.zkey, .vkey.json)"
    echo "  all          Run compile + setup"
    echo "  help         Show this help message"
    echo ""
}

case "$1" in
    compile)
        compile
        ;;
    setup)
        setup
        ;;
    all)
        all
        ;;
    help|*)
        help
        ;;
esac