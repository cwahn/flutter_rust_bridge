# FRB Code Generation Pipeline Analysis - Focus on Generic & External Types

## Step 1: Understanding Type Recognition in Concrete Parser

Let me examine exactly how UUID (external) and Vec (generic) are recognized in the parser.

**File**: `frb_codegen/src/library/codegen/parser/mir/parser/ty/concrete.rs`

### External Type Recognition (UUID Example):
```rust
("Uuid", []) if check_prefix("uuid") => Delegate(MirTypeDelegate::Uuid),
```

**Pattern Analysis**:
- `"Uuid"` - Exact type name match
- `[]` - No generic parameters (empty array)
- `check_prefix("uuid")` - Must be from `uuid` crate
- `Delegate(MirTypeDelegate::Uuid)` - Creates delegate variant

### Generic Type Recognition (Vec Example):
```rust
("Vec", [element]) => mir_list(self.parse_type(element)?, true),
```

**Pattern Analysis**:
- `"Vec"` - Exact type name match
- `[element]` - One generic parameter (element type)
- No crate prefix check (built-in type)
- `mir_list(...)` - Special function to create list type
- `self.parse_type(element)?` - **RECURSIVELY** parse the element type

### Multi-Parameter Generic Type (HashMap Example):
```rust
("HashMap", [key, value]) => {
    self.parse_mir_hash_map(key, value, None)?
},
("HashMap", [key, value, hasher]) => {
    self.parse_mir_hash_map(key, value, Some(hasher))?
},
```

**Pattern Analysis**:
- `"HashMap"` - Exact type name match
- `[key, value]` or `[key, value, hasher]` - Multiple generic parameters
- `self.parse_mir_hash_map(...)` - Delegate to specialized parsing function
- Each parameter is parsed recursively

**CRITICAL INSIGHT**: The parser uses **destructuring patterns** to extract generic arguments and **recursive parsing** to handle nested generics.

## Step 2: Generic Parameter Parsing Functions

### Vec Processing - `mir_list` Function:
**File**: `frb_codegen/src/library/codegen/ir/mir/ty/general_list.rs`

```rust
pub(crate) fn mir_list(inner: MirType, strict_dart_type: bool) -> MirType {
    match inner {
        // Optimization: Primitive types become PrimitiveList for efficiency
        MirType::Primitive(inner) if inner != MirTypePrimitive::Bool => {
            PrimitiveList(MirTypePrimitiveList {
                primitive: inner.clone(),
                strict_dart_type,
            })
        }
        // Generic case: Any other type becomes GeneralList
        _ => GeneralList(MirTypeGeneralList {
            inner: Box::new(inner),  // Store the element type
        }),
    }
}
```

**Key Pattern**: `Vec<T>` becomes either:
- `PrimitiveList` for primitive types (optimized)
- `GeneralList` for complex types (generic)

### HashMap Processing - `parse_mir_hash_map` Function:
**File**: `frb_codegen/src/library/codegen/parser/mir/parser/ty/concrete.rs`

```rust
fn parse_mir_hash_map(
    &mut self,
    key: &Type,      // K parameter from HashMap<K,V>
    value: &Type,    // V parameter from HashMap<K,V>
    hasher: Option<&Type>,  // Optional hasher parameter
) -> anyhow::Result<MirType> {
    // STEP 1: Recursively parse each generic parameter
    let key = self.parse_type(key)?;     // Parse K
    let value = self.parse_type(value)?; // Parse V
    let hasher = hasher.map(|hasher| self.parse_type(hasher)).transpose()?; // Parse hasher if present

    // STEP 2: Create delegate with parsed parameters
    Ok(Delegate(MirTypeDelegate::Map(MirTypeDelegateMap {
        key: Box::new(key.clone()),       // Store K
        value: Box::new(value.clone()),   // Store V
        hasher: hasher.map(Box::new),     // Store optional hasher
        element_delegate: self.create_mir_record(vec![key, value]), // For serialization
    })))
}
```

