# Flutter Rust Bridge: Generic Types and External Crate Support Analysis

## Overview
This document analyzes how Flutter Rust Bridge (FRB) supports special generic types and external crate types, providing essential information for implementing support for new external generic types.

## Current Special Generic Type Support

### 1. Built-in Generic Types (Hardcoded Support)

FRB has hardcoded support for specific generic types in `frb_codegen/src/library/codegen/parser/mir/parser/ty/concrete.rs`:

```rust
// In parse_type_path_data_concrete method:
("Vec", [element]) => mir_list(self.parse_type(element)?, true),
("HashMap", [key, value]) => self.parse_mir_hash_map(key, value, None)?,
("HashMap", [key, value, hasher]) => self.parse_mir_hash_map(key, value, Some(hasher))?,
("HashSet", [inner]) => self.parse_mir_hash_set(inner, None)?,
("HashSet", [inner, hasher]) => self.parse_mir_hash_set(inner, Some(hasher))?,
("Box", [inner]) => {
    let inner = self.parse_type(inner)?;
    // ... special handling for Box<T>
}
```

**Key Insight**: Generic types are parsed by:
1. Pattern matching on the type name (e.g., "Vec", "HashMap")
2. Extracting generic arguments 
3. Recursively parsing inner types
4. Creating specific `MirTypeDelegate` variants

### 2. Delegate Type System

Generic types are represented as delegate types in `frb_codegen/src/library/codegen/ir/mir/ty/delegate.rs`:

```rust
pub enum MirTypeDelegate {
    // Collections
    Map(MirTypeDelegateMap),
    Set(MirTypeDelegateSet),
    
    // Standard library types
    String,
    Char,
    Uuid,  // External crate support
    
    // Time types (external crate)
    Time(MirTypeDelegateTime),
    
    // Special generics
    Array(MirTypeDelegateArray),
    // ... other variants
}
```

**Key Insight**: Each supported generic/external type has a dedicated delegate variant that:
- Stores the generic parameters as `Box<MirType>`
- Handles serialization/deserialization logic
- Manages type information for code generation

### 3. External Crate Type Support (UUID Example)

#### 3.1 Feature Flag Integration
In `frb_rust/Cargo.toml`:
```toml
[features]
uuid = ["dep:uuid", "allo-isolate/uuid"]

[dependencies]
uuid = { workspace = true, optional = true }
```

#### 3.2 Type Recognition
In `concrete.rs`:
```rust
("Uuid", []) if check_prefix("uuid") => Delegate(MirTypeDelegate::Uuid),
```

**Pattern**: External types are recognized by:
1. Type name matching ("Uuid")
2. Crate prefix checking (`check_prefix("uuid")`)
3. No generic parameters (`[]`)

#### 3.3 Codec Implementation
In `frb_codegen/src/library/codegen/generator/codec/sse/ty/delegate.rs`:

**Dart Side Encoding**:
```rust
MirTypeDelegate::Uuid => "self.toBytes()".to_owned(),
```

**Rust Side Encoding**:
```rust
MirTypeDelegate::Uuid => "self.as_bytes().to_vec()".to_owned(),
```

**Dart Side Decoding**:
```rust
MirTypeDelegate::Uuid => "UuidValue.fromByteList(inner)".to_owned(),
```

**Rust Side Decoding**:
```rust
MirTypeDelegate::Uuid => {
    r#"uuid::Uuid::from_slice(&inner).expect("fail to decode uuid")"#.to_owned()
}
```

## External Type Support (Mirroring System)

### 1. Mirror Attribute Parsing
In `frb_codegen/src/library/codegen/parser/mir/parser/attribute.rs`:

```rust
pub(crate) fn mirror(&self) -> Vec<Path> {
    self.0.iter()
        .filter_map(|item| if_then_some!(let FrbAttribute::Mirror(mirror) = item, mirror.0.clone()))
        .flatten()
        .collect()
}
```

### 2. Mirror Processing
In `frb_codegen/src/library/codegen/parser/hir/flat/parser/mirror_ident.rs`:

```rust
pub(crate) fn parse_mirror_ident(
    ident: &Ident,
    attrs: &[Attribute],
) -> anyhow::Result<ParseMirrorIdentOutput> {
    let attributes = FrbAttributes::parse(attrs)?;
    let mirror_info = attributes.mirror();
    
    // Extract mirror type names from attributes
    let res = mirror_info.into_iter()
        .filter_map(|path| {
            // Extract simple identifiers from mirror paths
        })
        .collect_vec();
}
```

### 3. Mirror Usage Pattern
```rust
// Import the external type
pub use external_lib::ExternalType;

// Create mirror definition  
#[frb(mirror(ExternalType))]
pub struct _ExternalType {
    pub field1: String,
    pub field2: i32,
}

// Use in functions
pub fn use_external_type(arg: ExternalType) -> ExternalType {
    arg
}
```

## Requirements for Adding New External Generic Type Support

### 1. Type Recognition Layer
**Location**: `frb_codegen/src/library/codegen/parser/mir/parser/ty/concrete.rs`

