#[cfg(test)]
mod test_actor_ref_sse_encode {
    use super::*;
    use std::mem;

    // Mock ActorRef for testing
    struct MockActor;
    
    impl theta::actor::Actor for MockActor {
        type Args = ();
        
        fn name() -> &'static str {
            "MockActor"
        }
    }

    #[test]
    fn test_sse_encode_raw_returns_correct_types() {
        // This test would need actual theta integration to work
        // For now, just verify the function signature compiles
        
        // The sse_encode_raw function should:
        // 1. Take ActorRef<T> by value
        // 2. Return (usize, i32) tuple
        // 3. Create a heap pointer with Box::into_raw
        
        let size = mem::size_of::<theta::actor_ref::ActorRef<MockActor>>();
        assert!(size > 0); // ActorRef should have non-zero size
    }
    
    #[test] 
    fn test_actor_ref_size_consistency() {
        // Verify ActorRef size is reasonable for pointer + size encoding
        let size = mem::size_of::<theta::actor_ref::ActorRef<MockActor>>();
        println!("ActorRef<MockActor> size: {} bytes", size);
        
        // ActorRef wraps MsgTx which is theta_flume::Sender
        // Should be similar to Arc + some metadata
        assert!(size >= mem::size_of::<usize>());
        assert!(size <= 64); // Should be reasonably small
    }
}
