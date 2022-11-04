use crate::util::{load_image, result_dynamic_image_as};
use image::{ImageOutputFormat, ImageFormat};
use sqlite3_loadable::{api::value_int, errors::Result, sqlite3_context, sqlite3_value};

pub fn img_as_png(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, _) = load_image(values.get(0).unwrap().to_owned())?;
    result_dynamic_image_as(context, image, Some(ImageFormat::Png))
}

pub fn img_as_jpeg(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, _) = load_image(values.get(0).unwrap().to_owned())?;
    let quality = value_int(values.get(1).unwrap().to_owned());
    result_dynamic_image_as(
        context,
        image,
        Some(ImageFormat::Jpeg),
    )
}

pub fn img_as_webp(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, _) = load_image(values.get(0).unwrap().to_owned())?;
    result_dynamic_image_as(context, image, Some(ImageFormat::WebP))
}
