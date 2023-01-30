use crate::util::{
    load_image, result_dynamic_image_as, result_imagebuffer_as, result_imagebuffer_luma_as,
};

use image::{
    imageops, DynamicImage, GenericImageView, ImageBuffer, ImageFormat, ImageOutputFormat,
};
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};

pub fn img_width(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, _) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    api::result_int(context, image.dimensions().0.try_into().unwrap());
    Ok(())
}

pub fn img_grayscale(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    result_imagebuffer_luma_as(context, imageops::grayscale(&image), format) //ImageOutputFormat::Jpeg(90))
}

pub fn img_blur(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    let sigma = api::value_double(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as f32;
    let blurred = imageops::blur(&image, sigma);
    let x = DynamicImage::ImageRgba8(blurred).into_luma8();
    result_imagebuffer_luma_as(context, x, format)
}

pub fn img_replace(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (mut base, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    let (top, _) = load_image(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    let x = api::value_int(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as i64;
    let y = api::value_int(
        values
            .get(3)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as i64;

    imageops::replace(&mut base, &top, x, y);
    result_dynamic_image_as(context, base, format)
}

pub fn img_overlay_all(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (mut base, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    for group in values[1..].chunks(3) {
        let (top, _) = load_image(
            group
                .get(0)
                .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
        )?;
        let x = api::value_int(
            group
                .get(1)
                .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
        ) as i64;
        let y = api::value_int(
            group
                .get(2)
                .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
        ) as i64;
        imageops::overlay(&mut base, &top, x, y);
    }
    result_dynamic_image_as(context, base, format)
}

pub fn img_flip_horizontal(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let (mut image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    imageops::flip_horizontal_in_place(&mut image);
    result_dynamic_image_as(context, image, format)
}

pub fn img_flip_vertical(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
) -> Result<()> {
    let (mut image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    imageops::flip_vertical_in_place(&mut image);
    result_dynamic_image_as(context, image, format)
}

pub fn img_rotate180(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (mut image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    imageops::rotate180_in_place(&mut image);
    result_dynamic_image_as(context, image, format)
}

pub fn img_rotate90(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    let rotated = imageops::rotate90(&image);
    result_imagebuffer_as(context, rotated, format)
}

pub fn img_rotate270(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    let rotated = imageops::rotate270(&image);
    result_imagebuffer_as(context, rotated, format)
}

pub fn img_filter3x3(_context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    todo!();
}

pub fn img_thumbnail(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    let width = api::value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;
    let height = api::value_int(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;

    let thumbnail = imageops::thumbnail(&image, width, height);
    result_imagebuffer_as(context, thumbnail, format)
}

pub fn img_solid_rgb(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let width = api::value_int(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;
    let height = api::value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;
    let r = api::value_int(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u8;
    let g = api::value_int(
        values
            .get(3)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u8;
    let b = api::value_int(
        values
            .get(4)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u8;

    let pix = image::Rgba([r, g, b, 1]);
    let solid = ImageBuffer::from_fn(width, height, |_, _| pix);
    result_imagebuffer_as(context, solid, Some(ImageFormat::Jpeg))
}

pub fn img_rect(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let width = api::value_int(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;
    let height = api::value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;
    let border_width = 4;

    let pix_border = image::Rgba([200, 0, 0, 1]);
    let pix_content = image::Rgba([100, 100, 100, 0]);
    let solid = ImageBuffer::from_fn(
        width + (border_width * 2),
        height + (border_width * 2),
        |x, y| {
            let in_content = (x > border_width && x < (width + border_width))
                && (y > border_width && y < (height + border_width));
            if in_content {
                pix_content
            } else {
                pix_border
            }
        },
    );
    result_imagebuffer_as(context, solid, Some(ImageFormat::Gif))
}

pub fn img_resize(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    let width = api::value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;
    let height = api::value_int(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;
    let filter = match values.get(3) {
        Some(value) => match api::value_text(value)?.to_lowercase().as_str() {
            "nearest" => imageops::FilterType::Nearest,
            "catmullrom" => imageops::FilterType::CatmullRom,
            "gaussian" => imageops::FilterType::Gaussian,
            "lanczos3" => imageops::FilterType::Lanczos3,
            "triangle" => imageops::FilterType::Triangle,
            _ => {
                return Err(Error::new_message(
                    "Uknown filter type, must be one of TODO",
                ))
            }
        },
        None => imageops::FilterType::Nearest,
    };
    let resized = imageops::resize(&image, width, height, filter);
    result_imagebuffer_as(context, resized, format)
}

pub fn img_crop_png(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    img_crop_as(context, values, Some(ImageOutputFormat::Png))
}
pub fn img_crop_jpeg(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    img_crop_as(context, values, Some(ImageOutputFormat::Jpeg(90)))
}

pub fn img_crop(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    img_crop_as(context, values, None)
}

pub fn img_crop_as(
    context: *mut sqlite3_context,
    values: &[*mut sqlite3_value],
    _format: Option<ImageOutputFormat>,
) -> Result<()> {
    let (mut image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    )?;
    let x = api::value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;
    let y = api::value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;
    let width = api::value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;
    let height = api::value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?,
    ) as u32;
    let subimg = imageops::crop(&mut image, x, y, width, height);
    result_imagebuffer_as(context, subimg.to_image(), format)
}
