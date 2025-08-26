# ActorRef Integration: Opaque-Based Delegate Implementation

## Executive Summary

After analyzing the three investigation documents and understanding the limitations of UUID-based encoding, the correct approach is:

**ActorRef<T> should be a DELEGATE type that internally uses OPAQUE encoding/decoding but provides first-party Dart API integration.**

## Problem Analysis

### Why UUID Encoding Fails
```rust
// WRONG: This loses runtime state
"self.id().as_bytes().to_vec()"
// ActorRef contains:
// - Message sender channels
// - Actor system context  
// - Runtime handles
// These cannot be reconstructed from just a UUID
```

### The Opaque Solution
ActorRef should use the **same encoding as opaque types** (pointer/handle) but be presented as a **first-party delegate type** in the API.

## How Opaque Types Work

### Opaque Encoding Pattern
```rust
// Rust side - from rust_opaque.rs and rust_auto_opaque_explicit.rs
fn sse_encode_rust_opaque(self) -> usize {
    // Stores object in global map, returns handle
    flutter_rust_bridge::for_generated::rust_auto_opaque_explicit_encode(self)
}

fn sse_decode_rust_opaque(handle: usize) -> T {
    // Retrieves object from global map using handle
    flutter_rust_bridge::for_generated::rust_auto_opaque_explicit_decode(handle)
}
```

### Opaque Storage Infrastructure
```rust
// From frb_rust/src/misc/rust_auto_opaque.rs
static OPAQUE_MAP: LazyLock<RwLock<HashMap<usize, Box<dyn Any + Send + Sync>>>> = ...;

pub fn encode<T: 'static + Send + Sync>(obj: T) -> usize {
    let handle = generate_handle();
    OPAQUE_MAP.write().unwrap().insert(handle, Box::new(obj));
    handle
}

pub fn decode<T: 'static>(handle: usize) -> T {
    let obj = OPAQUE_MAP.write().unwrap().remove(&handle).unwrap();
    *obj.downcast::<T>().unwrap()
}
```

### Dart Opaque Pattern
```dart
// From generated code for opaque types
class RustOpaque<T> {
  final int handle;
  RustOpaque._(this.handle);
  
  // Automatic disposal when GC'd
  void dispose() => disposeRustOpaque(handle);
}
```

## ActorRef Implementation Strategy

### Core Architecture: Delegate + Opaque Hybrid

```rust
// 1. ActorRef is detected as DELEGATE type (first-party API)
MirTypeDelegate::ActorRef(MirTypeDelegateActorRef {
    inner: Box<MirType> // The actor message type T
})

// 2. But encoding/decoding uses OPAQUE infrastructure
// SSE Encode (Rust->Dart)
MirTypeDelegate::ActorRef(_) => {
    "flutter_rust_bridge::for_generated::rust_auto_opaque_explicit_encode(self)"
}

// SSE Decode (Dart->Rust)  
MirTypeDelegate::ActorRef(_) => {
    "flutter_rust_bridge::for_generated::rust_auto_opaque_explicit_decode(inner)"
}
```

### Dart ActorRef Class
```dart
// First-party ActorRef class with opaque handle internally
class ActorRef<T> {
  final int _handle; // Opaque handle to Rust ActorRef
  
  ActorRef._(this._handle);
  
  // Required by FRB delegate pattern
  int toOpaque() => _handle;
  static ActorRef<T> fromOpaque<T>(int handle) => ActorRef._(handle);
  
  // Business logic
  void tell(T message) {
    // Generated FFI call: _tellActorRef(_handle, message)
    _generatedTellMethod(_handle, message);
  }
  
  void dispose() => _disposeActorRef(_handle);
}
```

## Implementation Plan

### Phase 1: Opaque Infrastructure Integration
1. **Rust SSE Codec**: Use `rust_auto_opaque_explicit_encode/decode`
2. **Dart SSE Codec**: Use `toOpaque()` and `fromOpaque()` pattern
3. **CST/DCO Codecs**: Follow opaque integer handle pattern

### Phase 2: Dart Class Creation
1. **Create ActorRef<T> class** in `frb_dart/lib/src/actor_ref.dart`
2. **Implement required delegate methods**: `toOpaque()`, `fromOpaque()`
3. **Add tell() method stub** for FFI generation
4. **Add proper disposal** to prevent memory leaks

