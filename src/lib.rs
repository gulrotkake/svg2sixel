use resvg::tiny_skia::{self, Pixmap};
use resvg::usvg::{Options, Tree};
use sixel_bytes::SixelError;

pub fn svg2sixel(svg: &str) -> Result<String, SixelError> {
    let mut options = Options::default();
    options.fontdb_mut().load_system_fonts();

    let rtree = Tree::from_str(svg, &options).unwrap();
    let pixmap_size = rtree.size();
    let mut pixmap = Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32).unwrap();

    resvg::render(
        &rtree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );

    sixel_bytes::sixel_string(
        pixmap.data(),
        pixmap_size.width() as _,
        pixmap_size.height() as _,
        sixel_bytes::PixelFormat::RGBA8888,
        sixel_bytes::DiffusionMethod::Stucki,
    )
}
