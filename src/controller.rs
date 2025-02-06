use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct U7ControlVal {
    pub control: Address,
    pub value: u8, // Top bit is ignored
}

#[derive(Serialize, Deserialize)]
pub struct BitControlVal {
    pub control: OnControl,
    pub value: bool,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Address {
    BusSend(BusSend),
    BusMaster(Bus),
    EqControl(EqControl),
    OnControl(OnControl),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum BusSend {
    StereoOut(Channel),
    Aux1(Channel),
    Aux2(Channel),
    Aux3(Channel),
    Aux4(Channel),
    Effect1(Channel), // Does not have Return1
    Effect2(Channel), // Does not have Return2
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum Bus {
    StereoOut,
    Aux1,
    Aux2,
    Aux3,
    Aux4,
    Effect1,
    Effect2,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum EqControl {
    On(EqChannel),
    Param {
        channel: EqChannel,
        band: EqBand,
        knob: EqKnob
    },
    Attenuator(Channel), // sans Returns
}
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum EqBand {
    Low,
    LoMid,
    HiMid,
    High,
}
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum EqChannel {
    CH1,
    CH2,
    CH3,
    CH4,
    CH5,
    CH6,
    CH7,
    CH8,
    CH9,
    CH10,
    CH11,
    CH12,
    CH1314,
    CH1516,
    Return1,
    Return2,
    Aux1,
    Aux2,
    Aux3,
    Aux4,
    StereoOut,
}
impl EqChannel {
    pub fn from_u8(value: u8) -> Option<EqChannel> {
        match value {
            0 => Some(EqChannel::CH1),
            1 => Some(EqChannel::CH2),
            2 => Some(EqChannel::CH3),
            3 => Some(EqChannel::CH4),
            4 => Some(EqChannel::CH5),
            5 => Some(EqChannel::CH6),
            6 => Some(EqChannel::CH7),
            7 => Some(EqChannel::CH8),
            8 => Some(EqChannel::CH9),
            9 => Some(EqChannel::CH10),
            10 => Some(EqChannel::CH11),
            11 => Some(EqChannel::CH12),
            12 => Some(EqChannel::CH1314),
            13 => Some(EqChannel::CH1516),
            14 => Some(EqChannel::Return1),
            15 => Some(EqChannel::Return2),
            16 => Some(EqChannel::Aux1),
            17 => Some(EqChannel::Aux2),
            18 => Some(EqChannel::Aux3),
            19 => Some(EqChannel::Aux4),
            20 => Some(EqChannel::StereoOut),
            _ => None,
        }
    }
}
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
pub enum EqKnob {
    F,
    G,
    Q,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
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

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum OnControl {
    Channel(EqChannel),
    Effect1Send,
    Effect2Send,
}
impl OnControl { // TODO: Move to BiHashMap
    pub fn to_address(&self) -> (u16, u8) {
        match self {
            OnControl::Channel(c) => match c {
                EqChannel::CH1 => (0xB0, 0b000),
                EqChannel::CH2 => (0xB0, 0b001),
                EqChannel::CH3 => (0xB0, 0b010),
                EqChannel::CH4 => (0xB0, 0b011),
                EqChannel::CH5 => (0xB0, 0b100),
                EqChannel::CH6 => (0xB0, 0b101),
                EqChannel::CH7 => (0xB0, 0b110),
                EqChannel::CH8 => (0xB0, 0b111),

                EqChannel::CH9 => (0xB1, 0b000),
                EqChannel::CH10 => (0xB1, 0b001),
                EqChannel::CH11 => (0xB1, 0b010),
                EqChannel::CH12 => (0xB1, 0b011),
                EqChannel::CH1314 => (0xB1, 0b100),
                EqChannel::CH1516 => (0xB1, 0b101),
                EqChannel::Return1 => (0xB1, 0b110),
                EqChannel::Return2 => (0xB1, 0b111),

                EqChannel::Aux1 => (0xB2, 0b000),
                EqChannel::Aux2 => (0xB2, 0b001),
                EqChannel::Aux3 => (0xB2, 0b010),
                EqChannel::Aux4 => (0xB2, 0b011),

                EqChannel::StereoOut => (0xB4, 0b111),
            },

            OnControl::Effect1Send => (0xB3, 0b000),
            OnControl::Effect2Send => (0xB3, 0b001),
        }
    }
    pub fn from_address(address: (u16, u8)) -> Option<Self> {
        match address {
            (0xB0, 0b000) => Some(Self::Channel(EqChannel::CH1)),
            (0xB0, 0b001) => Some(Self::Channel(EqChannel::CH2)),
            (0xB0, 0b010) => Some(Self::Channel(EqChannel::CH3)),
            (0xB0, 0b011) => Some(Self::Channel(EqChannel::CH4)),
            (0xB0, 0b100) => Some(Self::Channel(EqChannel::CH5)),
            (0xB0, 0b101) => Some(Self::Channel(EqChannel::CH6)),
            (0xB0, 0b110) => Some(Self::Channel(EqChannel::CH7)),
            (0xB0, 0b111) => Some(Self::Channel(EqChannel::CH8)),

            (0xB1, 0b000) => Some(Self::Channel(EqChannel::CH9)),
            (0xB1, 0b001) => Some(Self::Channel(EqChannel::CH10)),
            (0xB1, 0b010) => Some(Self::Channel(EqChannel::CH11)),
            (0xB1, 0b011) => Some(Self::Channel(EqChannel::CH12)),
            (0xB1, 0b100) => Some(Self::Channel(EqChannel::CH1314)),
            (0xB1, 0b101) => Some(Self::Channel(EqChannel::CH1516)),
            (0xB1, 0b110) => Some(Self::Channel(EqChannel::Return1)),
            (0xB1, 0b111) => Some(Self::Channel(EqChannel::Return2)),

            (0xB2, 0b000) => Some(Self::Channel(EqChannel::Aux1)),
            (0xB2, 0b001) => Some(Self::Channel(EqChannel::Aux2)),
            (0xB2, 0b010) => Some(Self::Channel(EqChannel::Aux3)),
            (0xB2, 0b011) => Some(Self::Channel(EqChannel::Aux4)),

            (0xB4, 0b111) => Some(Self::Channel(EqChannel::StereoOut)),

            (0xB3, 0b000) => Some(Self::Effect1Send),
            (0xB3, 0b001) => Some(Self::Effect2Send),

            _ => None,
        }
    }
}