pub mod color;
pub mod util;

#[cfg(test)]
mod tests {
    use crate::color::WHITE;
    use super::util::{get_pixel, set_pixel};

    #[test]
    fn it_works() {
        const WIDTH: usize = 3;
        const HEIGHT: usize = 3;
        let buffer: &mut [u8] = &mut [0u8; WIDTH * HEIGHT * 4];

        set_pixel(buffer, (1, 1), WIDTH as u32, HEIGHT as u32, WHITE);
        assert_eq!(get_pixel(buffer, (1, 1), WIDTH as u32, HEIGHT as u32), Some(WHITE));
    }
}
