use std::{
    ops::DerefMut,
    stringify,
    sync::{Mutex, OnceLock},
};

use wasmtime::{AsContextMut, Engine, Instance, Linker, Memory, Module, Store, TypedFunc};

static CART: OnceLock<WasmCart> = OnceLock::new();
pub struct WasmCart {
    store: Mutex<Store<()>>,
    instance: Instance,
    init: TypedFunc<(), ()>,
    update: TypedFunc<(), ()>,
    v_blank: TypedFunc<u32, ()>,
}

impl WasmCart {
    pub fn load(file_path: &str) -> wasmtime::Result<()> {
        let cart = Self::new(file_path)?;
        CART.get_or_init(move || cart);
        Ok(())
    }

    pub fn init() {
        let cart = Self::get();
        let mut store = cart.store.lock().unwrap();
        cart.init.call(store.deref_mut(), ()).unwrap();
    }

    pub fn update() {
        let cart = Self::get();
        let mut store = cart.store.lock().unwrap();
        cart.update.call(store.deref_mut(), ()).unwrap();
    }

    pub fn v_blank(y: u32) {
        let cart = Self::get();
        let mut store = cart.store.lock().unwrap();
        cart.v_blank.call(store.deref_mut(), y).unwrap();
    }

    fn get() -> &'static Self {
        &CART.get().unwrap()
    }

    fn new(file_path: &str) -> wasmtime::Result<Self> {
        let engine = Engine::default();
        let module = Module::from_file(&engine, file_path)?;

        let mut store: Store<()> = Store::new(&engine, ());

        let mut linker = Linker::<()>::new(&engine);

        func!(
            linker,
            tile::write_palette,
            tile::write_tile,
            tile::set_background_tile,
            tile::set_background_scroll_post,
            tile::set_background_scroll_post_x,
            tile::set_background_scroll_post_y,
            tile::get_background_scroll_post,
            tile::get_background_scroll_post_x,
            tile::get_background_scroll_post_y,
            tile::set_background_scroll_pre,
            tile::set_background_scroll_pre_x,
            tile::set_background_scroll_pre_y,
            tile::get_background_scroll_pre,
            tile::get_background_scroll_pre_x,
            tile::get_background_scroll_pre_y,
            tile::set_background_transformation_matrix,
            dbg::write_character,
            dbg::write_str,
            dbg::end_line
        );

        let instance = linker.instantiate(&mut store, &module)?;

        let init = instance.get_typed_func::<(), ()>(&mut store, "init")?;
        let update = instance.get_typed_func::<(), ()>(&mut store, "update")?;
        let v_blank = instance.get_typed_func::<u32, ()>(&mut store, "v_blank")?;

        Ok(Self {
            store: Mutex::new(store),
            instance,
            init,
            update,
            v_blank,
        })
    }

    fn get_memory(&self, store: impl AsContextMut) -> Memory {
        self.instance.get_memory(store, "memory").unwrap()
    }
}

macro func($linker: expr, $($module: ident :: $f: ident),+) {
    $(
        $linker.func_wrap(stringify!($module), stringify!($f), $module::$f)?;
    )+
}

mod dbg {
    use std::ffi::CStr;

    use wasmtime::Caller;

    use super::WasmCart;

    pub fn write_character(c: u32) {
        print!("{}", char::from_u32(c).unwrap());
    }

    pub fn write_str(mut caller: Caller<()>, s: u32) {
        let cart = WasmCart::get();
        let mem = cart.get_memory(&mut caller);

        unsafe {
            let p = mem.data_ptr(caller).offset(s as isize) as *const i8;

            let cs = CStr::from_ptr(p);

            print!("{}", cs.to_str().unwrap());
        }
    }

    pub fn end_line() {
        println!();
    }
}

mod tile {
    use wasmtime::Caller;

    use crate::{math::Fixed, tile::TileState};

    use super::WasmCart;

    pub fn write_palette(palette: u32, color: u32) {
        TileState::get().palette[(palette & 255) as usize] = color.into();
    }

    pub fn write_tile(mut caller: Caller<()>, tile: u32, data_ptr: u32) {
        let cart = WasmCart::get();
        let mem = cart.get_memory(&mut caller);
        mem.read(
            caller,
            data_ptr as usize,
            &mut TileState::get().tiles[(tile & 255) as usize].0,
        )
        .unwrap();
    }

    pub fn set_background_tile(bg: u32, x: i32, y: i32, tile: i32) {
        TileState::get().backgrounds[bg as usize].tiles
            [(x & 0b111111) as usize + (y & 0b111111) as usize * 64] = tile as u8;
    }

    pub fn get_background_scroll_pre(bg: u32) -> (u32, u32) {
        let coords = TileState::get().backgrounds[bg as usize].pre_offset;
        (i16::from(coords.0) as u32, i16::from(coords.1) as u32)
    }

    pub fn get_background_scroll_pre_x(bg: u32) -> u32 {
        let coords = TileState::get().backgrounds[bg as usize].pre_offset;
        i16::from(coords.0) as u32
    }

    pub fn get_background_scroll_pre_y(bg: u32) -> u32 {
        let coords = TileState::get().backgrounds[bg as usize].pre_offset;
        i16::from(coords.1) as u32
    }

    pub fn set_background_scroll_pre(bg: u32, x: i32, y: i32) {
        TileState::get().backgrounds[bg as usize].pre_offset = (x as i16, y as i16);
    }

    pub fn set_background_scroll_pre_x(bg: u32, x: i32) {
        TileState::get().backgrounds[bg as usize].pre_offset.0 = x as i16;
    }

    pub fn set_background_scroll_pre_y(bg: u32, y: i32) {
        TileState::get().backgrounds[bg as usize].pre_offset.0 = y as i16;
    }

    pub fn get_background_scroll_post(bg: u32) -> (u32, u32) {
        let coords = TileState::get().backgrounds[bg as usize].post_offset;
        (i16::from(coords.0) as u32, i16::from(coords.1) as u32)
    }

    pub fn get_background_scroll_post_x(bg: u32) -> u32 {
        let coords = TileState::get().backgrounds[bg as usize].post_offset;
        i16::from(coords.0) as u32
    }

    pub fn get_background_scroll_post_y(bg: u32) -> u32 {
        let coords = TileState::get().backgrounds[bg as usize].post_offset;
        i16::from(coords.1) as u32
    }

    pub fn set_background_scroll_post(bg: u32, x: i32, y: i32) {
        TileState::get().backgrounds[bg as usize].post_offset = (x as i16, y as i16);
    }

    pub fn set_background_scroll_post_x(bg: u32, x: i32) {
        TileState::get().backgrounds[bg as usize].post_offset.0 = x as i16;
    }

    pub fn set_background_scroll_post_y(bg: u32, y: i32) {
        TileState::get().backgrounds[bg as usize].post_offset.0 = y as i16;
    }

    pub fn set_background_transformation_matrix(bg: u32, a: i32, b: i32, c: i32, d: i32) {
        TileState::get().backgrounds[bg as usize].matrix = (
            (Fixed::from(a as i16), Fixed::from(b as i16)),
            (Fixed::from(c as i16), Fixed::from(d as i16)),
        )
    }
}
