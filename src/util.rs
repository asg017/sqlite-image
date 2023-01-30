use std::io::Cursor;

use image::{io::Reader, DynamicImage, ImageBuffer, ImageFormat, ImageOutputFormat, Luma, Rgba};
use sqlite_loadable::prelude::*;
use sqlite_loadable::{api, Error, Result};
use std::io::{Read, Seek};

pub fn load_image(value: &*mut sqlite3_value) -> Result<(DynamicImage, Option<ImageFormat>)> {
    let raw = api::value_blob(value);
    let reader = Reader::new(Cursor::new(raw)).with_guessed_format().unwrap();
    let format = reader.format();
    let image = reader
        .decode()
        .map_err(|err| Error::new_message(format!("Error decoding image: {}", err).as_str()))?;
    Ok((image, format))
}

pub fn result_dynamic_image_as(
    context: *mut sqlite3_context,
    image: DynamicImage,
    format: Option<ImageFormat>,
) -> Result<()> {
    let format: ImageOutputFormat = format
        .ok_or_else(|| Error::new_message("Could not determine image format"))?
        .try_into()
        .unwrap();
    let mut c = Cursor::new(Vec::new());
    image.write_to(&mut c, format).unwrap();
    let mut buffer = Vec::new();
    c.seek(std::io::SeekFrom::Start(0))
        .map_err(|_| Error::new_message("Erroring seeking in-memory cursor"))?;
    c.read_to_end(&mut buffer).unwrap();

    api::result_blob(context, buffer.as_slice());
    Ok(())
}

pub fn result_imagebuffer_as(
    context: *mut sqlite3_context,
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    format: Option<ImageFormat>,
) -> Result<()> {
    let format: ImageOutputFormat = format
        .ok_or_else(|| Error::new_message("Could not determine image format"))?
        .try_into()
        .unwrap();
    let mut c = Cursor::new(Vec::new());
    image.write_to(&mut c, format).unwrap();
    let mut buffer = Vec::new();
    c.seek(std::io::SeekFrom::Start(0))
        .map_err(|_| Error::new_message("Erroring seeking in-memory cursor"))?;
    c.read_to_end(&mut buffer)
        .map_err(|_| Error::new_message("Erroring reading in-memory cursor"))?;

    api::result_blob(context, buffer.as_slice());
    Ok(())
}

pub fn result_imagebuffer_luma_as(
    context: *mut sqlite3_context,
    image: ImageBuffer<Luma<u8>, Vec<u8>>,
    format: Option<ImageFormat>,
) -> Result<()> {
    let format: ImageOutputFormat = format
        .ok_or_else(|| Error::new_message("Could not determine image format"))?
        .try_into()
        .unwrap();
    let mut c = Cursor::new(Vec::new());
    //image.set
    image.write_to(&mut c, format).unwrap();
    let mut buffer = Vec::new();
    c.seek(std::io::SeekFrom::Start(0))
        .map_err(|_| Error::new_message("Erroring seeking in-memory cursor"))?;
    c.read_to_end(&mut buffer)
        .map_err(|_| Error::new_message("Erroring reading in-memory cursor"))?;

    api::result_blob(context, buffer.as_slice());
    Ok(())
}
