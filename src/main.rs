use text_colorizer::*;
use evdev::{Device, Key, InputEvent, EventType};
use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest};
use serde::{Deserialize};
use std::sync::Mutex;
use std::env;

#[derive(Deserialize)]
struct EventParam {
    key: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving on http://0.0.0.0:8081 ...");

    HttpServer::new(move || {
        let args: Vec<String> = env::args().skip(1).collect();
        let input_device_path = match args.len() {
            0 => "/dev/input/event3",
            _ => &args[0]
        };
        
        let input_device = match Device::open(input_device_path) {
            Ok(v) => Mutex::new(v),
            Err(e) =>  {
                eprintln!("{} failed to open device '{}': {:?}",
                          "Error:".red().bold(), input_device_path, e);
                std::process::exit(1);
            }
        };

        App::new()
            .app_data(web::Data::new(input_device))
            .route("/event", web::post().to(post_event))
    })
        .bind("0.0.0.0:8081")?
        .run().await
}

fn post_event(req: HttpRequest, event: web::Json<EventParam>) -> HttpResponse {
    use std::str::FromStr;

    let mut device = req.app_data::<web::Data<Mutex<Device>>>().unwrap().lock().unwrap();

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



