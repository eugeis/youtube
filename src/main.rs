extern crate sanitize_filename;
use clap::Parser;
use rusty_ytdl::Video;

#[tokio::main]
async fn main() {

  let args = Args::parse();

  println!("download to: {}", &args.dir);

  if args.url.contains("/playlist?") {
    download_playlist(&args.url, &args.dir).await;
  } else {
    download_video(&args.url, &args.dir).await;
  }
}

async fn download_video(video_url: &str, dir: &str) {
  let video = Video::new(video_url).unwrap();
  let video_info = &video.get_basic_info().await.unwrap();

  match download_mp3(&video.get_video_id(), &video_info.video_details.title, dir).await {
    Err(e) => {
      println!("Error: {}", e);
    },
    _ => ()
  }
}

async fn download_playlist(playlist_url: &str, dir: &str) {
    use rusty_ytdl::search::{Playlist, PlaylistSearchOptions};

    let playlist = Playlist::get(
        playlist_url,
        Some(&PlaylistSearchOptions {
            limit: 6000,
            fetch_all: false,
            ..Default::default()
        }),
    )
    .await;

    match playlist {
        Ok(x) => {
            println!("Download playlist: {}", x.name);
            for video in x.videos {
                match download_mp3(&video.id, &video.title, dir).await {
                    Err(e) => println!("Error: {}", e),
                    _ => ()
                }
            }
        },
        Err(e) => println!("Error: {}", e),
    }
}

async fn download_mp3(video_id: &str, video_title: &str, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = format!("{}.mp3", sanitize_filename::sanitize(video_title));

    // Or direct download to path
    let path = std::path::Path::new(dir).join(&filename);

    if path.exists() {
        println!("File already exists: {}", filename);
        return Ok(());
    }

    let video = Video::new(video_id).unwrap();

    video.download(path).await.unwrap();

    println!("Downloaded: {}", filename);

    Ok(())
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Url of a YouTube playlist or video
    #[arg(short, long)]
    url: String,

    /// Dir to download to
    #[arg(short, long, default_value = ".")]
    dir: String,
}