**Key Pattern for Multi-Parameter Generics**:
1. **Extract each parameter** from the generic type
2. **Recursively parse** each parameter with `self.parse_type()`
3. **Store in delegate struct** with `Box::new()` for each parameter
4. **Create element_delegate** for serialization protocol

**CRITICAL INSIGHT**: Both simple generics (`Vec<T>`) and complex generics (`HashMap<K,V>`) follow the same pattern:
1. Extract parameters from generic type arguments
2. Recursively parse each parameter
3. Store parsed parameters in a delegate structure

## Next Step
Let me examine the delegate type definitions to understand how these parsed parameters are stored.

## Step 2: Understanding the Parse Phase Detail

The parse phase has multiple sub-phases in `frb_codegen/src/library/codegen/parser/mod.rs`:

```rust
fn parse_inner(config, dumper, progress_bar_pack, on_hir_flat) -> Result<MirPack> {
    // 1. HIR RAW: Parse raw Rust AST
    let hir_raw = hir::raw::parse(&config.hir, dumper)?;
    
    // 2. HIR TREE: Build AST tree structure  
    let hir_tree = hir::tree::parse(&config.hir, hir_raw, &dumper_hir_tree)?;
    
    // 3. HIR NAIVE FLAT: Flatten the tree into list
    let hir_naive_flat = hir::naive_flat::parse(&config.hir, hir_tree, &dumper_hir_naive_flat)?;
    
    // 4. HIR FLAT: Process attributes and structure
    let hir_flat = hir::flat::parse(&config.hir, hir_naive_flat, &dumper_hir_flat)?;
    
    // 5. EARLY GENERATOR: Prepare for MIR conversion
    let ir_early_generator = early_generator::execute(hir_flat, &config.mir, &dumper_early_generator)?;
    
    // 6. MIR: Convert to Mid-level IR with type resolution
    let mir_pack = mir::parse(&config.mir, &ir_early_generator, &dumper_mir, mir::ParseMode::Normal)?;
}
```

**Key Stages**:
- **HIR** (High-level IR): Raw Rust AST parsing
- **MIR** (Mid-level IR): Type resolution and semantic analysis

## Step 3: Understanding MIR Type Parsing

The MIR parsing creates a `TypeParser` which is the core component for handling types in `frb_codegen/src/library/codegen/parser/mir/parser/mod.rs`:

```rust
pub(crate) fn parse(config, ir_pack, parse_mode) -> Result<MirPack> {
    let hir_flat = &ir_pack.hir_flat_pack;
    let structs_map = hir_flat.structs_map();
    let enums_map = hir_flat.enums_map();

    // KEY: This is where type parsing happens
    let mut type_parser = TypeParser::new_from_pack(ir_pack);
    
    // Parse functions and their types
    let custom_ser_des_infos = custom_ser_des::parse(&hir_flat.functions, &mut type_parser, &context)?;
    
    // Parse trait implementations
    let trait_impls = trait_impl::parse(&hir_flat.trait_impls, &mut type_parser, ...)?;
    
    // Parse functions (this is where generic types in function signatures are processed)
    let (funcs_all, funcs_skip) = function::parse(config, &hir_flat.functions, &hir_flat.constants, &mut type_parser, &structs_map, parse_mode)?;
    
    // Parse extra types (structs/enums)
    let extra_types_all = extra_type::parse(config, &structs_map, &enums_map, &mut type_parser, parse_mode)?;
}
```

**Key Insight**: The `TypeParser` is the central component that converts Rust types into FRB's internal type representation.

## Step 4: Understanding TypePath Parsing for Generic Types

The `TypeParser` handles different types through a fallback chain in `frb_codegen/src/library/codegen/parser/mir/parser/ty/path.rs`:

