use super::color::Color;

pub fn map_coordinate_to_index(position: (u32, u32), width: u32) -> usize {
    (position.1 * width + position.0) as usize
}

pub fn map_index_to_coordinate(index: usize, width: u32) -> (u32, u32) {
    ((index as u32) % width, (index as u32) / width)
}

pub fn set_pixel(frame: &mut [u8], position: (u32, u32), width: u32, height: u32, color: Color) {
    if position.0 >= width || position.1 >= height {
        return;
    }
    let naive_index = map_coordinate_to_index(position, width);
    let start = naive_index * 4;
    let end = start + 4;

    frame[start..end].copy_from_slice(&color);
}

pub fn get_pixel(frame: &[u8], position: (u32, u32), width: u32, height: u32) -> Option<Color> {
    if position.0 >= width || position.1 >= height {
        return None;
    }
    let naive_index = map_coordinate_to_index(position, width);
    let start = naive_index * 4;
    let end = start + 4;

    let mut output = [0u8; 4];

    output.copy_from_slice(&frame[start..end]);

    Some(output)
}

pub fn blend(a: Color, b: Color) -> Color {
    let mut output = [0u8; 4];
    for channel in 0..3 {
        output[channel] = ((a[channel] as u16 + b[channel] as u16) / 2) as u8;
    }
    output[3] = 0xFF;

    output
}
