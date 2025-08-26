/// ActorRef<T> represents a reference to an actor that can receive messages of type T.
/// This is the Dart representation of theta::actor_ref::ActorRef<T>.
/// 
/// Internally uses an opaque handle to preserve Rust runtime state.
class ActorRef<T> {
  /// The opaque handle to the Rust ActorRef (preserves runtime state)
  final int _handle;

  /// Create an ActorRef with the given opaque handle
  const ActorRef._(this._handle);

  /// Create an ActorRef from an opaque handle (required by FRB delegate pattern)
  factory ActorRef.fromOpaque(int handle) => ActorRef._(handle);

  /// Create an ActorRef from opaque handle (used by SSE decoder)
  factory ActorRef.fromOpaqueHandle(int handle) => ActorRef._(handle);

  /// Get the opaque handle (required by SSE encoder)
  int get frbOpaqueHandle => _handle;

  /// Convert to opaque handle (required by FRB delegate pattern)
  int toOpaque() => _handle;

  /// Send a message to this actor (fire-and-forget)
  /// This method will be implemented by generated FFI code
  void tell(T message) {
    throw UnimplementedError('tell() method should be implemented by generated FFI code');
  }

  /// Dispose this ActorRef and free associated resources
  void dispose() {
    // This will be implemented by generated FFI code
    throw UnimplementedError('dispose() method should be implemented by generated FFI code');
  }

  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is ActorRef<T> && other._handle == _handle;
  }

  @override
  int get hashCode => _handle.hashCode;

  @override
  String toString() => 'ActorRef<$T>(handle: $_handle)';
}

/// Internal implementation class for ActorRef.
/// This class contains the codec implementations that are used by FRB.
class ActorRefImpl<T> extends ActorRef<T> {
  ActorRefImpl._(super.handle) : super._();

  /// Internal CST encoder implementation.
  /// Uses opaque encoding to preserve Rust runtime state.
  dynamic frbInternalCstEncode() {
    // Encode as opaque integer handle
    return _handle;
  }

  /// Internal DCO decoder implementation.
  /// Reconstructs ActorRef from opaque handle.
  static ActorRefImpl<T> frbInternalDcoDecode<T>(List<dynamic> raw) {
    // Decode opaque integer handle from first element
    final handle = raw[0] as int;
    return ActorRefImpl<T>._(handle);
  }

  /// Internal SSE encoder implementation.
  /// Uses opaque encoding like RustOpaque types.
  int frbInternalSseEncode({required bool move}) {
    // Encode as opaque integer handle
    return _handle;
  }

  /// Internal SSE decoder implementation.
  /// Reconstructs ActorRef from opaque handle.
  static ActorRefImpl<T> frbInternalSseDecode<T>(int ptr, int size) {
    // Decode opaque integer handle (ptr), ignore size for ActorRef
    return ActorRefImpl<T>._(ptr);
  }
}
