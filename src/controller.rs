use serde::{Serialize, Deserialize};
use derivative::Derivative;

#[derive(Serialize, Deserialize, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub enum Address {
    BusSend(BusSend),
}

#[derive(Serialize, Deserialize, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub enum BusSend {
    StereoOut(Channel),
    Aux1(Channel),
    Aux2(Channel),
    Aux3(Channel),
    Aux4(Channel),
    Effect1(Channel), // Does not have Return1
    Effect2(Channel), // Does not have Return2
}

#[derive(Serialize, Deserialize, Eq, Derivative)]
#[derivative(PartialEq, Hash)]
pub enum Channel {
    CH1 = 0,
    CH2 = 1,
    CH3 = 2,
    CH4 = 3,
    CH5 = 4,
    CH6 = 5,
    CH7 = 6,
    CH8 = 7,
    CH9 = 8,
    CH10 = 9,
    CH11 = 10,
    CH12 = 11,
    CH1314 = 12,
    CH1516 = 13,
    Return1 = 14,
    Return2 = 15,
}
impl Channel {
    pub fn from_u8(value: u8) -> Option<Channel> {
        match value {
            0 => Some(Channel::CH1),
            1 => Some(Channel::CH2),
            2 => Some(Channel::CH3),
            3 => Some(Channel::CH4),
            4 => Some(Channel::CH5),
            5 => Some(Channel::CH6),
            6 => Some(Channel::CH7),
            7 => Some(Channel::CH8),
            8 => Some(Channel::CH9),
            9 => Some(Channel::CH10),
            10 => Some(Channel::CH11),
            11 => Some(Channel::CH12),
            12 => Some(Channel::CH1314),
            13 => Some(Channel::CH1516),
            14 => Some(Channel::Return1),
            15 => Some(Channel::Return2),
            _ => None,
        }
    }
}