```rust
fn parse_type_path_core(type_path: &TypePath, path: &Path) -> Result<MirType> {
    let segments = extract_path_data(path)?;
    let splayed_segments = splay_segments(&segments);

    if let Some(last_segment) = splayed_segments.last() {
        // 1. Custom serialization/deserialization
        if let Some(ans) = self.parse_type_path_data_custom_ser_des(last_segment)? { return Ok(ans); }
        
        // 2. Primitive types (i32, f64, etc)
        if let Some(ans) = self.parse_type_path_data_primitive(last_segment)? { return Ok(ans); }
        
        // 3. CONCRETE TYPES - This is where Vec<T>, HashMap<K,V>, uuid::Uuid are handled!
        if let Some(ans) = self.parse_type_path_data_concrete(last_segment, &splayed_segments)? { return Ok(ans); }
        
        // 4. Struct types
        if let Some(ans) = self.parse_type_path_data_struct(path, last_segment, None)? { return Ok(ans); }
        
        // 5. Enum types  
        if let Some(ans) = self.parse_type_path_data_enum(path, last_segment)? { return Ok(ans); }
        
        // 6. Trait types
        if let Some(ans) = self.parse_type_path_data_trait(last_segment)? { return Ok(ans); }
        
        // 7. Rust opaque types
        if let Some(ans) = self.parse_type_path_data_rust_opaque(last_segment)? { return Ok(ans); }
        
        // 8. Auto opaque explicit
        if let Some(ans) = self.parse_type_path_data_rust_auto_opaque_explicit(last_segment)? { return Ok(ans); }
        
        // 9. Optional types
        if let Some(ans) = self.parse_type_path_data_optional(type_path, last_segment)? { return Ok(ans); }
    }

    // Fallback: treat as auto opaque
    self.parse_type_rust_auto_opaque_implicit(None, &syn::Type::Path(type_path.to_owned()), None, None)
}
```

**KEY FINDING**: Generic types like `Vec<T>`, `HashMap<K,V>`, and external types like `uuid::Uuid` are handled in the **`parse_type_path_data_concrete`** method!

## Step 5: Core Generic Type Parsing Logic - The Concrete Types

Found it! The `parse_type_path_data_concrete` method in `concrete.rs` is where ALL special type handling happens:

```rust
pub(crate) fn parse_type_path_data_concrete(
    &mut self,
    last_segment: &SplayedSegment,  // e.g., ("Vec", [element_type])
    splayed_segments: &[SplayedSegment],
) -> anyhow::Result<Option<MirType>> {
    // Extract namespace/crate prefix (e.g., "std", "uuid", "chrono")
    let non_last_segments = (splayed_segments.split_last().unwrap().1.iter())
        .map(|segment| segment.0)
        .join("::");
    let check_prefix = |matcher: &str| non_last_segments == matcher || non_last_segments.is_empty();

    Ok(Some(match last_segment {
        // External crate types
        ("Duration", []) if check_prefix("chrono") => Delegate(MirTypeDelegate::Time(MirTypeDelegateTime::Duration)),
        ("NaiveDateTime", []) if check_prefix("chrono") => Delegate(MirTypeDelegate::Time(MirTypeDelegateTime::Naive)),
        ("DateTime", args) if check_prefix("chrono") => self.parse_datetime(args)?,
        ("Uuid", []) if check_prefix("uuid") => Delegate(MirTypeDelegate::Uuid),
        
        // Built-in types
        ("String", []) | ("str", []) => Delegate(MirTypeDelegate::String),
        ("char", []) => Delegate(MirTypeDelegate::Char),
        ("Backtrace", []) => Delegate(MirTypeDelegate::Backtrace),
        
        // Generic types - THE KEY PATTERNS
        ("Box", [inner]) => {
            let inner = self.parse_type(inner)?;  // Recursively parse inner type
            // Special handling for Box<T>...
        },
        
        ("Vec", [element]) => mir_list(self.parse_type(element)?, true),  // Recursively parse element type
        
        ("HashMap", [key, value]) => self.parse_mir_hash_map(key, value, None)?,  // Parse both key and value types
        ("HashMap", [key, value, hasher]) => self.parse_mir_hash_map(key, value, Some(hasher))?,
        
        ("HashSet", [inner]) => self.parse_mir_hash_set(inner, None)?,
        ("HashSet", [inner, hasher]) => self.parse_mir_hash_set(inner, Some(hasher))?,
        
        ("StreamSink", [inner]) => Delegate(MirTypeDelegate::StreamSink(MirTypeDelegateStreamSink {
            inner_ok: Box::new(self.parse_type(inner)?),  // Recursively parse inner type
            inner_err: stream_sink_err_type(),
            codec: self.context.default_stream_sink_codec,
        })),
        
        _ => return Ok(None),  // Not a concrete type, try other parsers
    }))
}
```

