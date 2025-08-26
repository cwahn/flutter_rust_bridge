use crate::actor_ref_methods::*;
use flutter_rust_bridge::frb;
use theta::actor::ActorRef;

// Test struct for code generation
#[derive(Clone, Debug)]
pub struct CounterActor {
    pub count: i32,
}

// Test functions for FRB code generation
#[frb(sync)]
pub fn create_counter_actor() -> ActorRef<CounterActor> {
    todo!("Implementation would create an ActorRef")
}

#[frb(sync)]
pub fn get_actor_id(actor: &ActorRef<CounterActor>) -> i32 {
    actor_ref_id(actor)
}

#[frb(sync)]
pub fn send_message(actor: &ActorRef<CounterActor>, message: CounterActor) {
    actor_ref_tell(actor, message);
}

#[frb(sync)]
pub fn encode_actor_for_dart(actor: ActorRef<CounterActor>) -> (usize, i32) {
    actor_ref_sse_encode_raw(actor)
}
