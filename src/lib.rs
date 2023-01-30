pub mod img;
pub mod img_as;
pub mod meta;
pub mod util;
pub use crate::meta::{img_debug, img_version};

use sqlite_loadable::prelude::*;
use sqlite_loadable::{define_scalar_function, FunctionFlags, Result};

#[sqlite_entrypoint]
pub fn sqlite3_img_init(db: *mut sqlite3) -> Result<()> {
    let f = FunctionFlags::DETERMINISTIC;
    define_scalar_function(db, "img_version", 0, img_version, f)?;
    define_scalar_function(db, "img_debug", 0, img_debug, f)?;

    define_scalar_function(db, "img_replace", 4, img::img_replace, f)?;
    define_scalar_function(db, "img_overlay_all", -1, img::img_overlay_all, f)?;

    define_scalar_function(db, "img_width", 1, img::img_width, f)?;

    define_scalar_function(db, "img_blur", 2, img::img_blur, f)?;
    define_scalar_function(db, "img_grayscale", 1, img::img_grayscale, f)?;

    define_scalar_function(db, "img_thumbnail", 3, img::img_thumbnail, f)?;
    define_scalar_function(db, "img_resize", 3, img::img_resize, f)?;
    define_scalar_function(db, "img_resize", 4, img::img_resize, f)?;

    define_scalar_function(db, "img_flip_vertical", 1, img::img_flip_vertical, f)?;
    define_scalar_function(db, "img_flip_horizontal", 1, img::img_flip_horizontal, f)?;

    define_scalar_function(db, "img_rotate90", 1, img::img_rotate90, f)?;
    define_scalar_function(db, "img_rotate180", 1, img::img_rotate180, f)?;
    define_scalar_function(db, "img_rotate270", 1, img::img_rotate270, f)?;

    define_scalar_function(db, "img_solid_rgb", 5, img::img_solid_rgb, f)?;
    define_scalar_function(db, "img_rect", 2, img::img_rect, f)?;

    define_scalar_function(db, "img_crop", 5, img::img_crop, f)?;
    define_scalar_function(db, "img_crop_png", 5, img::img_crop_png, f)?;
    define_scalar_function(db, "img_crop_jpeg", 5, img::img_crop_jpeg, f)?;

    define_scalar_function(db, "img_as_png", 1, img_as::img_as_png, f)?;
    define_scalar_function(db, "img_as_jpeg", 2, img_as::img_as_jpeg, f)?;
    define_scalar_function(db, "img_as_webp", 1, img_as::img_as_webp, f)?;
    Ok(())
}
