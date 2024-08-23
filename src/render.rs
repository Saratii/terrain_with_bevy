use crate::world_generation::Pixel;

pub fn render_grid(grid: &Vec<Pixel>, image_buffer: &mut Vec<u8>) {
    for i in 0..grid.len() {
        match grid[i] {
            Pixel::Ground => {
                image_buffer[4*i] = 88;
                image_buffer[4*i+1] = 57;
                image_buffer[4*i+2] = 39;
                image_buffer[4*i+3] = 255;
            },
            Pixel::Sky => {
                image_buffer[4*i] = 135;
                image_buffer[4*i+1] = 206;
                image_buffer[4*i+2] = 235;
                image_buffer[4*i+3] = 255;
            },
            Pixel::Clear => {
                image_buffer[4*i] = 0;
                image_buffer[4*i+1] = 0;
                image_buffer[4*i+2] = 0;
                image_buffer[4*i+3] = 0;
            },
            Pixel::TranslucentGrey => {
                image_buffer[4*i] = 135;
                image_buffer[4*i+1] = 206;
                image_buffer[4*i+2] = 235;
                image_buffer[4*i+3] = 150;
            },
            Pixel::White => {
                image_buffer[4*i] = 255;
                image_buffer[4*i+1] = 255;
                image_buffer[4*i+2] = 255;
                image_buffer[4*i+3] = 255;
            },
        };
    }
}