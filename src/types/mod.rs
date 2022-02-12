mod render_space;
mod svg_description;

pub use render_space::RenderSpace;
pub use svg_description::SvgDescription;

pub type Result<T> = color_eyre::Result<T>;
