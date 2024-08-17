use crate::world_generation::Pixel;

pub fn render_grid(grid: &Vec<Pixel>, image_buffer: &mut Vec<u8>) {
    for i in 0..grid.len() {
        match grid[i] {
            Pixel::Ground => {
                image_buffer[4*i] = 88;
                image_buffer[4*i+1] = 57;
                image_buffer[4*i+2] = 39;
            },
            Pixel::Sky => {
                image_buffer[4*i] = 135;
                image_buffer[4*i+1] = 206;
                image_buffer[4*i+2] = 235;
            },
        };
    }
}