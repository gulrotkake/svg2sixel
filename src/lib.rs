use core::fmt;

use kitty_image::{Action, ActionTransmission, Command, Format, Medium, WrappedCommand};
use resvg::tiny_skia::{self, Pixmap};
use resvg::usvg::{Options, Tree};
use sixel_bytes::SixelError;

#[derive(Debug)]
pub enum Svg2SixelError {
    Usvg(usvg::Error),
    Sixel(SixelError),
    BadSize,
}

impl fmt::Display for Svg2SixelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Svg2SixelError::Usvg(err) => write!(f, "usvg error: {0}", err),
            Svg2SixelError::Sixel(err) => write!(f, "sixel error: {0}", err),
            Svg2SixelError::BadSize => write!(f, "Bad svg pixmap size"),
        }
    }
}

pub fn svg2kitty(svg: &str) -> Result<String, Svg2SixelError> {
    let mut options = Options::default();
    options.fontdb_mut().load_system_fonts();

    let rtree = Tree::from_str(svg, &options).map_err(|err| Svg2SixelError::Usvg(err))?;
    let pixmap_size = rtree.size();
    let mut pixmap = Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32)
        .ok_or(Svg2SixelError::BadSize)?;

    resvg::render(
        &rtree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );

    let action = Action::TransmitAndDisplay(
        ActionTransmission {
            format: Format::Rgba32,
            medium: Medium::Direct,
            width: pixmap.width(),
            height: pixmap.height(),
            ..Default::default()
        },
        kitty_image::ActionPut {
            move_cursor: true,
            ..Default::default()
        },
    );
    let mut command = Command::new(action);
    command.payload = pixmap.data().into();
    Ok(WrappedCommand::new(command).to_string())
}

pub fn svg2sixel(svg: &str) -> Result<String, Svg2SixelError> {
    let mut options = Options::default();
    options.fontdb_mut().load_system_fonts();

    let rtree = Tree::from_str(svg, &options).map_err(Svg2SixelError::Usvg)?;
    let pixmap_size = rtree.size();
    let mut pixmap = Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32)
        .ok_or(Svg2SixelError::BadSize)?;

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
    .map_err(Svg2SixelError::Sixel)
}
