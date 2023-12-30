use std::{io::Cursor, net::SocketAddr, sync::OnceLock};

use axum::{
    extract::ConnectInfo,
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use image::{ImageBuffer, ImageOutputFormat, Rgb};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

const WIDTH: u32 = 256;
const HEIGHT: u32 = WIDTH;
const BACKGROUND_COLOR: Rgb<u8> = Rgb([177, 98, 134]);

const X: i32 = 8;
const Y: i32 = 96;
const SCALE: Scale = Scale { x: 32.0, y: 32.0 };
const TEXT_COLOR: Rgb<u8> = Rgb([235, 219, 178]);
const FONT_DATA: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fonts/UbuntuMono-R.ttf"
));

fn font() -> &'static Font<'static> {
    static FONT: OnceLock<Font> = OnceLock::new();
    FONT.get_or_init(|| Font::try_from_bytes(FONT_DATA).expect("Built-in font data was invalid"))
}

#[derive(Debug, thiserror::Error)]
#[error("Failed to generate image: {0}")]
struct AvatarError(#[from] image::ImageError);

impl IntoResponse for AvatarError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

async fn avatar(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, AvatarError> {
    // Wow, IPv6 causes a lot of headache.
    let ip = addr.ip().to_canonical();
    let mut img = ImageBuffer::from_pixel(WIDTH, HEIGHT, BACKGROUND_COLOR);

    draw_text_mut(&mut img, TEXT_COLOR, X, Y, SCALE, font(), "Hello,");
    let y = Y + SCALE.y as i32;
    draw_text_mut(&mut img, TEXT_COLOR, X, y, SCALE, font(), &format!("{ip}!"));

    let mut cursor = Cursor::new(vec![]);
    img.write_to(&mut cursor, ImageOutputFormat::Png)?;

    Ok(([(header::CONTENT_TYPE, "image/png")], cursor.into_inner()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/avatar.png", get(avatar));

    let listener = tokio::net::TcpListener::bind("[::]:3000").await?;
    let make_service = app.into_make_service_with_connect_info::<SocketAddr>();
    axum::serve(listener, make_service).await?;
    Ok(())
}
