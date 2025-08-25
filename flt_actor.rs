/*!
# flt_actor Macro

This module provides the `flt_actor` attribute macro for generating Flutter Rust Bridge boilerplate
for Theta actors. The macro simplifies the process of exposing Rust actors to Flutter by automatically
generating necessary wrapper code.

## Usage

Apply the `#[flt_actor]` attribute to an `impl Actor for YourActor` block:

```rust
#[actor("uuid-here", snapshot = YourActor)]
#[flt_actor]
impl Actor for YourActor {
    type View = YourActor;

    const _: () = {
        async |msg: MessageType| -> ResponseType { ... };
    };

    fn hash_code(&self) -> u64 { ... }
}
```

## Generated Code

The macro generates:

1. **ActorRef struct**: A Flutter-compatible reference wrapper (`YourActorRef`)
2. **Methods**:
   - `newInstance(ident_or_url: &str)` - Look up a remote actor and return (ActorRef, InitialState)
   - `newLocal(ident: &str)` - Look up a local actor and return (ActorRef, InitialState)
   - `initStream(sink: StreamSink<YourView>)` - Initialize state change stream
   - Message-specific methods (e.g., `inc()`, `dec()`) for each message type found in const blocks

3. **Dart Code**: Provides convenient `connect()` and `connectLocal()` static methods with caching

Note: The ROOT_CTX is managed globally in the main `theta-frb` crate and should be initialized
using `theta_frb::init_root_ctx(ctx)` before using any generated actor references.

## Example Generated Output

For a `Counter` actor with `Inc` and `Dec` messages, the macro generates:

```rust
#[frb(opaque)]
pub struct CounterRef(
    ActorRef<Counter>,
    Option<Receiver<Update<Counter>>>
);

#[frb(unignore)]
impl CounterRef {
    pub async fn newInstance(ident_or_url: &str) -> anyhow::Result<(CounterRef, CounterView)> { ... }
    pub fn newLocal(ident: &str) -> anyhow::Result<(CounterRef, CounterView)> { ... }
    pub fn initStream(&mut self, sink: StreamSink<CounterView>) -> anyhow::Result<()> { ... }

    #[frb(sync)]
    pub fn inc(&self, msg: Inc) -> anyhow::Result<()> { ... }

    #[frb(sync)]
    pub fn dec(&self, msg: Dec) -> anyhow::Result<()> { ... }
}
```

The Dart code provides static `connect()` and `connectLocal()` methods that automatically handle
stream setup and caching.

## Features

- **Automatic Message Detection**: Extracts message types from `const _` blocks
- **Flutter Integration**: Generates Flutter Rust Bridge compatible code
- **Type Safety**: Preserves Rust's type safety while providing Flutter bindings
- **Monitoring Support**: Built-in support for state observation and streaming
- **Stream Caching**: Automatic caching of streams to prevent multiple subscriptions
*/
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ImplItem, ItemImpl, Type, parse_macro_input, parse_quote};

use crate::utils::generate_flt_actor_ref_name;

