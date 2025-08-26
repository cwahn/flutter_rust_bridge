use std::hash::{Hash, Hasher};

use flutter_rust_bridge::frb;
use iroh::Endpoint;
use rustc_hash::FxHasher;
use serde::{de, Deserialize, Serialize};
use theta::prelude::*;

use tracing::{debug, error, field::debug, info, trace, warn};
use tracing_subscriber::fmt::time::ChronoLocal;

// crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER.async_runtime().0

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();

    crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER
        .async_runtime()
        .0
        .block_on(async {
            tracing_subscriber::fmt()
                // .with_max_level(tracing::Level::TRACE)
                .with_env_filter("info,theta=trace,theta_frb_dev=trace")
                .with_timer(ChronoLocal::new("%y%m%d %H:%M:%S%.3f %Z".into()))
                .try_init();

            let endpoint = Endpoint::builder()
                .alpns(vec![b"theta".to_vec()])
                .discovery_n0()
                .bind()
                .await
                .expect("Endpoint binding should succeed");

            let root_ctx = RootContext::init(endpoint);
            debug!("Root context initialized");

            let counter = root_ctx.spawn(Counter::new());
            root_ctx.bind(b"counter", counter);
        });
}

#[derive(Debug, Clone, Hash, ActorArgs, Serialize, Deserialize)]
pub struct Counter {
    // ! Fields should be public or implement accessor
    pub value: i64,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterView {
    pub value: i64,
}

impl From<&Counter> for CounterView {
    fn from(counter: &Counter) -> Self {
        Self {
            value: counter.value,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inc {
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dec {
    pub amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterResponse {
    pub new_value: i64,
}

#[actor("a1b2c3d4-5e6f-7890-abcd-ef1234567890")]
impl Actor for Counter {
    type View = CounterView;

    const _: () = {
        async |msg: Inc| -> CounterResponse {
            let new_value = self.value + msg.amount;
            self.value = new_value;
            info!("Counter incremented by {} to {}", msg.amount, new_value);
            CounterResponse { new_value }
        };

        async |msg: Dec| -> CounterResponse {
            let new_value = self.value - msg.amount;
            self.value = new_value;
            info!("Counter decremented by {} to {}", msg.amount, new_value);
            CounterResponse { new_value }
        };
    };

    fn hash_code(&self) -> u64 {
        let mut hasher = FxHasher::default();
        Hash::hash(self, &mut hasher);
        hasher.finish()
    }
}

/// Get a reference to the counter actor
/// This demonstrates our new ActorRef delegate integration
#[frb]
pub async fn get_counter() -> anyhow::Result<theta::actor_ref::ActorRef<Counter>> {
    // In a real app, you'd have a way to store and retrieve the root context
    // For this test, we'll create a new one (this is just for demonstration)
    let endpoint = Endpoint::builder()
        .alpns(vec![b"theta".to_vec()])
        .discovery_n0()
        .bind()
        .await?;

    let root_ctx = RootContext::init(endpoint);
    let counter = root_ctx.spawn(Counter::new());
    Ok(counter)
}

/// Increment the counter via ActorRef
#[frb]
pub fn increment_counter(actor_ref: theta::actor_ref::ActorRef<Counter>, amount: i64) -> anyhow::Result<()> {
    actor_ref.tell(Inc { amount })?;
    Ok(())
}

/// Decrement the counter via ActorRef  
#[frb]
pub fn decrement_counter(actor_ref: theta::actor_ref::ActorRef<Counter>, amount: i64) -> anyhow::Result<()> {
    actor_ref.tell(Dec { amount })?;
    Ok(())
}

// #[frb(
//     opaque,
//     dart_code = r#"
// static final streamRegistry = <UuidValue, CachedStream<CounterView>>{};

// CachedStream<CounterView>? cachedStream_ = null;

// static Future<CounterRef> connect(String identOrUrl) async {
//     final (inst, initState) = await CounterRef.newInstance(identOrUrl);
//     // final rawStream = inst.initStream().asBroadcastStream();
//     final rawStream = inst.initStream();

//     // final initState = await rawStream.first;
//     final cachedStream = rawStream.cached(initState).asBroadcastStream();

//     streamRegistry[inst.id] = cachedStream;
//     inst.cachedStream_ = cachedStream;

//     return inst;
// }

// static CounterRef connectLocal(String ident) {
//     final (inst, initState) = CounterRef.newLocal(ident);
//     // final rawStream = inst.initStream().asBroadcastStream();
//     final rawStream = inst.initStream();

//     // final initState = await rawStream.first;
//     final cachedStream = rawStream.cached(initState).asBroadcastStream();

//     streamRegistry[inst.id] = cachedStream;
//     inst.cachedStream_ = cachedStream;

//     return inst;
// }

// CachedStream<CounterView> get stream => cachedStream_!;

// CounterView get state => cachedStream_!.snapshot;
//     "#
// )]
// pub struct CounterRef(
//     ::theta::actor_ref::ActorRef<Counter>,
//     Option<theta_flume::Receiver<Update<Counter>>>,
// );

// #[frb(unignore)]
// impl CounterRef {
//     #[::flutter_rust_bridge::frb(sync, getter)]
//     pub fn id(&self) -> Uuid {
//         self.0.id()
//     }

//     #[frb(positional)]
//     pub async fn new(ident_or_url: &str) -> anyhow::Result<(CounterRef, CounterView)> {
//         let actor = get_root_ctx().lookup(ident_or_url).await?;

//         let (tx, rx) = unbounded_anonymous();

//         if let Err(_e) = monitor(
//             actor
//                 .id()
//                 .as_simple()
//                 .encode_lower(&mut Uuid::encode_buffer()),
//             tx,
//         )
//         .await
//         {
//             error!("Failed to monitor counter: {_e}");
//         }

//         let Some(Update::State(init_state)) = rx.recv().await else {
//             return Err(anyhow::anyhow!(
//                 "Failed to receive initial state from counter"
//             ));
//         };

//         Ok((CounterRef(actor, Some(rx)), init_state))
//     }

//     #[frb(sync, positional)]
//     pub fn new_local(ident: &str) -> anyhow::Result<(CounterRef, CounterView)> {
//         let actor = get_root_ctx().lookup_local(ident)?;

//         let (tx, rx) = unbounded_anonymous();

//         if let Err(_e) = monitor_local_id(actor.id(), tx) {
//             error!("Failed to monitor counter: {_e}");
//         }

//         let Some(Update::State(init_state)) = rx.recv_blocking() else {
//             return Err(anyhow::anyhow!(
//                 "Failed to receive initial state from counter"
//             ));
//         };

//         Ok((CounterRef(actor, Some(rx)), init_state))
//     }

//     pub fn init_stream(
//         &mut self,
//         sink: crate::frb_generated::StreamSink<CounterView>,
//     ) -> anyhow::Result<()> {
//         let rx = self
//             .1
//             .take()
//             .expect("Stream could be initialized only once");

//         crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER
//             .async_runtime()
//             .0
//             .spawn(async move {
//                 loop {
//                     let Some(update) = rx.recv().await else {
//                         break info!("Observation stream closed");
//                     };

//                     if let Update::State(state) = update {
//                         if let Err(e) = sink.add(state.into()) {
//                             break warn!("Frb monitoring channel closed: {e}");
//                         }
//                     }
//                 }
//             });

//         Ok(())
//     }

//     #[frb(sync)]
//     pub fn inc(&self, msg: Inc) -> anyhow::Result<()> {
//         Ok(self.0.tell(msg)?)
//     }

//     #[frb(sync)]
//     pub fn dec(&self, msg: Dec) -> anyhow::Result<()> {
//         Ok(self.0.tell(msg)?)
//     }
// }
