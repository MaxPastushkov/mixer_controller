mod state_map;
mod controller;
mod broadcast;

use midir::{MidiOutput, MidiInput};
//use controllers::*;
use actix_web::{post, get, middleware::Logger, web, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use bimap::BiHashMap;
use crate::controller::*;
use crate::state_map::init_state_map;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use serde::Serialize;
use crate::broadcast::Broadcaster;

static STATE_MAP: Lazy<Mutex<BiHashMap<u16, Address>>> = Lazy::new(|| Mutex::new(BiHashMap::new()));

fn send_midi_data(data: [u8; 4]) {
    let midi_out = MidiOutput::new("midir").unwrap();
    let out_port = &midi_out.ports()[1];
    let mut conn_out = midi_out.connect(&out_port, "midir-test").unwrap();

    let mut message: [u8; 10] = [0xF0, 0x43, 0x10, 0x3E, 0x04, 0x00, 0x00, 0x00, 0x00, 0xF7];
    message[5] = data[0];
    message[6] = data[1];
    message[7] = data[2];
    message[8] = data[3];

    let _ = conn_out.send(&message);
    conn_out.close();
}

#[post("/u7")]
async fn update_u7_value(body: web::Json<U7ControlVal>) -> HttpResponse {

    let mut data: [u8; 4] = [0x10, 0x00, 0x00, 0x00];

    let addr: u16 = *STATE_MAP.lock().unwrap().get_by_right(&body.control).unwrap();
    data[1] = ((addr >> 7) & 0x7F) as u8; // Upper 7 bits
    data[2] = (addr & 0x7F) as u8; // Lower 7 bits
    data[3] = body.value;

    send_midi_data(data);

    HttpResponse::Ok().finish()
}

#[post("/bit")]
async fn update_bit_value(body: web::Json<BitControlVal>) -> HttpResponse {

    let mut data: [u8; 4] = [0x40, 0x00, 0x00, 0x00];

    let (group, id) = body.control.to_address();
    data[1] = ((group >> 7) & 0x7F) as u8; // Upper 7 bits
    data[2] = (group & 0x7F) as u8; // Lower 7 bits
    data[3] = id | (if body.value { 0b1000 } else { 0b0000 });

    send_midi_data(data);

    HttpResponse::Ok().finish()
}

#[get("/events")]
async fn event_stream(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    broadcaster.new_client().await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let broadcaster = Broadcaster::create();

    init_state_map();

    let midi_in = MidiInput::new("midir").unwrap();
    let in_port = &midi_in.ports()[1];

    let _conn_in = midi_in.connect(
        &in_port,
        "midir-in",
        |stamp, message, _| {
            let _ = broadcaster.broadcast("Test");
        },
        (),
    );


    // let addr: u16 = *STATE_MAP.lock().unwrap().get_by_right(&Address::EqControl(EqControl::Param {
    //     channel: EqChannel::StereoOut,
    //     band: EqBand::LoMid,
    //     knob: EqKnob::F,
    // })).unwrap();
    //
    // let message: [u8; 4] = [0x10, ((addr & 0b1111111_0000000) >> 7) as u8, (addr & 0b1111111) as u8, 0x77];
    // send_midi_data(message);
    //
    // for a in message.iter() {
    //     println!("{:02X}", a);
    // }
    //
    // let tmp = U7ControlVal {
    //     control: Address::BusSend(BusSend::StereoOut(Channel::CH1)),
    //     value: 0x7F,
    // };
    //
    // println!("{}", serde_json::to_string(&tmp)?);
    //
    // println!("{}", serde_json::to_string(&Address::EqControl(EqControl::Param {
    //     channel: EqChannel::StereoOut,
    //     band: EqBand::LoMid,
    //     knob: EqKnob::F,
    // }))?);

    HttpServer::new(|| {

        let cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .allow_any_origin()
            .send_wildcard();

        App::new().wrap(cors)
            .app_data(web::Data::from(Arc::clone(&broadcaster)))
            .service(update_u7_value)
            .service(update_bit_value)
            .service(event_stream)
            .wrap(Logger::default)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
