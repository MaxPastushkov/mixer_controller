use bimap::{BiHashMap};
use crate::controller::{Address, Channel, BusSend};

pub fn init_state_map(map: &mut BiHashMap<u16, Address>) {

    for i in 0u8..=15 {
        map.insert((0x0C + i) as u16, Address::BusSend(BusSend::StereoOut(Channel::from_u8(i).unwrap())));
        map.insert((0x27 + i) as u16, Address::BusSend(BusSend::Aux1(Channel::from_u8(i).unwrap())));
        map.insert((0x33 + i) as u16, Address::BusSend(BusSend::Aux2(Channel::from_u8(i).unwrap())));
        map.insert((0x3F + i) as u16, Address::BusSend(BusSend::Aux3(Channel::from_u8(i).unwrap())));
        map.insert((0x4B + i) as u16, Address::BusSend(BusSend::Aux4(Channel::from_u8(i).unwrap())));

        // Channels 1-12 for effects are normal, the rest are not
        if i <= Channel::CH12 as u8 {
            map.insert((0x57 + i) as u16, Address::BusSend(BusSend::Effect1(Channel::from_u8(i).unwrap())));
            map.insert((0x63 + i) as u16, Address::BusSend(BusSend::Effect2(Channel::from_u8(i).unwrap())));
        }
    }

    // Special cases for effects sends
    map.insert(0x77, Address::BusSend(BusSend::Effect1(Channel::CH1314)));
    map.insert(0x78, Address::BusSend(BusSend::Effect1(Channel::CH1516)));
    map.insert(0x79, Address::BusSend(BusSend::Effect2(Channel::CH1314)));
    map.insert(0x7A, Address::BusSend(BusSend::Effect2(Channel::CH1516)));
    map.insert(0x84, Address::BusSend(BusSend::Effect1(Channel::Return2)));
    map.insert(0x85, Address::BusSend(BusSend::Effect2(Channel::Return1)));
}