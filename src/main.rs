use std::{io::Cursor, net::SocketAddr};

use axum::{extract::ConnectInfo, http::header, response::IntoResponse, routing::get, Router};
use image::{ImageBuffer, ImageOutputFormat, Rgb};

const WIDTH: u32 = 256;
const HEIGHT: u32 = WIDTH;
const BACKGROUND_COLOR: Rgb<u8> = Rgb([177, 98, 134]);

async fn avatar(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    let _text = format!("Hello, {}!", addr.ip());
    let img = ImageBuffer::from_pixel(WIDTH, HEIGHT, BACKGROUND_COLOR);

    let mut cursor = Cursor::new(vec![]);
    img.write_to(&mut cursor, ImageOutputFormat::Png).unwrap();

    ([(header::CONTENT_TYPE, "image/png")], cursor.into_inner())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/avatar.png", get(avatar));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let make_service = app.into_make_service_with_connect_info::<SocketAddr>();
    axum::serve(listener, make_service).await.unwrap();
}
