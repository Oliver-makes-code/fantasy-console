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
            tile::set_background_palette,
            tile::set_background_tile_palette,
            tile::set_background_visible,
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
            sprite::set_sprite_palette,
            sprite::set_sprite_tile,
            sprite::set_sprite_visible,
            sprite::get_sprite_position,
            sprite::get_sprite_position_x,
            sprite::get_sprite_position_y,
            sprite::set_sprite_position,
            sprite::set_sprite_flip,
            dbg::write_character,
            dbg::write_str,
            dbg::write_int,
            dbg::write_uint,
            dbg::write_ptr,
            dbg::end_line,
            gamepad::get_state
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

mod gamepad {
    use crate::gamepad::GamepadStateManager;

    pub fn get_state(gamepad_idx: u32) -> u32 {
        GamepadStateManager::get().gamepads[gamepad_idx as usize]
            .state
            .bits() as u32
    }
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

    pub fn write_int(i: i64) {
        print!("{i}");
    }

    pub fn write_uint(i: u64) {
        print!("{i}");
    }

    pub fn write_ptr(i: u32) {
        print!("{i:#x}");
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

    pub fn set_background_tile(bg: u32, x: u32, y: u32, tile: i32) {
        TileState::get().backgrounds[bg as usize].tiles
            [(x & 0b111111) as usize + (y & 0b111111) as usize * 64] = tile as u8;
    }

    pub fn set_background_palette(palette: u32) {
        TileState::get().background_color = palette as u8;
    }

    pub fn set_background_tile_palette(bg: u32, x: i32, y: i32, palette: u32) {
        TileState::get().backgrounds[bg as usize].palettes
            [(x & 0b111111) as usize + (y & 0b111111) as usize * 64] = palette as u8;
    }

    pub fn set_background_visible(bg: u32, visible: u32) {
        TileState::get().backgrounds[bg as usize].visible = visible != 0;
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

mod sprite {
    use crate::tile::TileState;

    pub fn set_sprite_visible(sprite: u32, visible: u32) {
        TileState::get().sprites[sprite as usize].visible = visible != 0;
    }

    pub fn set_sprite_tile(sprite: u32, tile: u32) {
        TileState::get().sprites[sprite as usize].tile = tile as u8;
    }

    pub fn set_sprite_palette(sprite: u32, palette: u32) {
        TileState::get().sprites[sprite as usize].palette = palette as u8;
    }

    pub fn get_sprite_position(sprite: u32) -> (i32, i32) {
        let pos = TileState::get().sprites[sprite as usize].position;

        (pos.0 as i32, pos.1 as i32)
    }

    pub fn get_sprite_position_x(sprite: u32) -> i32 {
        TileState::get().sprites[sprite as usize].position.0 as i32
    }

    pub fn get_sprite_position_y(sprite: u32) -> i32 {
        TileState::get().sprites[sprite as usize].position.1 as i32
    }

    pub fn set_sprite_position(sprite: u32, x: i32, y: i32) {
        TileState::get().sprites[sprite as usize].position = (x as i16, y as i16);
    }

    pub fn set_sprite_flip(sprite: u32, flip_x: u32, flip_y: u32) {
        let sprite = &mut TileState::get().sprites[sprite as usize];

        sprite.flip_x = flip_x != 0;
        sprite.flip_y = flip_y != 0;
    }
}

