/*!
# ActorRef Integration Library

This library demonstrates ActorRef<T> integration with Flutter Rust Bridge v2.
*/

pub mod actor_ref_methods;
pub mod actor_ref_test;

// Re-export for FRB
#[cfg(feature = "theta")]
pub use theta::actor_ref::ActorRef;

#[cfg(feature = "theta")]
pub use actor_ref_methods::*;

// Basic test function to verify FRB detection
use flutter_rust_bridge::frb;

#[frb(sync)]
pub fn hello_actor_ref() -> String {
    "ActorRef integration test".to_string()
}

#[cfg(feature = "theta")]
#[frb(sync)]
pub fn create_test_counter() -> String {
    "Would create CounterActor here with theta integration".to_string()
}

// Test ActorRef functions (would work with theta feature)
#[cfg(feature = "theta")]
#[frb(sync)]
pub fn test_actor_ref_id(actor_ref: ActorRef<CounterActor>) -> uuid::Uuid {
    actor_ref_id(&actor_ref)
}

#[cfg(feature = "theta")]
#[frb(sync)]
pub fn test_actor_ref_tell(actor_ref: ActorRef<CounterActor>, message: String) -> anyhow::Result<()> {
    actor_ref_tell(&actor_ref, message)
}

#[cfg(feature = "theta")]
#[frb(sync)]
pub fn test_actor_ref_encode(actor_ref: ActorRef<CounterActor>) -> (usize, i32) {
    actor_ref_sse_encode_raw(actor_ref)
}

// Test actor definition
#[cfg(feature = "theta")]
#[derive(Debug, Clone)]
pub struct CounterActor {
    pub count: i32,
}

#[cfg(feature = "theta")]
impl theta::actor::Actor for CounterActor {
    type Args = i32;
    
    fn name() -> &'static str {
        "CounterActor"
    }
}
