use crate::controller::{Address, Channel, BusSend, EqChannel, EqBand, EqKnob, Bus};
use crate::STATE_MAP;

pub fn init_state_map() {

    let mut map = STATE_MAP.lock().unwrap();

    for i in 0u8..=15 {
        map.insert((0x0C + i) as u16, Address::BusSend(BusSend::StereoOut(Channel::from_u8(i).unwrap())));

        if i <= Channel::CH12 as u8 {
            // Channels 1-12 for effects are normal, the rest are not
            map.insert((0x57 + i) as u16, Address::BusSend(BusSend::Effect1(Channel::from_u8(i).unwrap())));
            map.insert((0x63 + i) as u16, Address::BusSend(BusSend::Effect2(Channel::from_u8(i).unwrap())));

            map.insert((0x27 + i) as u16, Address::BusSend(BusSend::Aux1(Channel::from_u8(i).unwrap())));
            map.insert((0x33 + i) as u16, Address::BusSend(BusSend::Aux2(Channel::from_u8(i).unwrap())));
            map.insert((0x3F + i) as u16, Address::BusSend(BusSend::Aux3(Channel::from_u8(i).unwrap())));
            map.insert((0x4B + i) as u16, Address::BusSend(BusSend::Aux4(Channel::from_u8(i).unwrap())));
        }
    }

    // Stereo sends to auxes are different for some reason
    map.insert(0x6F, Address::BusSend(BusSend::Aux1(Channel::CH1314)));
    map.insert(0x70, Address::BusSend(BusSend::Aux1(Channel::CH1516)));
    map.insert(0x71, Address::BusSend(BusSend::Aux2(Channel::CH1314)));
    map.insert(0x72, Address::BusSend(BusSend::Aux2(Channel::CH1516)));
    map.insert(0x73, Address::BusSend(BusSend::Aux3(Channel::CH1314)));
    map.insert(0x74, Address::BusSend(BusSend::Aux3(Channel::CH1516)));
    map.insert(0x75, Address::BusSend(BusSend::Aux4(Channel::CH1314)));
    map.insert(0x76, Address::BusSend(BusSend::Aux4(Channel::CH1516)));
    map.insert(0x7B, Address::BusSend(BusSend::Aux1(Channel::Return1)));
    map.insert(0x7C, Address::BusSend(BusSend::Aux1(Channel::Return2)));
    map.insert(0x7D, Address::BusSend(BusSend::Aux2(Channel::Return1)));
    map.insert(0x7E, Address::BusSend(BusSend::Aux2(Channel::Return2)));
    map.insert(0x7F, Address::BusSend(BusSend::Aux3(Channel::Return1)));
    map.insert(0x80, Address::BusSend(BusSend::Aux3(Channel::Return2)));
    map.insert(0x81, Address::BusSend(BusSend::Aux4(Channel::Return1)));
    map.insert(0x82, Address::BusSend(BusSend::Aux4(Channel::Return2)));

    // Special cases for effects sends
    map.insert(0x77, Address::BusSend(BusSend::Effect1(Channel::CH1314)));
    map.insert(0x78, Address::BusSend(BusSend::Effect1(Channel::CH1516)));
    map.insert(0x79, Address::BusSend(BusSend::Effect2(Channel::CH1314)));
    map.insert(0x7A, Address::BusSend(BusSend::Effect2(Channel::CH1516)));
    map.insert(0x84, Address::BusSend(BusSend::Effect1(Channel::Return2)));
    map.insert(0x85, Address::BusSend(BusSend::Effect2(Channel::Return1)));

    let mut i: u16 = 0x120;
    for knob in vec![EqKnob::F, EqKnob::G, EqKnob::Q] {
        for band in vec![EqBand::Low, EqBand::LoMid, EqBand::HiMid, EqBand::High] {
            for eq_channel in 0u8..=21 {
                if let Some(channel) = EqChannel::from_u8(eq_channel) {
                    map.insert(i, Address::EqControl(knob(band(channel))));
                }
                i += 1;
            }
        }
    }

    // Master volumes
    map.insert(0x1C, Address::BusMaster(Bus::Aux1));
    map.insert(0x1D, Address::BusMaster(Bus::Aux2));
    map.insert(0x1E, Address::BusMaster(Bus::Aux3));
    map.insert(0x1F, Address::BusMaster(Bus::Aux4));
    map.insert(0x24, Address::BusMaster(Bus::StereoOut));
    map.insert(0x25, Address::BusMaster(Bus::Effect1));
    map.insert(0x26, Address::BusMaster(Bus::Effect2));
}