**CRITICAL PATTERN FOR ADDING NEW GENERIC TYPES**:
1. **Pattern Match**: `("TypeName", [arg1, arg2, ...])` for generic types
2. **Crate Check**: `if check_prefix("crate_name")` for external crates  
3. **Recursive Parsing**: `self.parse_type(inner)?` for each generic argument
4. **Delegate Creation**: Return `Delegate(MirTypeDelegate::YourType(...))`

## Step 6: Understanding Generic Type Structure in MIR

### HashMap Example - Multi-Parameter Generic Type

The `parse_mir_hash_map` method shows how multi-parameter generics are handled:

```rust
fn parse_mir_hash_map(
    &mut self,
    key: &Type,
    value: &Type, 
    hasher: Option<&Type>,
) -> anyhow::Result<MirType> {
    // STEP 1: Recursively parse each generic parameter
    let key = self.parse_type(key)?;
    let value = self.parse_type(value)?;
    let hasher = hasher.map(|hasher| self.parse_type(hasher)).transpose()?;

    // STEP 2: Create delegate type with parsed parameters
    Ok(Delegate(MirTypeDelegate::Map(MirTypeDelegateMap {
        key: Box<new(key.clone()),
        value: Box::new(value.clone()),
        hasher: hasher.map(Box::new),
        element_delegate: self.create_mir_record(vec![key, value]),  // For serialization
    })))
}
```

### MirTypeDelegate Enum Structure

In `delegate.rs`, each supported special type has its own variant:

```rust
pub enum MirTypeDelegate {
    // Simple external types (no generics)
    String,
    Char, 
    Uuid,           // uuid::Uuid - simple external type
    Backtrace,
    
    // Generic types with parameters
    Map(MirTypeDelegateMap),       // HashMap<K,V>
    Set(MirTypeDelegateSet),       // HashSet<T>
    StreamSink(MirTypeDelegateStreamSink),  // StreamSink<T>
    Array(MirTypeDelegateArray),   // [T; N]
    
    // External crate generics
    Time(MirTypeDelegateTime),     // chrono types
    
    // Other special cases...
}

// Each generic type has its own struct to hold parameters
pub struct MirTypeDelegateMap {
    pub key: Box<MirType>,          // K parameter
    pub value: Box<MirType>,        // V parameter  
    pub hasher: Option<Box<MirType>>, // Optional hasher parameter
    pub element_delegate: MirTypeRecord, // For serialization protocol
}

pub struct MirTypeDelegateSet {
    pub inner: Box<MirType>,        // T parameter
    pub hasher: Option<Box<MirType>>, // Optional hasher parameter
}
```

**KEY INSIGHT**: Each generic type needs:
1. **Enum variant** in `MirTypeDelegate`
2. **Struct definition** to hold generic parameters as `Box<MirType>`
3. **Pattern matching** in `concrete.rs` to recognize and parse it
4. **Recursive parsing** of all generic parameters

## Step 7: Code Generation Phase - API Dart Generation

The generator phase converts MIR types into actual Dart/Rust code. Looking at `api_dart/spec_generator/info.rs`:

### Dart Type Name Generation

