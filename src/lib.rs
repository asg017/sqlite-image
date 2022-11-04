pub mod img;
pub mod img_as;
pub mod meta;
pub mod util;
pub use crate::meta::{img_debug, img_version};

use sqlite3_loadable::{
    errors::Result,
    scalar::define_scalar_function,
    sqlite3,
    sqlite3_entrypoint,
    sqlite3_imports,
    //table::{define_table_function, define_virtual_table},
};

sqlite3_imports!();

#[sqlite3_entrypoint]
pub fn sqlite3_img_init(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(db, "img_version", 0, img_version)?;
    define_scalar_function(db, "img_debug", 0, img_debug)?;

    define_scalar_function(db, "img_replace", 4, img::img_replace)?;
    define_scalar_function(db, "img_overlay_all", -1, img::img_overlay_all)?;

    define_scalar_function(db, "img_width", 1, img::img_width)?;

    define_scalar_function(db, "img_blur", 2, img::img_blur)?;
    define_scalar_function(db, "img_grayscale", 1, img::img_grayscale)?;

    define_scalar_function(db, "img_thumbnail", 3, img::img_thumbnail)?;
    define_scalar_function(db, "img_resize", 3, img::img_resize)?;
    define_scalar_function(db, "img_resize", 4, img::img_resize)?;

    define_scalar_function(db, "img_flip_vertical", 1, img::img_flip_vertical)?;
    define_scalar_function(db, "img_flip_horizontal", 1, img::img_flip_horizontal)?;

    define_scalar_function(db, "img_rotate90", 1, img::img_rotate90)?;
    define_scalar_function(db, "img_rotate180", 1, img::img_rotate180)?;
    define_scalar_function(db, "img_rotate270", 1, img::img_rotate270)?;

    define_scalar_function(db, "img_solid_rgb", 5, img::img_solid_rgb)?;
    define_scalar_function(db, "img_rect", 2, img::img_rect)?;

    define_scalar_function(db, "img_crop", 5, img::img_crop)?;
    define_scalar_function(db, "img_crop_png", 5, img::img_crop_png)?;
    define_scalar_function(db, "img_crop_jpeg", 5, img::img_crop_jpeg)?;

    define_scalar_function(db, "img_as_png", 1, img_as::img_as_png)?;
    define_scalar_function(db, "img_as_jpeg", 2, img_as::img_as_jpeg)?;
    define_scalar_function(db, "img_as_webp", 1, img_as::img_as_webp)?;
    Ok(())
}
