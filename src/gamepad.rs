use std::{
    collections::HashSet,
    sync::{Mutex, MutexGuard, OnceLock},
};

use bitflags::bitflags;
use gilrs::{Button, EventType, GamepadId, Gilrs};

static STATE: OnceLock<Mutex<GamepadStateManager>> = OnceLock::new();

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct GamepadState: u16 {
        const Up = 1;
        const Down = 2;
        const Left = 4;
        const Right = 8;
        const North = 16;
        const South = 32;
        const East = 64;
        const West = 128;
        const LeftBumber = 256;
        const RightBumper = 512;
        const Start = 1024;
        const Select = 2048;
        const Disconnected = 4096;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PhysicalGamepadType {
    Gamepad(GamepadId),
    Disconnected,
}

#[derive(Debug, Clone, Copy)]
pub struct Gamepad {
    pub physical_gamepad: PhysicalGamepadType,
    pub state: GamepadState,
}

#[derive(Debug)]
pub struct GamepadStateManager {
    pub gil_gamepads: HashSet<GamepadId>,
    pub gilrs: Gilrs,
    pub gamepads: [Gamepad; 4],
}

impl GamepadStateManager {
    pub fn get() -> MutexGuard<'static, Self> {
        STATE
            .get_or_init(|| Mutex::new(Self::new()))
            .lock()
            .unwrap()
    }

    pub fn update() {
        Self::get().pump_events();
    }

    fn new() -> Self {
        let gilrs = Gilrs::new().unwrap();
        let mut gil_gamepads = HashSet::new();

        for (id, _) in gilrs.gamepads() {
            gil_gamepads.insert(id);
        }

        let gamepads = [Gamepad {
            physical_gamepad: PhysicalGamepadType::Disconnected,
            state: GamepadState::Disconnected,
        }; 4];

        Self {
            gilrs,
            gil_gamepads,
            gamepads,
        }
    }

    fn pump_events(&mut self) {
        let mut inputted_gamepads = HashSet::new();
        while let Some(gilrs::Event { id, event, time: _ }) = self.gilrs.next_event() {
            match event {
                EventType::Connected => {
                    self.gil_gamepads.insert(id);
                }
                EventType::Disconnected => {
                    self.gil_gamepads.remove(&id);
                }
                _ => {
                    inputted_gamepads.insert(id);
                }
            }
        }

        for gamepad in &self.gamepads {
            if let PhysicalGamepadType::Gamepad(id) = &gamepad.physical_gamepad {
                inputted_gamepads.remove(id);
            }
        }

        for i in 0..4 {
            let gamepad = &mut self.gamepads[i];

            if let PhysicalGamepadType::Gamepad(id) = gamepad.physical_gamepad
                && !self.gilrs.gamepad(id).is_connected()
            {
                gamepad.physical_gamepad = PhysicalGamepadType::Disconnected;
            }

            if let PhysicalGamepadType::Disconnected = gamepad.physical_gamepad
                && let Some(id) = inputted_gamepads.iter().next()
            {
                let id = id.clone();
                inputted_gamepads.remove(&id);
                gamepad.physical_gamepad = PhysicalGamepadType::Gamepad(id);
            }

            match gamepad.physical_gamepad {
                PhysicalGamepadType::Gamepad(id) => {
                    let gil_gamepad = self.gilrs.gamepad(id);
                    gamepad.state = GamepadState::empty();
                    gil_button_map!(
                        gamepad, gil_gamepad,
                        Button::DPadUp => GamepadState::Up,
                        Button::DPadDown => GamepadState::Down,
                        Button::DPadLeft => GamepadState::Left,
                        Button::DPadRight => GamepadState::Right,
                        Button::North => GamepadState::North,
                        Button::East => GamepadState::East,
                        Button::South => GamepadState::South,
                        Button::West => GamepadState::West,
                        Button::LeftTrigger => GamepadState::LeftBumber,
                        Button::RightTrigger => GamepadState::RightBumper,
                        Button::Start => GamepadState::Start,
                        Button::Select => GamepadState::Select
                    );
                }
                PhysicalGamepadType::Disconnected => {
                    gamepad.state = GamepadState::Disconnected;
                }
            }
        }
    }
}

macro gil_button_map(
    $gamepad: expr, $gil_gamepad: expr,
    $($gil_button: expr => $button: expr),+
) {
    $(
        if $gil_gamepad.is_pressed($gil_button) {
            $gamepad.state |= $button
        }
    )+
}
