mod decode;
mod encode;
mod entity_pool;
mod packet;

use proc_macro::TokenStream as StdTokenStream;

#[proc_macro_derive(Packet, attributes(packet))]
pub fn derive_packet(item: StdTokenStream) -> StdTokenStream {
    match packet::derive_packet(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(Encode, attributes(protocol))]
pub fn derive_encode(item: StdTokenStream) -> StdTokenStream {
    match encode::derive_encode(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(Decode, attributes(protocol))]
pub fn derive_decode(item: StdTokenStream) -> StdTokenStream {
    match decode::derive_decode(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(EntityPool, attributes(pool, canonical, required, persistent))]
pub fn derive_entity_pool(item: StdTokenStream) -> StdTokenStream {
    match entity_pool::derive_entity_pool(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