**Required Changes**:
```rust
// Add pattern matching for new type
("NewGenericType", [inner]) if check_prefix("new_crate") => {
    Delegate(MirTypeDelegate::NewGenericType(MirTypeDelegateNewGenericType {
        inner: Box::new(self.parse_type(inner)?),
        // other fields as needed
    }))
}
```

### 2. Delegate Type Definition
**Location**: `frb_codegen/src/library/codegen/ir/mir/ty/delegate.rs`

**Required Changes**:
```rust
pub enum MirTypeDelegate {
    // ... existing variants
    NewGenericType(MirTypeDelegateNewGenericType),
}

pub struct MirTypeDelegateNewGenericType {
    pub inner: Box<MirType>,
    // Additional metadata fields
}
```

### 3. Codec Implementation
**Location**: `frb_codegen/src/library/codegen/generator/codec/sse/ty/delegate.rs`

**Required Changes**:
```rust
// Encoding logic
MirTypeDelegate::NewGenericType(_) => {
    // Dart: how to convert Dart object to bytes
    // Rust: how to convert Rust object to bytes
}

// Decoding logic  
MirTypeDelegate::NewGenericType(_) => {
    // Dart: how to reconstruct Dart object from bytes
    // Rust: how to reconstruct Rust object from bytes
}
```

### 4. API Dart Generator
**Location**: `frb_codegen/src/library/codegen/generator/api_dart/spec_generator/info.rs`

**Required Changes**:
```rust
// Dart type name mapping
MirTypeDelegate::NewGenericType => "DartTypeName".to_owned(),

// Import statements
MirTypeDelegate::NewGenericType => {
    Some("import 'package:new_crate_dart/new_crate_dart.dart';".to_owned())
}
```

### 5. Feature Flag Integration
**Location**: `frb_rust/Cargo.toml`

**Required Changes**:
```toml
[features]
new-crate = ["dep:new-crate", "allo-isolate/new-crate"]

[dependencies]
new-crate = { workspace = true, optional = true }
```

### 6. Wire Protocol Generator
**Location**: `frb_codegen/src/library/codegen/generator/wire/rust/spec_generator/codec/cst/decoder/ty/delegate.rs`

**Required Changes**:
```rust
MirTypeDelegate::NewGenericType => {
    // CST decoder implementation for the new type
    Acc::distribute(/* implementation */)
}
```

## Code Generation Pipeline

### 1. Parsing Phase
1. **HIR (High-level IR)**: Raw Rust AST parsing
2. **MIR (Mid-level IR)**: Type resolution and attribute processing
3. **Type Parser**: Generic type recognition and delegate creation

### 2. Code Generation Phase
1. **API Dart Generator**: Generates Dart type definitions and imports
2. **Codec Generator**: Generates serialization/deserialization code
3. **Wire Generator**: Generates low-level communication protocol code

### 3. Integration Phase
1. **Template System**: Project scaffolding and integration
2. **Feature Flags**: Optional dependency management
3. **Build System**: Platform-specific compilation

## Essential Implementation Steps

### Step 1: Define the Delegate Type
1. Add enum variant to `MirTypeDelegate`
2. Define struct for type parameters and metadata
3. Implement trait bounds and type constraints

### Step 2: Implement Type Recognition
1. Add pattern matching in `concrete.rs`
2. Handle generic parameter parsing
3. Validate crate prefix and type structure

### Step 3: Implement Codec Logic
1. Define serialization format (usually byte arrays)
2. Implement encoding for both Dart and Rust sides
3. Implement decoding with proper error handling
4. Handle edge cases and validation

### Step 4: Generate API Bindings
1. Define Dart type mapping
2. Add required import statements
3. Handle type annotations and metadata

### Step 5: Feature Integration
1. Add feature flag support
2. Update dependency management
3. Ensure optional compilation

### Step 6: Testing and Validation
1. Create test cases for the new type
2. Validate serialization round-trips
3. Test error handling and edge cases
4. Verify platform compatibility

## Key Considerations

### 1. Serialization Format
- Must be platform-independent
- Should be efficient for the target use case
- Must handle all possible values of the type
- Should provide good error messages

### 2. Generic Parameter Handling
- Recursive type parsing for nested generics
- Type constraint validation
- Lifetime parameter handling (if applicable)

### 3. Error Handling
- Graceful degradation for unsupported features
- Clear error messages for type mismatches
- Validation of external type compatibility

### 4. Performance Considerations
- Minimize serialization overhead
- Efficient memory usage for large collections
- Lazy evaluation where possible

### 5. Compatibility
- Cross-platform byte layout
- Version compatibility with external crates
- Future-proofing for API changes

## Conclusion

Adding support for a new external generic type in FRB requires:

1. **Type System Integration**: Adding delegate types and recognition logic
2. **Codec Implementation**: Defining serialization/deserialization protocols  
3. **Code Generation**: Creating proper Dart/Rust API bindings
4. **Build System Integration**: Feature flags and dependency management
5. **Testing and Validation**: Comprehensive test coverage

The key insight is that FRB uses a delegate pattern where each supported type has dedicated handling logic throughout the entire pipeline, from parsing to code generation. This provides flexibility but requires implementation at multiple layers.
