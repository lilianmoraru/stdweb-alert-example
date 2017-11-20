#![cfg_attr(not( any(target_arch = "wasm32", target_arch="asmjs") ),
            feature(plugin))]
#![cfg_attr(not( any(target_arch = "wasm32", target_arch="asmjs") ),
            plugin(rocket_codegen))]

#[cfg( any(target_arch = "wasm32", target_arch="asmjs") )]
#[macro_use]
extern crate stdweb;

#[cfg( not( any(target_arch = "wasm32", target_arch="asmjs") ) )]
extern crate rocket;

#[cfg( any(target_arch = "wasm32", target_arch="asmjs") )]
#[path = "client/mod.rs"]
mod detail;

#[cfg( not( any(target_arch = "wasm32", target_arch="asmjs") ) )]
#[path = "server/mod.rs"]
mod detail;

fn main() {
    detail::main();
}