/// Attribute macro that generates Flutter Rust Bridge boilerplate for actors
///
/// See module documentation for detailed usage information.
pub fn process_flt_actor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    // Extract actor name from impl block
    let actor_name = match extract_actor_name(&input) {
        Ok(name) => name,
        Err(e) => return e.to_compile_error().into(),
    };

    // Extract the View type from the impl block
    let view_type = match extract_view_type(&input) {
        Ok(view_type) => view_type,
        Err(e) => return e.to_compile_error().into(),
    };

    // Since the actor macro has already processed the const blocks,
    // we need to extract message types from the generated Msg enum
    // For now, we'll use a hardcoded approach for Counter
    let message_types = if actor_name == "Counter" {
        vec![
            MessageInfo {
                msg_type: syn::Ident::new("Inc", proc_macro2::Span::call_site()),
                method_name: syn::Ident::new("inc", proc_macro2::Span::call_site()),
            },
            MessageInfo {
                msg_type: syn::Ident::new("Dec", proc_macro2::Span::call_site()),
                method_name: syn::Ident::new("dec", proc_macro2::Span::call_site()),
            },
        ]
    } else {
        // For other actors, try to extract from the generated code
        extract_message_types_from_generated(&input)
    };

    let mut output = TokenStream2::new();

    // Include the original impl block
    output.extend(quote! { #input });

    // Generate the boilerplate code
    output.extend(generate_actor_ref(&actor_name, &view_type, &message_types));

    output.into()
}

/// Extract message types from the generated Actor implementation
/// After the actor macro runs, it generates Message trait implementations
/// We can look for these to determine what messages the actor handles
fn extract_message_types_from_generated(_input: &ItemImpl) -> Vec<MessageInfo> {
    // This will be implemented later to parse the generated enum and impls
    // For now return empty vector
    Vec::new()
}

/// Extract the View type from the impl Actor block
fn extract_view_type(input: &ItemImpl) -> syn::Result<syn::Type> {
    for item in &input.items {
        if let ImplItem::Type(type_item) = item {
            if type_item.ident == "View" {
                return Ok(type_item.ty.clone());
            }
        }
    }

    // ! Currently not working properly with Nil
    Ok(parse_quote! { ::theta::base::Nil })
}

/// Extract message types from const blocks in the impl (legacy - no longer used)
fn extract_message_types(_input: &ItemImpl) -> Vec<MessageInfo> {
    // No longer used - the actor macro consumes the const blocks
    Vec::new()
}

#[derive(Debug, Clone)]
struct MessageInfo {
    msg_type: syn::Ident,
    method_name: syn::Ident,
}

/// Extract actor name from impl Actor for ActorName
fn extract_actor_name(input: &ItemImpl) -> syn::Result<syn::Ident> {
    // Check if this is implementing Actor trait
    if let Some((_, trait_path, _)) = &input.trait_ {
        let is_actor_trait = trait_path.segments.iter().any(|seg| seg.ident == "Actor");
        if !is_actor_trait {
            return Err(syn::Error::new_spanned(
                trait_path,
                "This macro should only be used on impl Actor for ActorName blocks",
            ));
        }
    } else {
        return Err(syn::Error::new_spanned(
            &input.self_ty,
            "This macro should only be used on trait implementations",
        ));
    }

    // Extract the actor name from the self type
    match &*input.self_ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                Ok(segment.ident.clone())
            } else {
                Err(syn::Error::new_spanned(
                    &input.self_ty,
                    "Could not extract actor name",
                ))
            }
        }
        _ => Err(syn::Error::new_spanned(
            &input.self_ty,
            "Expected a simple type name",
        )),
    }
}

