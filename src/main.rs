use std::time::Instant;
use std::{
    fs,
    os::unix::process,
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
const FRAMES: i32 = 6572;
use macroquad::{
    audio::{PlaySoundParams, Sound, load_sound, play_sound},
    prelude::{scene::clear, *},
};
const FRAME_DURATION: f32 = 1.0 / 24.; // 30 FPS
fn conf() -> Conf {
    Conf {
        window_height: 360,
        window_width: 480,
        window_resizable: false,
        window_title: "Bad Apple".to_string(),
        ..Default::default()
    }
}
#[macroquad::main(conf)]
async fn main() {
    let song = load_sound("bad_apple.wav").await.unwrap();
    let mut images_paths = fs::read_dir("frames/")
        .unwrap()
        .map(|x| x.unwrap().path())
        .collect::<Vec<PathBuf>>();
    images_paths.sort_by_key(|p| {
        p.file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.split('_').nth(1))
            .and_then(|n| n.parse::<u32>().ok())
            .unwrap_or(0)
    });

    let mut idx = 0;
    play_sound(&song, PlaySoundParams::default());

    loop {
        let frame_start = Instant::now(); // track frame start

        clear_background(WHITE);

        let texture = Texture2D::from_file_with_format(
            fs::read(&images_paths[idx]).unwrap().as_slice(),
            Some(ImageFormat::Png),
        );
        draw_texture(&texture, 0., 0., WHITE);
        idx += 1;
        if idx >= images_paths.len() {
            break;
        }

        next_frame().await;
        println!("{}/{}", idx, FRAMES);
        // enforce 30 FPS
        let elapsed = frame_start.elapsed();
        if elapsed.as_secs_f32() < FRAME_DURATION {
            std::thread::sleep(Duration::from_secs_f32(
                FRAME_DURATION - elapsed.as_secs_f32(),
            ));
        }
    }
}
