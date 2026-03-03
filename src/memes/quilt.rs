use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    encoder::{FrameAlign, GifInfo, make_gif_or_combined_gif},
    image::ImageExt,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn quilt(images: Vec<InputImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let positions = [
        (68, 68),
        (68, 68),
        (68, 72),
        (69, 74),
        (69, 74),
        (69, 73),
        (69, 72),
        (69, 74),
        (68, 72),
        (68, 68),
        (68, 68),
        (68, 68),
        (68, 72),
        (68, 72),
        (69, 73),
        (69, 73),
        (69, 73),
        (69, 75),
        (68, 72),
        (68, 68),
        (68, 68),
    ];

    let user_head = images[0].image.resize_exact((142, 146));

    let func = |i: usize, _images: Vec<Image>| {
        let frame = load_image(format!("quilt/{}.png", i + 1))?;
        let (x, y) = positions[i];
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&user_head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        Ok(surface.image_snapshot())
    };
    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 21,
            duration: 0.03,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme! {
    "quilt",
    quilt,
    min_images = 1,
    max_images = 1,
    keywords = &["被窝"],
    date_created = local_date(2025, 9, 12),
    date_modified = local_date(2025, 9, 12),
}
