use crate::models::post::Post;
use crate::models::worker::Message;
use ffmpeg_cli::{FfmpegBuilder, File, Parameter};
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use std::process::Stdio;
use tokio::runtime::Handle;

pub fn process_video(connection: &mut PoolConnection<Postgres>, wm: Message) -> Result<(), ()> {
    let mut dest = String::from("static/");
    dest.push_str(&wm.dest_filename);
    let builder = FfmpegBuilder::new()
        .stderr(Stdio::piped())
        .option(Parameter::Single("nostdin"))
        .option(Parameter::Single("y"))
        .input(File::new(&wm.orig_filename))
        .output(
            File::new(&dest)
                .option(Parameter::KeyValue("vcodec", "libx265"))
                .option(Parameter::KeyValue("crf", "28")),
        );
    let make_permanent = async {
        let ffmpeg = builder.run().await.unwrap();
        let _ = ffmpeg.process.wait_with_output().unwrap();
        let mut display_path = String::from("/assets/");
        display_path.push_str(&wm.dest_filename);
        Post::make_permanent(connection, &wm.uuid, &display_path).await
    };

    let handle = Handle::current();
    Ok(handle
        .block_on(make_permanent)
        .map(|_| ())
        .map_err(|_| ())?)
}
