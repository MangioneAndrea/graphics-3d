use std::sync::mpsc::Sender;

use crate::ScreenChunk;

pub struct ColorsView;

impl super::View for ColorsView {
    fn get_name(&self) -> &'static str {
        "Colors"
    }

    fn step<'a>(&mut self, buffer: Sender<ScreenChunk>, width: u32, height: u32) {
        let mut sc = ScreenChunk {
            from: 0,
            data: vec![],
        };
        for index in 0..(width * height) {
            let y = index as f32 / width as f32;
            let x = index % width;
            let red = (x as f32 / width as f32 * 255.) as u32;
            let green = (y as f32 / height as f32 * 255.) as u32;
            let blue = 0; //(x * y as u32) % 255;

            // buffer[index as usize] = blue | (green << 8) | (red << 16);
            sc.data.push(blue | (green << 8) | (red << 16));
        }

        buffer.send(sc).unwrap();
    }
}
