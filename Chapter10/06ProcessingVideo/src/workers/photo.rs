use crate::models::post::Post;
use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageEncoder};
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tokio::runtime::Handle;

pub fn process_photo(
    connection: &mut PoolConnection<Postgres>,
    uuid: String,
    orig_path: String,
    dest_filename: String,
) -> Result<(), ()> {
    let orig_file = File::open(orig_path).map_err(|_| ())?;
    let file_reader = BufReader::new(orig_file);
    let image: DynamicImage = ImageReader::new(file_reader)
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    let dest_path = Path::new(rocket::fs::relative!("static")).join(&dest_filename);
    let mut file_writer = File::create(dest_path).map_err(|_| ())?;
    JpegEncoder::new_with_quality(&mut file_writer, 75)
        .write_image(
            image.as_bytes(),
            image.width(),
            image.height(),
            image.color(),
        )
        .map_err(|_| ())?;

    let mut display_path = String::from("/assets/");
    display_path.push_str(&dest_filename);
    let make_permanent = async {
        let res = Post::make_permanent(connection, &uuid, &display_path).await;
        res
    };
    let handle = Handle::current();
    Ok(handle
        .block_on(make_permanent)
        .map(|_| ())
        .map_err(|_| ())?)
}
