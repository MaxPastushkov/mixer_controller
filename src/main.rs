mod controllers;

use midir::{MidiOutput};
use controllers::*;
use actix_web::{post, web, App, HttpServer, HttpResponse};
use actix_cors::Cors;

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

#[post("/fader")]
async fn update_fader(fader: web::Json<FaderControlVal>) -> HttpResponse {
    send_midi_data(fader.serialize());
    HttpResponse::Ok().finish()
}

#[post("/on")]
async fn update_on(on: web::Json<OnControlVal>) -> HttpResponse {
    send_midi_data(on.serialize());
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {

        let cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .allow_any_origin()
            .send_wildcard();

        App::new().wrap(cors)
            .service(update_fader)
            .service(update_on)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// fn main() {
//
//     let midi_out = MidiOutput::new("My test output").unwrap();
//
//     let new_port = &midi_out.ports()[1];
//
//     println!(
//         "Connecting to port '{}' ...",
//         midi_out.port_name(&new_port).unwrap()
//     );
//
//     let mut conn_out = midi_out.connect(&new_port, "midir-test").unwrap();
//
//     let mut data: [u8; 10] = [0xF0, 0x43, 0x10, 0x3E, 0x04, 0x00, 0x00, 0x00, 0x00, 0xF7];
//
//     let _fader = FaderControlVal::new(FaderControl::Master(Effect2), 0);
//
//     let _fader2 = OnControlVal::new(OnControl::Channel(CH9), false);
//
//     let eq = EqControlVal {
//         control: EqControl::Param {
//             channel: EqChannel::CH1,
//             band: EqBand::High(EqSpecialMode::Normal),
//             knob: EqKnob::G
//         },
//         value: 0x7F
//     };
//
//     println!("{}", serde_json::to_string(&eq).unwrap());
//
//     let fader_data = eq.serialize();
//     data[5] = fader_data[0];
//     data[6] = fader_data[1];
//     data[7] = fader_data[2];
//     data[8] = fader_data[3];
//
//     //*TryInto::<&mut[u8; 4]>::try_into((&mut data[5..=8])).unwrap() = fader_data;
//
//     for byte in data {
//         print!("{:02X} ", byte);
//     }
//
//     let _ = conn_out.send(&data);
//
//     conn_out.close();
// }