### Phase 3: Method Generation
1. **Generate tell() FFI function** that takes `(handle, message)`
2. **Implement Rust tell handler** that decodes handle to ActorRef
3. **Generate proper imports** for ActorRef class

## Detailed Implementation

### SSE Codec (Fixed)
```rust
// Rust encode: ActorRef -> opaque handle
MirTypeDelegate::ActorRef(_) => {
    "flutter_rust_bridge::for_generated::rust_auto_opaque_explicit_encode(self)".to_owned()
}

// Rust decode: opaque handle -> ActorRef
MirTypeDelegate::ActorRef(_) => {
    "flutter_rust_bridge::for_generated::rust_auto_opaque_explicit_decode(inner)".to_owned()
}

// Dart encode: ActorRef -> int
MirTypeDelegate::ActorRef(_) => {
    "self.toOpaque()".to_owned()
}

// Dart decode: int -> ActorRef  
MirTypeDelegate::ActorRef(_) => {
    "ActorRef.fromOpaque(inner)".to_owned()
}
```

### CST Codec Pattern
```rust
// Should follow the opaque integer pattern
MirTypeDelegate::ActorRef(_) => Acc::distribute(Some(format!(
    "return cst_encode_{}(raw.toOpaque());",
    "usize" // Opaque handles are usize
))),
```

### DCO Codec Pattern  
```rust
// Should reconstruct from opaque integer
MirTypeDelegate::ActorRef(_) => {
    "return ActorRef.fromOpaque(dco_decode_usize(raw));".to_owned()
}
```

## Benefits of This Approach

### 1. **Preserves Runtime State**
- ActorRef keeps all channels, context, handles
- No data loss during serialization
- Full actor functionality maintained

### 2. **First-Party API**
- `ActorRef<T>` appears as native Dart type
- Proper generic type support
- IDE autocomplete and type checking
- Generated `tell(T message)` method

### 3. **Reuses Proven Infrastructure**
- Leverages existing opaque storage system
- Memory management handled automatically
- Garbage collection integration
- Thread-safety inherited from opaque system

### 4. **Efficient Wire Protocol**
- Only sends handle (4-8 bytes) across FFI boundary
- No complex serialization overhead
- Fast encoding/decoding

## Migration Path

### Step 1: Fix Current Implementation
```rust
// Replace UUID-based encoding with opaque encoding
// File: codec/sse/ty/delegate.rs
MirTypeDelegate::ActorRef(_) => {
    "flutter_rust_bridge::for_generated::rust_auto_opaque_explicit_encode(self)".to_owned()
}
```

### Step 2: Create Dart Class
```dart
// File: frb_dart/lib/src/actor_ref.dart
class ActorRef<T> {
  final int _handle;
  ActorRef._(this._handle);
  
  static ActorRef<T> fromOpaque<T>(int handle) => ActorRef._(handle);
  int toOpaque() => _handle;
  
  void tell(T message) => _nativeTell(_handle, message);
}
```

### Step 3: Export and Test
```dart
// File: frb_dart/lib/flutter_rust_bridge.dart
export 'src/actor_ref.dart';
```

### Step 4: Generate tell() Method
- Modify method generation to create `_nativeTell(int handle, T message)`
- Implement Rust handler that decodes handle to ActorRef and calls tell()

## Technical Details

### Handle Lifecycle
1. **Creation**: ActorRef created in Rust, encoded to handle, sent to Dart
2. **Usage**: Dart calls `tell()` with handle + message, decoded back to ActorRef in Rust
3. **Disposal**: Dart GC triggers disposal, removes from opaque map

### Type Safety
- `ActorRef<T>` preserves message type `T` in Dart
- Generated `tell(T message)` ensures type safety
- Runtime type checking in generated FFI layer

### Memory Management
- Opaque map handles ownership
- Dart finalizers trigger cleanup
- No manual memory management required

## Conclusion

This opaque-based delegate approach provides:
- ✅ **Runtime state preservation** (no UUID reconstruction issues)
- ✅ **First-party Dart API** (delegate type benefits)
- ✅ **Efficient encoding** (opaque handle wire protocol)
- ✅ **Type safety** (generic ActorRef<T> support)
- ✅ **Proven infrastructure** (reuses opaque system)

The implementation reuses the robust opaque type infrastructure while providing the clean delegate API that makes ActorRef feel like a native Flutter Rust Bridge type.
