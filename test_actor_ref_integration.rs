/*!
# Test ActorRef<T> FRB Integration

This file tests if FRB can properly detect and generate methods for ActorRef<T> 
using the opaque type system with sse_encode_raw().
*/

use flutter_rust_bridge::frb;

// Include our method implementations
mod actor_ref_methods;
use actor_ref_methods::*;

#[cfg(feature = "theta")]
use theta::{actor::Actor, actor_ref::ActorRef};

// Define a simple test actor for integration
#[cfg(feature = "theta")]
#[derive(Debug, Clone)]
pub struct CounterActor {
    pub count: i32,
}

#[cfg(feature = "theta")]
impl Actor for CounterActor {
    type Args = i32;
    
    fn name() -> &'static str {
        "CounterActor"
    }
}

// Test function that should be detected by FRB
#[frb(sync)]
pub fn create_test_actor_ref() -> anyhow::Result<String> {
    Ok("ActorRef creation would happen here with theta integration".to_string())
}

// Test function that uses ActorRef (would work once we have theta integration)
#[cfg(feature = "theta")]
#[frb(sync)]
pub fn test_actor_ref_operations(actor_ref: ActorRef<CounterActor>) -> anyhow::Result<String> {
    let id = actor_ref_id(&actor_ref);
    Ok(format!("ActorRef ID: {}", id))
}

// Test function for sse_encode_raw
#[cfg(feature = "theta")]
#[frb(sync)]
pub fn test_sse_encode_raw(actor_ref: ActorRef<CounterActor>) -> (usize, i32) {
    actor_ref_sse_encode_raw(actor_ref)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let result = create_test_actor_ref();
        assert!(result.is_ok());
    }
}
