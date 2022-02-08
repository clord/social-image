use rocket::http::Status;
use tiny_skia::{Pixmap, Transform};
use usvg::{FitTo, Options, Size, Tree};

pub fn svg_to_png(file: &[u8], png_path: &std::path::Path) -> std::result::Result<(), Status> {
    let mut opt = Options::default();
    opt.fontdb.load_system_fonts();
    opt.resources_dir = png_path.parent().map(|x| x.to_owned());

    if let Some(size) = Size::new(1080f64, 566f64) {
        opt.default_size = size;
    }

    match Tree::from_data(file, &opt.to_ref()) {
        Ok(rtree) => {
            let pixmap_size = rtree.svg_node().size.to_screen_size();

            match Pixmap::new(pixmap_size.width(), pixmap_size.height()) {
                None => Err(Status::BadRequest),
                Some(mut pixmap) => {
                    if resvg::render(
                        &rtree,
                        FitTo::Original,
                        Transform::default(),
                        pixmap.as_mut(),
                    )
                    .is_none()
                    {
                        error!("failed to render SVG");
                        return Err(Status::NotAcceptable);
                    }

                    if let Err(e) = pixmap.save_png(png_path) {
                        error!("Failed to save PNG: {e:?}");
                        return Err(Status::BadRequest);
                    }

                    Ok(())
                }
            }
        }
        Err(e) => {
            error!("while decoding file as SVG: {e:?}");
            Err(Status::BadRequest)
        }
    }
}
