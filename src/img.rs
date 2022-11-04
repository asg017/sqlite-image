use crate::util::{
    load_image, result_dynamic_image_as, result_imagebuffer_as, result_imagebuffer_luma_as,
};

use image::{
    imageops, DynamicImage, GenericImageView, ImageBuffer, ImageFormat, ImageOutputFormat,
};
use sqlite3_loadable::{
    api::{context_result_int, value_double, value_int, value_text},
    errors::Result,
    sqlite3_context, sqlite3_value, Error,
};

pub fn img_width(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, _) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    context_result_int(context, image.dimensions().0.try_into().unwrap());
    Ok(())
}

pub fn img_grayscale(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    result_imagebuffer_luma_as(context, imageops::grayscale(&image), format) //ImageOutputFormat::Jpeg(90))
}

pub fn img_blur(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    let sigma = value_double(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as f32;
    let blurred = imageops::blur(&image, sigma);
    let x = DynamicImage::ImageRgba8(blurred).into_luma8();
    result_imagebuffer_luma_as(context, x, format)
}

pub fn img_replace(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (mut base, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    let (top, _) = load_image(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    let x = value_int(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as i64;
    let y = value_int(
        values
            .get(3)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as i64;

    imageops::replace(&mut base, &top, x, y);
    result_dynamic_image_as(context, base, format)
}

pub fn img_overlay_all(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (mut base, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    for group in values[1..].chunks(3) {
        let (top, _) = load_image(
            group
                .get(0)
                .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
                .to_owned(),
        )?;
        let x = value_int(
            group
                .get(1)
                .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
                .to_owned(),
        ) as i64;
        let y = value_int(
            group
                .get(2)
                .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
                .to_owned(),
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
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
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
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    imageops::flip_vertical_in_place(&mut image);
    result_dynamic_image_as(context, image, format)
}

pub fn img_rotate180(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (mut image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    imageops::rotate180_in_place(&mut image);
    result_dynamic_image_as(context, image, format)
}

pub fn img_rotate90(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (mut image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    let rotated = imageops::rotate90(&mut image);
    result_imagebuffer_as(context, rotated, format)
}

pub fn img_rotate270(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (mut image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    let rotated = imageops::rotate270(&mut image);
    result_imagebuffer_as(context, rotated, format)
}

pub fn img_filter3x3(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    todo!();
}

pub fn img_thumbnail(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (mut image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    let width = value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u32;
    let height = value_int(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u32;

    let thumbnail = imageops::thumbnail(&mut image, width, height);
    result_imagebuffer_as(context, thumbnail, format)
}

pub fn img_solid_rgb(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let width = value_int(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u32;
    let height = value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u32;
    let r = value_int(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u8;
    let g = value_int(
        values
            .get(3)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u8;
    let b = value_int(
        values
            .get(4)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u8;

    let pix = image::Rgba([r, g, b, 1]);
    let solid = ImageBuffer::from_fn(width, height, |x, y| pix);
    result_imagebuffer_as(context, solid, Some(ImageFormat::Jpeg))
}

pub fn img_rect(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let width = value_int(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u32;
    let height = value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
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
    let (mut image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    let width = value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u32;
    let height = value_int(
        values
            .get(2)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u32;
    let filter = match values.get(3) {
        Some(value) => match value_text(value.to_owned())?.to_lowercase().as_str() {
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
    let resized = imageops::resize(&mut image, width, height, filter);
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
    format: Option<ImageOutputFormat>,
) -> Result<()> {
    let (mut image, format) = load_image(
        values
            .get(0)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    )?;
    let x = value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u32;
    let y = value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u32;
    let width = value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u32;
    let height = value_int(
        values
            .get(1)
            .ok_or_else(|| Error::new_message("Expected TODO as TODO argument"))?
            .to_owned(),
    ) as u32;
    let subimg = imageops::crop(&mut image, x, y, width, height);
    result_imagebuffer_as(context, subimg.to_image(), format)
}
