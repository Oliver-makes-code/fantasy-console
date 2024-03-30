#![feature(stmt_expr_attributes, decl_macro)]
use console::{wasm::WasmCart, WindowState};
use winit::event_loop::EventLoop;

fn main() {
    WasmCart::load("main.wasm").unwrap();
    let event_loop = EventLoop::new().unwrap();
    WindowState::new(&event_loop).run_event_loop(event_loop);
}
