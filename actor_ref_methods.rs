/*!
# ActorRef Methods for FRB Integration

This file contains method implementations for ActorRef<T> that will be auto-detected by FRB
and generated as methods on the opaque ActorRef type in Dart.

The key difference from flt_actor macro:
- flt_actor generated separate methods for each message type (inc(), dec(), etc.)
- This implementation provides a single generic tell() method that works with send_raw
*/

#[cfg(feature = "theta")]
use theta::actor_ref::ActorRef;

/// Get the unique identifier of this actor reference
/// This will be generated as: `Uuid get id => ...`
#[cfg(feature = "theta")]
pub fn actor_ref_id<T>(actor_ref: &ActorRef<T>) -> uuid::Uuid {
    actor_ref.id()
}

/// Send a message to the actor using send_raw
/// This is the single tell method that works with any message type
/// Unlike flt_actor which generates per-message methods, this is generic
#[cfg(feature = "theta")]
pub fn actor_ref_tell<T, M>(actor_ref: &ActorRef<T>, message: M) -> anyhow::Result<()> 
where
    M: serde::Serialize + Send + 'static,
{
    // Use send_raw to send any serializable message
    actor_ref.send_raw(message).map_err(|e| anyhow::anyhow!("Failed to send message: {}", e))
}

/// Connect to a remote actor by identifier or URL
/// This will be generated as a static method: `static Future<(ActorRef<T>, T)> prep(String identOrUrl) async`
#[cfg(feature = "theta")]
pub async fn actor_ref_prep<T>(ident_or_url: &str) -> anyhow::Result<(ActorRef<T>, T)>
where
    T: Clone + serde::de::DeserializeOwned + Send + Sync + 'static,
{
    // This is a placeholder - in real usage, this would use theta_frb functions
    // For now, just return an error indicating this needs proper implementation
    Err(anyhow::anyhow!("ActorRef::prep not yet implemented - needs theta_frb integration"))
}

/// Connect to a local actor by identifier  
/// This will be generated as a static method: `static (ActorRef<T>, T) prepLocal(String ident)`
#[cfg(feature = "theta")]
pub fn actor_ref_prep_local<T>(ident: &str) -> anyhow::Result<(ActorRef<T>, T)>
where
    T: Clone + serde::de::DeserializeOwned + Send + Sync + 'static,
{
    // This is a placeholder - in real usage, this would use theta_frb functions  
    // For now, just return an error indicating this needs proper implementation
    Err(anyhow::anyhow!("ActorRef::prepLocal not yet implemented - needs theta_frb integration"))
}

/// Initialize a stream for observing actor state changes
/// This would be used with FRB's StreamSink to provide reactive updates
#[cfg(feature = "theta")]
pub fn actor_ref_init_stream<T>(
    actor_ref: &mut ActorRef<T>, 
    sink: flutter_rust_bridge::StreamSink<T>
) -> anyhow::Result<()>
where
    T: Clone + serde::Serialize + Send + 'static,
{
    // This is a placeholder - in real usage, this would set up monitoring
    // and forward state updates to the StreamSink
    Err(anyhow::anyhow!("ActorRef::initStream not yet implemented - needs theta_frb integration"))
}