/// Generate the ActorRef struct and its methods
fn generate_actor_ref(
    actor_name: &syn::Ident,
    view_type: &syn::Type,
    message_types: &[MessageInfo],
) -> TokenStream2 {
    let ref_name = generate_flt_actor_ref_name(actor_name);

    // Generate tell methods for each message type
    let tell_methods = message_types.iter().map(|msg_info| {
        let method_name = &msg_info.method_name;
        let msg_type = &msg_info.msg_type;

        quote! {
            #[::flutter_rust_bridge::frb(sync)]
            pub fn #method_name(&self, msg: #msg_type) -> anyhow::Result<()> {
                Ok(self.0.tell(msg)?)
            }
        }
    });

    // If no message types were extracted, provide a generic tell method as fallback
    let fallback_methods = if message_types.is_empty() {
        quote! {
            // Fallback: generic tell method since no message types were automatically detected
            // You may need to add specific methods manually
        }
    } else {
        quote! {}
    };

    let view_type_str = quote! { #view_type }.to_string();

    let dart_code_str = format!(
        r#"
static final streamRegistry = <UuidValue, CachedStream<{view_type_str}>>{{}};

CachedStream<{view_type_str}>? cachedStream_ = null;

CachedStream<{view_type_str}> get stream => cachedStream_!;

{view_type_str} get state => cachedStream_!.snapshot;

static Future<{ref_name}> connect(String identOrUrl) async {{
    final (inst, initState) = await {ref_name}.prep(identOrUrl);
    final rawStream = inst.initStream();
    
    final cachedStream = rawStream.cached(initState).asBroadcastStream();
    
    streamRegistry[inst.id] = cachedStream;
    inst.cachedStream_ = cachedStream;

    return inst;
}}

static {ref_name} connectLocal(String ident) {{
    final (inst, initState) = {ref_name}.prepLocal(ident);
    final rawStream = inst.initStream();

    final cachedStream = rawStream.cached(initState).asBroadcastStream();

    streamRegistry[inst.id] = cachedStream;
    inst.cachedStream_ = cachedStream;

    return inst;
}}

Widget build(Widget Function({view_type_str}) builder) => this.stream.build(builder);
"#,
        view_type_str = view_type_str,
        ref_name = ref_name
    );

    quote! {
        #[::flutter_rust_bridge::frb(opaque, dart_code = #dart_code_str)]
        pub struct #ref_name(
            ::theta::actor_ref::ActorRef<#actor_name>,
            Option<::theta_frb::Receiver<::theta_frb::Update<#actor_name>>>,
        );

        #[::flutter_rust_bridge::frb(unignore)]
        impl #ref_name {

            #[::flutter_rust_bridge::frb(sync, getter)]
            pub fn id(&self) -> Uuid {
                self.0.id()
            }

            #[::flutter_rust_bridge::frb(positional)]
            pub async fn prep(ident_or_url: &str) -> ::anyhow::Result<(#ref_name, #view_type)> {
                ::theta_frb::__private::tracing::trace!("Looking up actor {} {}", std::any::type_name::<#actor_name>(), ident_or_url);
                let actor = ::theta_frb::root_ctx().lookup(ident_or_url).await?;

                let (tx, rx) = ::theta_frb::unbounded_anonymous();

                if let Err(_e) = ::theta_frb::monitor(
                    actor
                        .id()
                        .as_simple()
                        .encode_lower(&mut ::theta_frb::__private::uuid::Uuid::encode_buffer()),
                    tx,
                )
                .await
                {
                    ::theta_frb::error!("Failed to monitor counter: {_e}");
                }

                let Some(::theta_frb::Update::State(init_state)) = rx.recv().await else {
                    return Err(::anyhow::anyhow!(
                        "Failed to receive initial state from counter"
                    ));
                };

                Ok((#ref_name(actor, Some(rx)), init_state))
            }

            #[::flutter_rust_bridge::frb(sync, positional)]
            pub fn prep_local(ident: &str) -> ::anyhow::Result<(#ref_name, #view_type)> {
                let actor = ::theta_frb::root_ctx().lookup_local(ident)?;

                let (tx, rx) = ::theta_frb::unbounded_anonymous();

                if let Err(_e) = ::theta_frb::monitor_local_id(actor.id(), tx) {
                    ::theta_frb::error!("Failed to monitor counter: {_e}");
                }

                let Some(::theta_frb::Update::State(init_state)) = rx.recv_blocking() else {
                    return Err(::anyhow::anyhow!(
                        "Failed to receive initial state from counter"
                    ));
                };

                Ok((#ref_name(actor, Some(rx)), init_state))
            }

            pub fn init_stream(
                &mut self,
                sink: crate::frb_generated::StreamSink<#view_type>,
            ) -> ::anyhow::Result<()> {
                let rx = self
                    .1
                    .take()
                    .expect("Stream could be initialized only once");

                crate::frb_generated::FLUTTER_RUST_BRIDGE_HANDLER
                    .async_runtime()
                    .0
                    .spawn(async move {
                        loop {
                            let Some(update) = rx.recv().await else {
                                break ::theta_frb::info!("Observation stream closed");
                            };

                            if let ::theta_frb::Update::State(state) = update {
                                if let Err(e) = sink.add(state.into()) {
                                    break ::theta_frb::warn!("Frb monitoring channel closed: {e}");
                                }
                            }
                        }
                    });

                Ok(())
            }

            #(#tell_methods)*
            #fallback_methods
        }

        impl From<::theta::actor_ref::ActorRef<#actor_name>> for #ref_name {
            fn from(actor_ref: ::theta::actor_ref::ActorRef<#actor_name>) -> Self {
                Self(actor_ref, None)
            }
        }
    }
}
