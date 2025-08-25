# ActorRef<T> Integration into FRB - Implementation Journal

## Goal
Integrate `theta::actor_ref::ActorRef<T>` as a native FRB type (similar to Vec, HashMap, UUID) instead of relying on the external `flt_actor` macro.

## Key Requirements
1. Add "theta" feature flag support
2. Generate code similar to `flt_actor` macro but with key differences:
   - Single `tell` method instead of one method per message type
   - `tell` method should call `send_raw` with `Nil` continuation
   - No generic function support in Dart (FRB limitation)

## Implementation Plan
Following the pattern documented in our FRB analysis:
1. **Step 1**: Add delegate type definition for ActorRef<T>
2. **Step 2**: Add type recognition in concrete parser  
3. **Step 3**: Add API Dart generation
4. **Step 4**: Add codec implementation (serialization)
5. **Step 5**: Add feature flag support
6. **Step 6**: Add tests

## Progress Log

### Step 1: Add Delegate Type Definition
**Status**: ✅ COMPLETED
**Goal**: Add `ActorRef(MirTypeDelegateActorRef)` to `MirTypeDelegate` enum

**Actions Taken**:
1. Added `ActorRef(MirTypeDelegateActorRef)` variant to `MirTypeDelegate` enum
2. Added `MirTypeDelegateActorRef` struct with `inner: Box<MirType>` field  
3. Implemented required methods:
   - `safe_ident()`: Returns `"ActorRef_{inner_type}"`
   - `rust_api_type()`: Returns `"ActorRef<{inner_type}>"`
   - `get_delegate()`: Returns `MirType::Delegate(MirTypeDelegate::String)` (serializes as String)
   - `visit_children_types()`: Visits the inner actor type

**Files Modified**:
- `frb_codegen/src/library/codegen/ir/mir/ty/delegate.rs`

**Next Action**: Add type recognition in concrete parser

---

### Step 2: Add Type Recognition in Concrete Parser  
**Status**: ✅ COMPLETED
**Goal**: Add pattern matching for `theta::ActorRef<T>` in concrete type parser

**Actions Taken**:
1. Added pattern `("ActorRef", [inner]) if check_prefix("theta")` to concrete parser
2. Pattern recursively parses the inner actor type with `self.parse_type(inner)?`  
3. Creates `MirTypeDelegateActorRef` delegate with parsed inner type
4. Added `MirTypeDelegateActorRef` to imports

**Files Modified**:
- `frb_codegen/src/library/codegen/parser/mir/parser/ty/concrete.rs`

**Pattern Added**:
```rust
("ActorRef", [inner]) if check_prefix("theta") => {
    Delegate(MirTypeDelegate::ActorRef(MirTypeDelegateActorRef {
        inner: Box::new(self.parse_type(inner)?),
    }))
},
```

**Next Action**: Add API Dart generation

---

### Step 3: Add API Dart Generation
**Status**: ✅ COMPLETED  
**Goal**: Generate proper Dart API types and imports for ActorRef<T>

**Actions Taken**:
1. Added `dart_api_type()` generation that returns `"ActorRef<{inner_type}>"`
2. Recursively generates inner actor type using `ApiDartGenerator::new(*mir.inner.clone(), self.context).dart_api_type()`
3. Added `dart_import()` support to import `'package:theta/theta.dart'`

**Files Modified**:
- `frb_codegen/src/library/codegen/generator/api_dart/spec_generator/info.rs`

**Code Added**:
```rust
// In dart_api_type() match:
MirTypeDelegate::ActorRef(mir) => format!(
    "ActorRef<{}>",
    ApiDartGenerator::new(*mir.inner.clone(), self.context).dart_api_type(),
),

// In dart_import() match:  
MirTypeDelegate::ActorRef(_) => {
    Some("import 'package:theta/theta.dart';".to_owned())
}
```

**Next Action**: Add codec implementation (serialization)

---

### Step 4: Add Codec Implementation (Serialization)
**Status**: ✅ COMPLETED
**Goal**: Implement 4-way serialization for ActorRef<T> (Dart encode/decode, Rust encode/decode)

**Actions Taken**:
1. **Dart Encode**: `"self.ident()"` - Get actor identifier as string
2. **Rust Encode**: `"self.ident().to_string()"` - Convert actor identifier to string
3. **Dart Decode**: `"ActorRef.fromIdent(inner)"` - Create ActorRef from identifier string  
4. **Rust Decode**: `"theta_frb::root_ctx().lookup_local(&inner).expect("fail to lookup actor")"` - Lookup actor by identifier