```rust
fn dart_api_type(&self) -> String {
    match &self.mir {
        // Simple types
        MirTypeDelegate::String => "String".to_string(),
        MirTypeDelegate::Char => "String".to_string(),
        MirTypeDelegate::Uuid => "UuidValue".to_owned(),    // External type
        MirTypeDelegate::Backtrace => "String".to_string(),
        
        // Time types (external crate)
        MirTypeDelegate::Time(mir) => match mir {
            MirTypeDelegateTime::Local | MirTypeDelegateTime::Utc | MirTypeDelegateTime::Naive => "DateTime".to_string(),
            MirTypeDelegateTime::Duration => "Duration".to_string(),
        },
        
        // Generic types - RECURSIVE GENERATION
        MirTypeDelegate::Map(mir) => format!(
            "Map<{}, {}>",
            ApiDartGenerator::new(*mir.key.clone(), self.context).dart_api_type(),     // Recursively generate key type
            ApiDartGenerator::new(*mir.value.clone(), self.context).dart_api_type(),  // Recursively generate value type
        ),
        
        MirTypeDelegate::Set(mir) => format!(
            "Set<{}>", 
            ApiDartGenerator::new(*mir.inner.clone(), self.context).dart_api_type()   // Recursively generate inner type
        ),
    }
}
```

### Dart Import Generation

```rust
fn dart_import(&self) -> Option<String> {
    match &self.mir {
        MirTypeDelegate::Uuid => {
            Some("import 'package:uuid/uuid.dart';".to_owned())    // External package import
        }
        _ => None,
    }
}
```

**KEY PATTERN FOR DART GENERATION**:
1. **Type Names**: Map delegate types to Dart type names
2. **Recursive Generation**: For generic types, recursively generate inner type names
3. **Import Statements**: Add package imports for external types

## Step 8: Codec Generation - Serialization/Deserialization Logic

The codec generation in `codec/sse/ty/delegate.rs` handles how types are serialized between Dart and Rust. Each delegate type needs both encoding and decoding logic:

### UUID Example (Simple External Type)

```rust
// ENCODING: Type → Bytes
Lang::DartLang(_) => match &self.mir {
    MirTypeDelegate::Uuid => "self.toBytes()".to_owned(),  // Dart: UuidValue → List<int>
}
Lang::RustLang(_) => match &self.mir {
    MirTypeDelegate::Uuid => "self.as_bytes().to_vec()".to_owned(),  // Rust: uuid::Uuid → Vec<u8>
}

// DECODING: Bytes → Type  
Lang::DartLang(_) => match &self.mir {
    MirTypeDelegate::Uuid => "UuidValue.fromByteList(inner)".to_owned(),  // List<int> → UuidValue
}
Lang::RustLang(_) => match &self.mir {
    MirTypeDelegate::Uuid => r#"uuid::Uuid::from_slice(&inner).expect("fail to decode uuid")"#.to_owned(),  // Vec<u8> → uuid::Uuid
}
```

### HashMap Example (Generic Type)

```rust
// ENCODING: HashMap<K,V> → List of (K,V) tuples
Lang::DartLang(_) => match &self.mir {
    MirTypeDelegate::Map(_) => "self.entries.map((e) => (e.key, e.value)).toList()".to_owned(),  // Dart: Map<K,V> → List<(K,V)>
}
Lang::RustLang(_) => match &self.mir {
    MirTypeDelegate::Map(_) => "self.into_iter().collect()".to_owned(),  // Rust: HashMap<K,V> → Vec<(K,V)>
}

// DECODING: List of tuples → HashMap<K,V>
Lang::DartLang(_) => match &self.mir {
    MirTypeDelegate::Map(_) => "Map.fromEntries(inner.map((e) => MapEntry(e.$1, e.$2)))".to_owned(),  // List<(K,V)> → Map<K,V>
}
Lang::RustLang(_) => match &self.mir {
    MirTypeDelegate::Map(_) => "inner.into_iter().collect()".to_owned(),  // Vec<(K,V)> → HashMap<K,V>
}
```

**KEY CODEC PATTERNS**:
1. **Byte Serialization**: External types often convert to/from byte arrays
2. **Collection Flattening**: Generic collections convert to simpler structures (e.g., Map → List of tuples)
3. **Platform-Specific**: Different logic for Dart vs Rust sides
4. **Error Handling**: Rust decode often uses `.expect()` for error messages

**CRITICAL REQUIREMENT**: For any new type, you MUST implement all 4 combinations:
- Dart Encode, Dart Decode, Rust Encode, Rust Decode

## Step 9: Complete UUID Implementation Trace - The Full Pattern

