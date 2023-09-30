use bevy_egui::egui;

pub fn modulo_robust(first: i32, second: i32) -> usize {
    ((first + (second)) % (second)) as usize
}

fn torus_pixel_channel(
    image: &egui::ColorImage,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    channel: usize,
) -> f32 {
    image.pixels[modulo_robust(x, width) + (width as usize) * modulo_robust(y, height)].to_array()
        [channel] as f32
}

pub fn sum_neighbour_channel(
    image: &egui::ColorImage,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    channel: usize,
) -> f32 {
    torus_pixel_channel(image, x - 1, y - 1, width, height, channel) / 1.41
        + torus_pixel_channel(image, x, y - 1, width, height, channel)
        + torus_pixel_channel(image, x + 1, y - 1, width, height, channel) / 1.41
        + torus_pixel_channel(image, x - 1, y - 1, width, height, channel) / 1.41
        + torus_pixel_channel(image, x + 1, y, width, height, channel)
        + torus_pixel_channel(image, x - 1, y, width, height, channel)
        + torus_pixel_channel(image, x - 1, y + 1, width, height, channel) / 1.41
        + torus_pixel_channel(image, x, y + 1, width, height, channel)
        + torus_pixel_channel(image, x + 1, y + 1, width, height, channel) / 1.41
}
