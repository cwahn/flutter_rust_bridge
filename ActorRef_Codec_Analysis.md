# ActorRef Codec Architecture Analysis

## Investigation Summary

After extensively investigating UUID, StreamSink, and other delegate implementations in Flutter Rust Bridge, I've identified the exact responsibilities of each codec and the precise implementation options for ActorRef<T>. The key insight is that FRB has TWO distinct patterns:

### Pattern 1: Delegate Types (UUID, String, StreamSink, etc.)
- **Purpose**: Native types with specialized encoding/decoding logic
- **Wire Protocol**: Uses underlying primitive types (Vec<u8> for UUID, String for big integers, etc.)
- **Dart Class**: Either external dependency (UUID uses `package:uuid`) or built-in (String, etc.)
- **Codec Functions**: Auto-generated `sse_encode_Uuid`, `dco_decode_Uuid`, `cst_encode_Uuid`

### Pattern 2: Opaque Types (Custom structs, etc.)
- **Purpose**: User-defined types treated as opaque handles
- **Wire Protocol**: Opaque pointer/reference across language boundary
- **Methods**: Generated method calls for struct methods
- **Codec Functions**: Opaque pointer management, no semantic encoding

## Codec Responsibilities Deep Dive

### SSE (Simple Serialization) Codec
**Responsibility**: Serialize to byte buffer, deserialize from byte buffer

**UUID Implementation**:
```rust
// Encode: UUID -> Vec<u8>
MirTypeDelegate::Uuid => "self.as_bytes().to_vec()".to_owned(),

// Decode: Vec<u8> -> UUID  
MirTypeDelegate::Uuid => {
    r#"uuid::Uuid::from_slice(&inner).expect("fail to decode uuid")"#.to_owned()
}
```

**Dart Side**:
```rust
// Encode: UuidValue -> Uint8List
MirTypeDelegate::Uuid => "self.toBytes()".to_owned(),

// Decode: Uint8List -> UuidValue
MirTypeDelegate::Uuid => "UuidValue.fromByteList(inner)".to_owned(),
```

### CST (C-Struct) Codec  
**Responsibility**: Dart->Rust encoding via C structs

**UUID Implementation**:
```rust
// Dart encoder calls underlying list encoder
MirTypeDelegate::Uuid => Acc::distribute(Some(format!(
    "return cst_encode_{}(raw.toBytes());",
    uint8list_safe_ident(true)  // "list_prim_u_8_strict"
))),
```

**Generated Function**:
```dart
ffi.Pointer<wire_cst_list_prim_u_8_strict> cst_encode_Uuid(UuidValue raw) {
    return cst_encode_list_prim_u_8_strict(raw.toBytes());
}
```

### DCO (Dynamic Cast Object) Codec
**Responsibility**: Rust->Dart decoding via dynamic casting

**UUID Implementation**:
```rust
// Decoder reconstructs from primitive
MirTypeDelegate::Uuid => {
    "return UuidValue.fromByteList(dco_decode_list_prim_u_8_strict(raw));".to_owned()
}
```

## Core Architecture Insight

**The fundamental pattern is**: Delegate types have specialized wire encodings but use standard Dart classes that must provide the required methods:

1. **Wire Encoding**: UUID uses `Vec<u8>` on the wire (not a custom wire type)
2. **Dart Class**: Uses external `UuidValue` from `package:uuid`
3. **Required Methods**: `.toBytes()` for encoding, `.fromByteList()` for decoding
4. **Codec Generation**: Auto-generates functions that call these methods

## Implementation Options for ActorRef<T>

### Option 1: Delegate Type with UUID Wire Protocol
```rust
// ActorRef<T> encodes as its underlying UUID
// Wire protocol: Vec<u8> (16 bytes)
// Dart class: Custom ActorRef<T> class

// SSE encode/decode
MirTypeDelegate::ActorRef(_) => "self.id().as_bytes().to_vec()".to_owned(),
MirTypeDelegate::ActorRef(_) => "theta::actor_ref::ActorRef::from_id(uuid::Uuid::from_slice(&inner).expect(\"fail to decode uuid\"))".to_owned(),

// CST encode
"return cst_encode_list_prim_u_8_strict(raw.id.toBytes());"

// DCO decode
"return ActorRef.fromId(UuidValue.fromByteList(dco_decode_list_prim_u_8_strict(raw)));"
```

**Pros:**
- Clean wire protocol (just 16 bytes)
- Follows exact UUID pattern
- Efficient encoding
- First-party integration

**Cons:**
- Requires creating Dart `ActorRef<T>` class
- Need to implement `.tell()` method in Dart
- Must handle generic type parameter `<T>`

### Option 2: Delegate Type with String Wire Protocol
```rust
// ActorRef<T> encodes as UUID string representation
// Wire protocol: String (36 chars: "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx")
// Dart class: Custom ActorRef<T> class

// SSE encode/decode
MirTypeDelegate::ActorRef(_) => "self.id().to_string()".to_owned(),
MirTypeDelegate::ActorRef(_) => "theta::actor_ref::ActorRef::from_id(uuid::Uuid::parse_str(&inner).expect(\"fail to parse uuid\"))".to_owned(),

// CST encode
"return cst_encode_String(raw.id.toString());"

// DCO decode
"return ActorRef.fromId(UuidValue.fromString(raw));"
```

**Pros:**
- Human readable on wire
- Simple string-based protocol
- Easier debugging

**Cons:**
- Less efficient (36 bytes vs 16 bytes)
- String parsing overhead
- Still requires Dart class creation

### Option 3: Opaque Type with Method Generation
```rust
// ActorRef<T> as opaque type
// Wire protocol: Opaque pointer
// Methods: Generated .tell() method calls

// No delegate implementation needed
// FRB generates method stubs that call Rust impl
```

**Pros:**
- Zero Dart infrastructure needed
- Automatic method generation
- Type-safe generic handling
- No codec complexity

**Cons:**
- Opaque pointers (memory overhead)
- Not a true "native" type
- More complex FFI boundary

## Recommendation: Option 1 (Delegate with UUID Wire Protocol)

**Rationale**: This provides the exact same functionality as `flt_actor` but as a first-party FRB delegate type. The wire protocol is identical to UUID (16 bytes), and the Dart side would look like:

```dart
// Generated ActorRef class (similar to UuidValue)
class ActorRef<T> {
  final UuidValue id;
  
  const ActorRef._(this.id);
  
  // Required by FRB delegate pattern
  Uint8List toBytes() => id.toBytes();
  static ActorRef<T> fromByteList<T>(Uint8List bytes) => 
    ActorRef._(UuidValue.fromByteList(bytes));
  
  // Business logic  
  void tell(T message) {
    // Call generated FFI function with this.id and message
  }
}
```

**Missing Infrastructure Needed**:

1. **Dart ActorRef Class**: Create in `frb_dart/lib/src/` with required methods
2. **Tell Method Generation**: Add method generation for `.tell()` that calls Rust
3. **Generic Type Handling**: Ensure `ActorRef<T>` preserves type parameter
4. **Import Generation**: Auto-generate import for ActorRef in generated files

This approach preserves all macro functionality while providing first-party integration with proper wire protocol efficiency.