Let me trace how `uuid::Uuid` is implemented across ALL layers to understand the complete pattern:

### 1. Type Recognition (Parser Layer)
**File**: `parser/mir/parser/ty/concrete.rs`
```rust
("Uuid", []) if check_prefix("uuid") => Delegate(MirTypeDelegate::Uuid),
```

### 2. Delegate Definition (Type System Layer)  
**File**: `ir/mir/ty/delegate.rs`
```rust
pub enum MirTypeDelegate {
    Uuid,  // Simple enum variant (no generics)
    // ...
}

// Methods for type information:
fn get_delegate(&self) -> MirType {
    match self {
        MirTypeDelegate::Uuid => MirType::PrimitiveList(MirTypePrimitiveList {
            primitive: MirTypePrimitive::U8,  // Serialized as byte array
        }),
    }
}

fn dart_api_type(&self) -> String {
    match self {
        MirTypeDelegate::Uuid => "uuid::Uuid".to_owned(),
    }
}

fn rust_api_type(&self) -> String {
    match self {
        MirTypeDelegate::Uuid => "Uuid".to_owned(),
    }
}
```

### 3. API Dart Generation (Binding Layer)
**File**: `generator/api_dart/spec_generator/info.rs`
```rust
fn dart_api_type(&self) -> String {
    match &self.mir {
        MirTypeDelegate::Uuid => "UuidValue".to_owned(),  // Dart type name
    }
}

fn dart_import(&self) -> Option<String> {
    match &self.mir {
        MirTypeDelegate::Uuid => Some("import 'package:uuid/uuid.dart';".to_owned()),  // Package import
    }
}
```

### 4. Codec Generation (Serialization Layer)
**File**: `generator/codec/sse/ty/delegate.rs`
```rust
// Dart Encode: UuidValue → List<int>
MirTypeDelegate::Uuid => "self.toBytes()".to_owned(),

// Rust Encode: uuid::Uuid → Vec<u8>  
MirTypeDelegate::Uuid => "self.as_bytes().to_vec()".to_owned(),

// Dart Decode: List<int> → UuidValue
MirTypeDelegate::Uuid => "UuidValue.fromByteList(inner)".to_owned(),

// Rust Decode: Vec<u8> → uuid::Uuid
MirTypeDelegate::Uuid => r#"uuid::Uuid::from_slice(&inner).expect("fail to decode uuid")"#.to_owned(),
```

### 5. Wire Protocol Generation (Low-level Protocol Layer)
**File**: `generator/wire/*/decoder/ty/delegate.rs` and `encoder/ty/delegate.rs`
```rust
// CST (C-style) decoder/encoder
MirTypeDelegate::Uuid => Acc::distribute(/* CST protocol handling */),

// DCO (Dart object) decoder  
MirTypeDelegate::Uuid => /* DCO protocol handling */,
```

**THE COMPLETE IMPLEMENTATION PATTERN**:
1. **Parser**: Pattern match type name + crate prefix → Create delegate
2. **Type System**: Define delegate variant + serialization metadata 
3. **API Generation**: Map to target language types + imports
4. **Codec**: Implement 4-way serialization (Dart↔Bytes↔Rust)
5. **Wire Protocol**: Low-level transport encoding/decoding
6. **Feature Flag**: Optional dependency management

## Step 10: Implementation Guide for Adding New Generic External Type

Based on my analysis, here's the complete step-by-step process to add support for a new external generic type:

### EXAMPLE: Adding `MyExternal<T>` from `my_crate`

#### Step 1: Add Delegate Type Definition
**File**: `frb_codegen/src/library/codegen/ir/mir/ty/delegate.rs`

