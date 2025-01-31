mod controllers;

use midir::{Ignore, MidiInput, MidiOutput};
use controllers::*;
use controllers::Channel::*;
use controllers::MasterChannel::*;

fn main() {
    
    let mut midi_out = MidiOutput::new("My test output").unwrap();

    let new_port = &midi_out.ports()[1];
    
    println!(
        "Connecting to port '{}' ...",
        midi_out.port_name(&new_port).unwrap()
    );

    let mut conn_out = midi_out.connect(&new_port, "midir-test").unwrap();

    let mut data: [u8; 10] = [0xF0, 0x43, 0x10, 0x3E, 0x04, 0x00, 0x00, 0x00, 0x00, 0xF7];

    // TODO: properly put data into message
    let fader = FaderControlVal {
        control: FaderControl::Master(Effect2),
        value: 0x00,
    };

    let fader2 = OnControlVal {
        control: OnControl::Channel(CH9),
        value: false,
    };

    let fader_data = fader2.serialize();
    data[5] = fader_data[0];
    data[6] = fader_data[1];
    data[7] = fader_data[2];
    data[8] = fader_data[3];

    for byte in data {
        print!("{:02X} ", byte);
    }

    conn_out.send(&data);

    conn_out.close();
}
