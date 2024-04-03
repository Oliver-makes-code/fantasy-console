#![feature(stmt_expr_attributes, decl_macro)]
use std::env;

use console::{wasm::WasmCart, WindowState};
use winit::event_loop::EventLoop;

fn main() {
    let args: Vec<String> = env::args().collect();
    WasmCart::load(&args[1]).unwrap();
    let event_loop = EventLoop::new().unwrap();
    WindowState::new(&event_loop).run_event_loop(event_loop);
}