```rust
pub enum MirTypeDelegate {
    // ... existing variants
    MyExternal(MirTypeDelegateMyExternal),  // Add new variant
}

// Define struct for generic parameters
pub struct MirTypeDelegateMyExternal {
    pub inner: Box<MirType>,  // T parameter
    // Add other metadata as needed
}

// Implement required methods
impl MirTypeDelegate {
    fn get_delegate(&self) -> MirType {
        match self {
            MirTypeDelegate::MyExternal(_) => /* Define serialization format */,
        }
    }
    
    fn rust_api_type(&self) -> String {
        match self {
            MirTypeDelegate::MyExternal => "my_crate::MyExternal".to_owned(),
        }
    }
    
    fn dart_api_type(&self) -> String {
        match self {
            MirTypeDelegate::MyExternal => "MyExternal".to_owned(),
        }
    }
}
```

#### Step 2: Add Type Recognition
**File**: `frb_codegen/src/library/codegen/parser/mir/parser/ty/concrete.rs`

```rust
pub(crate) fn parse_type_path_data_concrete(&mut self, last_segment, splayed_segments) -> Result<Option<MirType>> {
    Ok(Some(match last_segment {
        // ... existing patterns
        ("MyExternal", [inner]) if check_prefix("my_crate") => {
            Delegate(MirTypeDelegate::MyExternal(MirTypeDelegateMyExternal {
                inner: Box::new(self.parse_type(inner)?),  // Recursively parse T
            }))
        },
        // ... rest
    }))
}
```

#### Step 3: Add API Dart Generation
**File**: `frb_codegen/src/library/codegen/generator/api_dart/spec_generator/info.rs`

```rust
fn dart_api_type(&self) -> String {
    match &self.mir {
        // ... existing cases
        MirTypeDelegate::MyExternal(mir) => format!(
            "MyExternal<{}>",
            ApiDartGenerator::new(*mir.inner.clone(), self.context).dart_api_type()  // Recursive generation
        ),
    }
}

fn dart_import(&self) -> Option<String> {
    match &self.mir {
        // ... existing cases
        MirTypeDelegate::MyExternal(_) => Some("import 'package:my_dart_package/my_dart_package.dart';".to_owned()),
    }
}
```

#### Step 4: Add Codec Implementation
**File**: `frb_codegen/src/library/codegen/generator/codec/sse/ty/delegate.rs`

```rust
fn generate_encode(&self, lang: &Lang) -> Option<String> {
    match lang {
        Lang::DartLang(_) => match &self.mir {
            // ... existing cases
            MirTypeDelegate::MyExternal(_) => "self.toSerializableFormat()".to_owned(),  // Dart encode
        },
        Lang::RustLang(_) => match &self.mir {
            // ... existing cases  
            MirTypeDelegate::MyExternal(_) => "self.to_serializable()".to_owned(),  // Rust encode
        },
    }
}

fn generate_decode(&self, lang: &Lang) -> Option<String> {
    match lang {
        Lang::DartLang(_) => match &self.mir {
            // ... existing cases
            MirTypeDelegate::MyExternal(_) => "MyExternal.fromSerializableFormat(inner)".to_owned(),  // Dart decode
        },
        Lang::RustLang(_) => match &self.mir {
            // ... existing cases
            MirTypeDelegate::MyExternal(_) => "my_crate::MyExternal::from_serializable(inner)".to_owned(),  // Rust decode
        },
    }
}
```

#### Step 5: Add Wire Protocol Support
**Files**: `generator/wire/*/decoder/ty/delegate.rs` and `encoder/ty/delegate.rs`

```rust
// In CST decoder
MirTypeDelegate::MyExternal(mir) => Acc::distribute(/* CST implementation */),

// In CST encoder  
MirTypeDelegate::MyExternal(mir) => Acc::distribute(/* CST implementation */),

// In DCO decoder
MirTypeDelegate::MyExternal(mir) => /* DCO implementation */,
```

#### Step 6: Add Feature Flag Support
**File**: `frb_rust/Cargo.toml`

```toml
[features]
my-crate = ["dep:my-crate", "allo-isolate/my-crate"]

[dependencies]
my-crate = { workspace = true, optional = true }
```

#### Step 7: Add Tests
Create test files following the pattern of `uuid_type.rs` in the examples.

### CRITICAL REQUIREMENTS:

1. **Serialization Format**: Decide how `MyExternal<T>` converts to/from bytes
2. **Error Handling**: All decode operations need proper error handling
3. **Recursive Types**: Generic parameters must be recursively processed
4. **Platform Compatibility**: Ensure serialization works across all platforms
5. **Performance**: Consider serialization overhead for the chosen format

## Step 3: Understanding Delegate Storage Structures

After the parser recognizes special types and extracts generic parameters, these parameters are stored in specialized delegate structures defined in `delegate.rs`:

### A. Delegate Enum Structure

The main `MirTypeDelegate` enum defines all special types that get special handling:

```rust
pub enum MirTypeDelegate {
    // External crate types (no parameters)
    Uuid,                           // External type: uuid::Uuid
    Backtrace,                      // External type: std::backtrace::Backtrace
    AnyhowException,                // External type: anyhow::Error
    
    // Generic types with parameters
    Map(MirTypeDelegateMap),        // HashMap<K, V>
    Set(MirTypeDelegateSet),        // HashSet<T>
    Array(MirTypeDelegateArray),    // [T; N]
    StreamSink(MirTypeDelegateStreamSink), // StreamSink<T, E>
    
    // Special parameter types
    Time(MirTypeDelegateTime),      // chrono types (enum variants)
    BigPrimitive(MirTypeDelegateBigPrimitive), // i128/u128
    // ... other types
}
```

### B. Generic Parameter Storage Examples

#### HashMap Storage:
```rust
pub struct MirTypeDelegateMap {
    pub key: Box<MirType>,          // K parameter recursively parsed
    pub value: Box<MirType>,        // V parameter recursively parsed
    pub hasher: Option<Box<MirType>>, // Optional hasher (usually None)
    pub element_delegate: MirTypeRecord, // For tuple conversion
}
```

#### Vec Storage (GeneralList):
```rust
pub struct MirTypeGeneralList {
    pub inner: Box<MirType>,        // T parameter recursively parsed
}
```

#### Set Storage:
```rust
pub struct MirTypeDelegateSet {
    pub inner: Box<MirType>,        // T parameter recursively parsed
    pub hasher: Option<Box<MirType>>, // Optional hasher
}
```

#### Array Storage:
```rust
pub struct MirTypeDelegateArray {
    pub length: usize,              // Array length from const generic
    pub mode: MirTypeDelegateArrayMode,
}

pub enum MirTypeDelegateArrayMode {
    General(Box<MirType>),          // T parameter for complex types
    Primitive(MirTypePrimitive),    // Optimized for primitive types
}
```

### C. Key Pattern: Recursive Parameter Storage

**Important**: All generic parameters are stored as `Box<MirType>`, which means:
1. **Recursive parsing**: Each parameter goes through the full parsing pipeline
2. **Nested generics work**: `Vec<HashMap<String, Vec<i32>>>` parses correctly
3. **Type safety**: Each parameter maintains its own type information

### D. External Type Pattern

External types like UUID follow a simple pattern:
- **Recognition**: Matched by exact string ("Uuid")
- **No parameters**: Just an enum variant with no data
- **Delegation**: All special behavior is in the code generation phase

This means adding a new external type requires:
1. Adding to the pattern matcher in `concrete.rs`
2. Adding an enum variant to `MirTypeDelegate`
3. Adding code generation logic for that variant

## Next Steps
The analysis shows the complete architecture for supporting both generic and external types in FRB. The key insight is that everything flows through the delegate pattern:
1. **Parser** recognizes types and extracts parameters
2. **Delegates** store parameters recursively as `Box<MirType>`
3. **Generators** convert delegates to target language code
4. **Codecs** handle serialization between languages

This architecture makes adding new types systematic: follow the existing patterns for similar types (external vs generic) and implement the required interfaces at each layer.

### IMPLEMENTATION COMPLEXITY:
- **Simple External Type** (like UUID): ~6 files to modify
- **Generic External Type**: ~10+ files to modify + more complex logic
- **Multi-parameter Generic**: Even more complex (see HashMap example)

This analysis shows that adding generic type support requires changes across the entire codebase stack, from parsing to code generation to protocol handling.

