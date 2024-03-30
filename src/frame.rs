use crate::color::Color;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 240;
pub const DIMENSIONS: (usize, usize) = (WIDTH, HEIGHT);
const BUFFER_LEN: usize = WIDTH * HEIGHT * 4;

pub struct FrameBuffer {
    data: [u8; BUFFER_LEN],
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self {
            data: [255; BUFFER_LEN],
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x > WIDTH || y > HEIGHT {
            return;
        }

        let start_pos = (x + y * WIDTH) * 4;
        self.data[start_pos] = color.r;
        self.data[start_pos + 1] = color.g;
        self.data[start_pos + 2] = color.b;
    }
}

impl<'a> From<&'a FrameBuffer> for &'a [u8] {
    fn from(value: &'a FrameBuffer) -> Self {
        &value.data
    }
}
