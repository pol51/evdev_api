use text_colorizer::*;
use evdev::{Device, Key, InputEvent, EventType};
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize};

#[derive(Deserialize)]
struct EventParam {
    key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving on http://0.0.0.0:8081 ...");
    HttpServer::new(|| App::new().route("/event", web::post().to(post_event)))
        .bind("0.0.0.0:8081")?
        .run().await
}

fn post_event(event: web::Json<EventParam>) -> HttpResponse {
    use std::str::FromStr;
    
    let device_path = "/dev/input/event3";
    let mut device = match Device::open(device_path) {
        Ok(v) => v,
        Err(e) =>  {
            eprintln!("{} failed to open device '{}': {:?}",
                    "Error:".red().bold(), device_path, e);
            std::process::exit(1);
        }
    };
    
    match Key::from_str(&event.key) {
      Ok(v) => {
        device.send_events(&[
          InputEvent::new_now(EventType::KEY, v.code(), 1),
          InputEvent::new_now(EventType::KEY, v.code(), 0),
          InputEvent::new_now(EventType::SYNCHRONIZATION, 0, 0),          
        ]).unwrap();
        eprintln!("{} key pressed '{}'",
                "Success:".green().bold(), event.key);
        HttpResponse::Ok()
          .content_type("text/html")
          .body(format!("Key pressed: {}", event.key))
      },
      Err(e) => {
        eprintln!("{} invalid key '{}': {:?}",
                "Error:".red().bold(), event.key, e);
        HttpResponse::BadRequest()
          .content_type("text/html")
          .body(format!("invalid key: {}", event.key))
      }
    }
}



