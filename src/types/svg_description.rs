use rocket::fs::TempFile;
use std::collections::HashMap;

/// What data is required to render an SVG
#[derive(FromForm)]
pub struct SvgDescription<'a> {
    /// Raw svg content that will be used during render
    pub svg: TempFile<'a>,

    /// Resources are files that will be referred to during render.
    /// if a ttf, ttc, otc, or otf is provided it will be loaded for use.
    pub resources: HashMap<String, TempFile<'a>>,
}