**Files Modified**:
- `frb_codegen/src/library/codegen/generator/codec/sse/ty/delegate.rs`

**Serialization Strategy**:
- ActorRef serializes to/from String (actor identifier)
- Uses theta framework's identifier system for actor lookup
- Local lookup on Rust side, identifier-based construction on Dart side

**Next Action**: Add feature flag support

---

### Step 5: Add Feature Flag Support
**Status**: ✅ COMPLETED
**Goal**: Add "theta" feature flag to optionally enable ActorRef support

**Actions Taken**:
1. Added `theta = { version = "0.1.0", optional = true }` to dependencies
2. Added `theta = ["dep:theta"]` to features section
3. Feature follows the same pattern as uuid, chrono, etc.

**Files Modified**:
- `frb_rust/Cargo.toml`

**Usage**:
Users can enable ActorRef support by adding to their Cargo.toml:
```toml
[dependencies]
flutter_rust_bridge = { version = "...", features = ["theta"] }
```

**Next Action**: Add tests and verify the integration

## Key Insights from flt_actor Review

After reviewing the `flt_actor` macro implementation, I discovered that my initial approach was incorrect. The `flt_actor` macro doesn't make `ActorRef<T>` a delegate type, but rather:

### Important Clarification from User

**The macro was a workaround when access to FRB source code wasn't available.** Now that we have source access, we should:

1. **Keep the APIs** from flt_actor (the methods and functionality)
2. **Don't exactly follow the implementation** - we can do better with native FRB integration
3. **Fix the type naming issue**: 
   - flt_actor generated `{SomeActor}Ref` (e.g., `CounterRef`) which was not good
   - We want `ActorRef<SomeActor>` to translate to `ActorRef` in Dart (like `Vec<T>` -> `List<T>`)
   - This should be a proper generic type in Dart

### CRITICAL REALIZATION: ActorRef<T> IS a Delegate Type

**ActorRef<T> should be a delegate type just like Uuid, Vec, StreamSink.** The flt_actor macro was a **second-party workaround** implementation when we didn't have access to FRB source code. Now we're implementing the **first-party native integration** as a proper delegate type.

### Correct Approach

ActorRef<T> should be implemented as:
1. **A delegate type** in the same pattern as Uuid, Vec, StreamSink, etc.
2. **With proper generic type naming** in Dart: `ActorRef<T>` not `TRef`
3. **Single `tell()` method** that works with `send_raw` for any message type
4. **Same API surface** as flt_actor but implemented natively in FRB

### Current flt_actor macro generates:

## Revised Implementation Strategy

Instead of making `ActorRef<T>` a delegate type, I should:
1. Make `ActorRef<T>` an **opaque type** in FRB
2. Generate methods on the opaque type similar to `flt_actor`
3. Handle the wire protocol for method serialization
4. Add the theta feature flag support

## Current Status: Compilation Errors
Need to fix missing wire protocol implementations for the delegate approach first, then pivot to the opaque approach.

---

### Current flt_actor macro generates:
```rust
#[frb(opaque)]
pub struct CounterRef(
    ActorRef<Counter>,
    Option<Receiver<Update<Counter>>>
);

impl CounterRef {
    pub async fn prep(ident_or_url: &str) -> Result<(CounterRef, CounterView)>;
    pub fn prep_local(ident: &str) -> Result<(CounterRef, CounterView)>;
    pub fn init_stream(&mut self, sink: StreamSink<CounterView>) -> Result<()>;
    
    // One method per message type
    pub fn inc(&self, msg: Inc) -> Result<()>;
    pub fn dec(&self, msg: Dec) -> Result<()>;
}
```

### Target FRB integration should generate:
```rust
// Generated automatically when ActorRef<T> is detected
impl ActorRefFrbExt<T> {
    pub async fn prep(ident_or_url: &str) -> Result<(ActorRef<T>, T::View)>;
    pub fn prep_local(ident: &str) -> Result<(ActorRef<T>, T::View)>;
    pub fn init_stream(&mut self, sink: StreamSink<T::View>) -> Result<()>;
    
    // Single generic tell method
    pub fn tell(&self, msg: T::Msg) -> Result<()> {
        self.send_raw(msg, Continuation::Nil)
    }
}
```

---
