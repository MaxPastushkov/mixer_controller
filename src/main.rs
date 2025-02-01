mod controllers;

use midir::{MidiOutput};
use controllers::*;
use controllers::Channel::*;
use controllers::MasterChannel::*;

fn main() {
    
    let midi_out = MidiOutput::new("My test output").unwrap();

    let new_port = &midi_out.ports()[1];
    
    println!(
        "Connecting to port '{}' ...",
        midi_out.port_name(&new_port).unwrap()
    );

    let mut conn_out = midi_out.connect(&new_port, "midir-test").unwrap();

    let mut data: [u8; 10] = [0xF0, 0x43, 0x10, 0x3E, 0x04, 0x00, 0x00, 0x00, 0x00, 0xF7];

    let _fader = FaderControlVal::new(FaderControl::Master(Effect2), 0);

    let _fader2 = OnControlVal::new(OnControl::Channel(CH9), false);

    let eq = EqControlVal {
        control: EqControl::Param {
            channel: EqChannel::CH1,
            band: EqBand::High(EqSpecialMode::Normal),
            knob: EqKnob::G
        },
        value: 0x7F
    };

    println!("{}", serde_json::to_string(&eq).unwrap());

    let fader_data = eq.serialize();
    data[5] = fader_data[0];
    data[6] = fader_data[1];
    data[7] = fader_data[2];
    data[8] = fader_data[3];

    //*TryInto::<&mut[u8; 4]>::try_into((&mut data[5..=8])).unwrap() = fader_data;

    for byte in data {
        print!("{:02X} ", byte);
    }

    let _ = conn_out.send(&data);

    conn_out.close();
}
