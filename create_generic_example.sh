#!/bin/bash

# Script to create a minimal generic support example

EXAMPLE_DIR="frb_example/generic_support"

echo "Creating generic support example structure..."

# Create directory structure
mkdir -p "$EXAMPLE_DIR"/{rust/src,lib/src/rust,test}

# Create Rust lib.rs
cat > "$EXAMPLE_DIR/rust/src/lib.rs" << 'EOF'
pub mod api;
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
EOF

# Create Rust API with simple generics
cat > "$EXAMPLE_DIR/rust/src/api.rs" << 'EOF'
// Simple generic types for initial testing

/// Basic generic struct with single type parameter
pub struct Container<T> {
    pub value: T,
}

/// Generic function
pub fn create_string_container() -> Container<String> {
    Container {
        value: "hello".to_string(),
    }
}

pub fn get_container_value(container: Container<String>) -> String {
    container.value
}
EOF

# Create Cargo.toml
cat > "$EXAMPLE_DIR/rust/Cargo.toml" << 'EOF'
[package]
name = "frb_example_generic_support"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "staticlib"]

[dependencies]
flutter_rust_bridge = { path = "../../frb_rust" }
EOF

# Create pubspec.yaml
cat > "$EXAMPLE_DIR/pubspec.yaml" << 'EOF'
name: frb_example_generic_support
description: Generic support testing example
version: 0.1.0
publish_to: 'none'

environment:
  sdk: '>=2.18.0 <4.0.0'

dependencies:
  flutter_rust_bridge: ^2.0.0

dev_dependencies:
  test: ^1.21.0
EOF

# Create FRB config
cat > "$EXAMPLE_DIR/flutter_rust_bridge.yaml" << 'EOF'
rust_input: rust/src/api.rs
dart_output: lib/src/rust/
rust_output: rust/src/frb_generated.rs
EOF

# Create build.dart
cat > "$EXAMPLE_DIR/build.dart" << 'EOF'
import 'package:flutter_rust_bridge_codegen/flutter_rust_bridge_codegen.dart';

Future<void> main(List<String> args) async {
  await generateForFlutterRustBridge();
}
EOF

# Create basic Dart test
cat > "$EXAMPLE_DIR/test/generic_test.dart" << 'EOF'
import 'package:test/test.dart';
// Import will be generated after running build

void main() {
  group('Generic Support Tests', () {
    test('basic container test', () {
      // Test implementation will be added after code generation works
      expect(true, isTrue); // Placeholder
    });
  });
}
EOF

echo "Generic support example created at $EXAMPLE_DIR"
echo "Next steps:"
echo "1. Complete generic implementation in frb_codegen"
echo "2. Run code generation: ./frb_internal generate-run-frb-codegen-command-generate --package frb_example--generic_support"
echo "3. Add to CI pipeline"
EOF
