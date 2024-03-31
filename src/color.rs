#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        ((value.r as u32) << 16) | ((value.g as u32) << 8) | (value.b as u32)
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self {
            r: ((value >> 16) % 256) as u8,
            g: ((value >> 8) % 256) as u8,
            b: (value % 256) as u8,
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}
