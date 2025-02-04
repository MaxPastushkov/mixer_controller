mod controllers;
mod state_map;
mod controller;

use midir::{MidiOutput};
//use controllers::*;
use actix_web::{post, web, App, HttpServer, HttpResponse};
use actix_cors::Cors;
use bimap::BiHashMap;
use crate::controller::*;
use crate::state_map::init_state_map;

fn send_midi_data(data: [u8; 4]) {
    let midi_out = MidiOutput::new("My test output").unwrap();
    let new_port = &midi_out.ports()[1];
    let mut conn_out = midi_out.connect(&new_port, "midir-test").unwrap();

    let mut message: [u8; 10] = [0xF0, 0x43, 0x10, 0x3E, 0x04, 0x00, 0x00, 0x00, 0x00, 0xF7];
    message[5] = data[0];
    message[6] = data[1];
    message[7] = data[2];
    message[8] = data[3];

    let _ = conn_out.send(&message);
    conn_out.close();
}

// #[post("/fader")]
// async fn update_fader(fader: web::Json<FaderControlVal>) -> HttpResponse {
//     send_midi_data(fader.serialize());
//     HttpResponse::Ok().finish()
// }
//
// #[post("/on")]
// async fn update_on(on: web::Json<OnControlVal>) -> HttpResponse {
//     send_midi_data(on.serialize());
//     HttpResponse::Ok().finish()
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut state_map = BiHashMap::<u16, Address>::new();
    init_state_map(&mut state_map);

    let addr: &u16 = state_map.get_by_right(&Address::BusSend(BusSend::Effect2(Channel::Return1))).unwrap();
    let message: [u8; 4] = [0x10, ((addr & 0b1111111_0000000) >> 7) as u8, (addr & 0b1111111) as u8, 0x7F];
    send_midi_data(message);

    for a in message.iter() {
        println!("{:02X}", a);
    }

    HttpServer::new(|| {

        let cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .allow_any_origin()
            .send_wildcard();

        App::new().wrap(cors)
            // .service(update_fader)
            // .service(update_on)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
