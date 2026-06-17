use macroquad::prelude::*;
use std::net::UdpSocket;
use serde::Deserialize;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::time::Instant; // Zamanlama için

#[derive(Deserialize, Debug, Clone)] struct Telemetry { targets: Vec<Target> }
#[derive(Deserialize, Debug, Clone)] struct Target { 
    id: i32, mesafe: f32, vx: f32, vy: f32, bbox: BBox 
}
#[derive(Deserialize, Debug, Clone)] struct BBox { x: f32, y: f32, w: f32, h: f32 }

#[macroquad::main("C2-SYSTEM | BATTLEFIELD MANAGEMENT")]
async fn main() {
    let socket = UdpSocket::bind("127.0.0.1:9999").expect("Soket hatası!");
    socket.set_nonblocking(true).unwrap();
    
    let (_stream, stream_handle) = OutputStream::try_default().expect("Ses cihazı yok!");
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let mut targets: Vec<Target> = Vec::new();
    let mut ammo = 30; // Başlangıç mühimmatı
    let mut last_shot_time = Instant::now(); // Atış sıklığı kontrolü

    loop {
        clear_background(BLACK);

        let mut buf = [0; 8192];
        if let Ok((amt, _)) = socket.recv_from(&mut buf) {
            if let Ok(data) = serde_json::from_slice::<Telemetry>(&buf[..amt]) {
                targets = data.targets;
            }
        }

        draw_line(400.0, 0.0, 400.0, 600.0, 1.0, GREEN);
        draw_line(0.0, 300.0, 800.0, 300.0, 1.0, GREEN);
        draw_text(&format!("AMMO: {}/30", ammo), 20.0, 50.0, 30.0, WHITE);

        if ammo == 0 {
            draw_text("RELOAD REQUIRED - PRESS [R]", 300.0, 300.0, 30.0, RED);
            if is_key_pressed(KeyCode::R) { ammo = 30; }
        }

        if let Some(t) = targets.iter().min_by(|a, b| a.mesafe.partial_cmp(&b.mesafe).unwrap()) {
            let (x, y, w, h) = (t.bbox.x * 800.0, t.bbox.y * 600.0, t.bbox.w * 800.0, t.bbox.h * 600.0);
            draw_rectangle_lines(x-w/2.0, y-h/2.0, w, h, 2.0, ORANGE);

            let lead_time = 0.5;
            let future_x = x + (t.vx * lead_time * 100.0);
            let future_y = y + (t.vy * lead_time * 100.0);

            draw_line(future_x-10.0, future_y-10.0, future_x+10.0, future_y+10.0, 2.0, RED);
            draw_line(future_x-10.0, future_y+10.0, future_x+10.0, future_y-10.0, 2.0, RED);

            // OTONOM TETİKLEYİCİ
            let dist = ((x - future_x).powi(2) + (y - future_y).powi(2)).sqrt();
            if dist < 30.0 && ammo > 0 && last_shot_time.elapsed().as_millis() > 500 {
                draw_text("AUTO-ENGAGING...", x-w/2.0, y-h/2.0-30.0, 20.0, RED);
                if let Ok(file) = File::open("assets/shoot.wav") {
                    sink.append(Decoder::new(BufReader::new(file)).unwrap());
                    ammo -= 1;
                    last_shot_time = Instant::now();
                }
            }
        }
        next_frame().await
    }
}