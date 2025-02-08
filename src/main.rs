mod state_map;
mod controller;
mod broadcast;

use midir::{MidiOutput, MidiInput};
//use controllers::*;
use actix_web::{post, get, middleware::Logger, web, App, HttpServer, HttpResponse, Responder, HttpRequest};
use actix_cors::Cors;
use bimap::BiHashMap;
use crate::controller::*;
use crate::state_map::init_state_map;
use std::sync::{Arc, Mutex, LazyLock};
use crate::broadcast::Broadcaster;
use futures::executor;

const MIDI_PORT: usize = 1;

static STATE_MAP: LazyLock<Mutex<BiHashMap<u16, Address>>> = LazyLock::new(|| Mutex::new(BiHashMap::new()));

fn send_midi_data(data: &[u8]) {
    let midi_out = MidiOutput::new("midir").unwrap();
    let out_port = &midi_out.ports()[MIDI_PORT];
    let mut conn_out = midi_out.connect(&out_port, "midir-test").unwrap();

    if data.len() == 4 {
        let _ = conn_out.send(&[0xF0, 0x43, 0x10, 0x3E, 0x04, data[0], data[1], data[2], data[3], 0xF7]);
    } else {
        let _ = conn_out.send(data);
    }

    conn_out.close();
}

#[post("/u7")]
async fn update_u7_value(body: web::Json<U7ControlVal>, broadcaster: web::Data<Broadcaster>) -> HttpResponse {

    let mut data: [u8; 4] = [0x10, 0x00, 0x00, 0x00];

    let addr: u16 = *STATE_MAP.lock().unwrap().get_by_right(&body.control).unwrap();
    data[1] = ((addr >> 7) & 0x7F) as u8; // Upper 7 bits
    data[2] = (addr & 0x7F) as u8; // Lower 7 bits
    data[3] = body.value;

    send_midi_data(&data);

    broadcaster.broadcast(serde_json::to_string(&body).unwrap().as_str()).await;

    HttpResponse::Ok().finish()
}

#[post("/bit")]
async fn update_bit_value(body: web::Json<BitControlVal>, broadcaster: web::Data<Broadcaster>) -> HttpResponse {

    let mut data: [u8; 4] = [0x40, 0x00, 0x00, 0x00];

    let (group, id) = body.control.to_address();
    data[1] = ((group >> 7) & 0x7F) as u8; // Upper 7 bits
    data[2] = (group & 0x7F) as u8; // Lower 7 bits
    data[3] = id | (if body.value { 0b1000 } else { 0b0000 });

    send_midi_data(&data);

    broadcaster.broadcast(serde_json::to_string(&body).unwrap().as_str()).await;

    HttpResponse::Ok().finish()
}

#[get("/events")]
async fn event_stream(req: HttpRequest, broadcaster: web::Data<Broadcaster>) -> impl Responder {
    if let Some(val) = req.peer_addr() {
        println!("New SSE client: {:?}", val.ip());
    }

    // Get initial state
    send_midi_data(&[0xF0, 0x43, 0x20, 0x7E, 0x4C, 0x4D, 0x20, 0x20, 0x38, 0x42, 0x33, 0x34, 0x4D, 0x7F, 0xF7]);

    broadcaster.new_client().await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let broadcaster = Broadcaster::create();

    init_state_map();

    let midi_in = MidiInput::new("midir").unwrap();
    let in_port = &midi_in.ports()[MIDI_PORT];

    let broadcaster_ptr = Arc::clone(&broadcaster);

    let _conn_in = midi_in.connect(
        &in_port,
        "midir-in",
        move |_, message, _| {

            println!("Received MIDI Data: {:02X?}", message);

            if message.len() == 10 {
                let output: String = match message[5] {
                    0x10 => {
                        let addr: u16 = ((message[6] as u16) << 7) | ((message[7] as u16) & 0x7F);
                        let control: Address = *STATE_MAP.lock().unwrap().get_by_left(&addr).unwrap();
                        let obj = U7ControlVal {
                            control,
                            value: message[8],
                            client_id: String::new(),
                        };
                        serde_json::to_string(&obj).unwrap()
                    },
                    0x40 => {
                        let group: u16 = ((message[6] as u16) << 7) | ((message[7] as u16) & 0x7F);
                        let bits: u8 = message[8] & 0b0111;
                        let value: bool = (message[8] & 0b1000) > 0;
                        let obj = BitControlVal {
                            control: BitControl::from_address((group, bits)).unwrap(),
                            value,
                            client_id: String::new(),
                        };
                        serde_json::to_string(&obj).unwrap()
                    }
                    _ => "Internal error".to_string()
                };

                executor::block_on(broadcaster_ptr.broadcast(output.as_str()));

            } else if message.len() >= 30 && message[3] == 0x7E && message[14] == 0x4D {
                // Response from bulk scene data
                for i in 0..(message.len()-32)/2 {
                    let value: u8 = message[i*2 + 30] << 4 | message[i*2 + 31];
                    if let Some(address) = STATE_MAP.lock().unwrap().get_by_left(&(i as u16 + 0x0C)) {
                        let obj = U7ControlVal {
                            control: *address,
                            value,
                            client_id: String::new(),
                        };
                        executor::block_on(broadcaster_ptr.broadcast(serde_json::to_string(&obj).unwrap().as_str()));

                    } else if BitControl::from_address((i as u16 + 0x0C, 0)).is_some() {

                        for j in 0u8..=0b111 {
                            if let Some(control) = BitControl::from_address((i as u16 + 0x0C, j)) {
                                let obj = BitControlVal {
                                    control,
                                    value: value & (1 << j) != 0,
                                    client_id: String::new(),
                                };
                                executor::block_on(broadcaster_ptr.broadcast(serde_json::to_string(&obj).unwrap().as_str()));
                                // TODO: Make this line only run once
                            }
                        }
                    }
                }
            }
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

    HttpServer::new(move || {

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
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
            .wrap(Logger::default())
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
