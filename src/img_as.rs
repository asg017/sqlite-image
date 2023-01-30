use crate::util::{load_image, result_dynamic_image_as};
use image::{ImageFormat, ImageOutputFormat};
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, errors::Result};

pub fn img_as_png(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, _) = load_image(values.get(0).unwrap())?;
    result_dynamic_image_as(context, image, Some(ImageFormat::Png))
}

pub fn img_as_jpeg(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, _) = load_image(values.get(0).unwrap())?;
    let quality = api::value_int(values.get(1).unwrap());
    result_dynamic_image_as(context, image, Some(ImageFormat::Jpeg))
}

pub fn img_as_webp(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> Result<()> {
    let (image, _) = load_image(values.get(0).unwrap())?;
    result_dynamic_image_as(context, image, Some(ImageFormat::WebP))
}
