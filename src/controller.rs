use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct U7ControlVal {
    pub control: Address,
    pub value: u8, // Top bit is ignored
    pub client_id: String, // For client sync
}

#[derive(Serialize, Deserialize)]
pub struct BitControlVal {
    pub control: BitControl,
    pub value: bool,
    pub client_id: String, // For client sync
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Address {
    BusSend(BusSend),
    BusMaster(Bus),
    EqControl(EqKnob),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum BusSend {
    StereoOut(Channel),
    Aux1(Channel),
    Aux2(Channel),
    Aux3(Channel),
    Aux4(Channel),
    Effect1(Channel), // Does not have Return1
    Effect2(Channel), // Does not have Return2
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Bus {
    StereoOut,
    Aux1,
    Aux2,
    Aux3,
    Aux4,
    Effect1,
    Effect2,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum EqBand {
    Low(EqChannel),
    LoMid(EqChannel),
    HiMid(EqChannel),
    High(EqChannel),
}
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum EqChannel {
    Channel(Channel),
    Bus(Bus), // Bus cannot have Effect 1 or 2
}
impl EqChannel {
    pub fn from_u8(value: u8) -> Option<EqChannel> {
        match value {
            0 => Some(EqChannel::Channel(Channel::CH1)),
            1 => Some(EqChannel::Channel(Channel::CH2)),
            2 => Some(EqChannel::Channel(Channel::CH3)),
            3 => Some(EqChannel::Channel(Channel::CH4)),
            4 => Some(EqChannel::Channel(Channel::CH5)),
            5 => Some(EqChannel::Channel(Channel::CH6)),
            6 => Some(EqChannel::Channel(Channel::CH7)),
            7 => Some(EqChannel::Channel(Channel::CH8)),
            8 => Some(EqChannel::Channel(Channel::CH9)),
            9 => Some(EqChannel::Channel(Channel::CH10)),
            10 => Some(EqChannel::Channel(Channel::CH11)),
            11 => Some(EqChannel::Channel(Channel::CH12)),
            12 => Some(EqChannel::Channel(Channel::CH1314)),
            13 => Some(EqChannel::Channel(Channel::CH1516)),
            14 => Some(EqChannel::Channel(Channel::Return1)),
            15 => Some(EqChannel::Channel(Channel::Return2)),
            16 => Some(EqChannel::Bus(Bus::Aux1)),
            17 => Some(EqChannel::Bus(Bus::Aux2)),
            18 => Some(EqChannel::Bus(Bus::Aux3)),
            19 => Some(EqChannel::Bus(Bus::Aux4)),
            20 => Some(EqChannel::Bus(Bus::StereoOut)),
            _ => None,
        }
    }
}
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum EqKnob {
    F(EqBand),
    G(EqBand),
    Q(EqBand),
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
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

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub enum BitControl {
    ChannelEnable(Channel),
    BusEnable(Bus),
    EqEnable(EqChannel),
}
impl BitControl { // TODO: Move to BiHashMap
    pub fn to_address(&self) -> (u16, u8) {
        match self {

            Self::ChannelEnable(c) | Self::EqEnable(EqChannel::Channel(c)) => {
                let (offset, channel_bits) = match c {
                    Channel::CH1     => (0, 0b000),
                    Channel::CH2     => (0, 0b001),
                    Channel::CH3     => (0, 0b010),
                    Channel::CH4     => (0, 0b011),
                    Channel::CH5     => (0, 0b100),
                    Channel::CH6     => (0, 0b101),
                    Channel::CH7     => (0, 0b110),
                    Channel::CH8     => (0, 0b111),
                    Channel::CH9     => (1, 0b000),
                    Channel::CH10    => (1, 0b001),
                    Channel::CH11    => (1, 0b010),
                    Channel::CH12    => (1, 0b011),
                    Channel::CH1314  => (1, 0b100),
                    Channel::CH1516  => (1, 0b101),
                    Channel::Return1 => (1, 0b110),
                    Channel::Return2 => (1, 0b111),
                };
                match self {
                    Self::ChannelEnable(_) => (offset + 0x0B0, channel_bits),
                    Self::EqEnable(EqChannel::Channel(_)) => (offset + 0x11C, channel_bits),
                    _ => unreachable!(),
                }
            },
            Self::BusEnable(b) | Self::EqEnable(EqChannel::Bus(b)) => {
                let bus_bits = match b {
                    Bus::Aux1      => 0b000,
                    Bus::Aux2      => 0b001,
                    Bus::Aux3      => 0b010,
                    Bus::Aux4      => 0b011,
                    Bus::Effect1   => 0b000,
                    Bus::Effect2   => 0b001,
                    Bus::StereoOut => 0b111,
                };
                match self {

                    Self::BusEnable(_) => (match b {
                        Bus::Aux1 | Bus::Aux2 | Bus::Aux3 | Bus::Aux4 => 0x0B2,
                        Bus::Effect1 | Bus::Effect2 => 0x0B3,
                        Bus::StereoOut => 0x0B4,
                    }, bus_bits),

                    Self::EqEnable(EqChannel::Bus(_)) => (match b {
                        Bus::Aux1 | Bus::Aux2 | Bus::Aux3 | Bus::Aux4 | Bus::StereoOut => 0x011F,
                        _ => panic!(), // Invalid configuration for EQ Bus
                    }, bus_bits),

                    _ => unreachable!(),
                }
            }
        }
    }
    pub fn from_address(address: (u16, u8)) -> Option<Self> {


        let offset = match address.0 {
            0x0B0..=0x0B4 => 0x0B0,
            0x11C..=0x11F => 0x11C,
            _ => return None,
        };

        let channel: Option<Channel> = match (address.0 - offset, address.1) {
            (0, 0b000) => Some(Channel::CH1),
            (0, 0b001) => Some(Channel::CH2),
            (0, 0b010) => Some(Channel::CH3),
            (0, 0b011) => Some(Channel::CH4),
            (0, 0b100) => Some(Channel::CH5),
            (0, 0b101) => Some(Channel::CH6),
            (0, 0b110) => Some(Channel::CH7),
            (0, 0b111) => Some(Channel::CH8),
            (1, 0b000) => Some(Channel::CH9),
            (1, 0b001) => Some(Channel::CH10),
            (1, 0b010) => Some(Channel::CH11),
            (1, 0b011) => Some(Channel::CH12),
            (1, 0b100) => Some(Channel::CH1314),
            (1, 0b101) => Some(Channel::CH1516),
            (1, 0b110) => Some(Channel::Return1),
            (1, 0b111) => Some(Channel::Return2),
            _ => None,
        };

        if channel.is_some() {
            return match offset {
                0x0B0 => Some(Self::ChannelEnable(channel.unwrap())),
                0x11C => Some(Self::EqEnable(EqChannel::Channel(channel.unwrap()))),
                _ => unreachable!(),
            };
        }

        let bus: Option<Bus> = match address {
            (0x0B2 | 0x11F, 0b000) => Some(Bus::Aux1),
            (0x0B2 | 0x11F, 0b001) => Some(Bus::Aux2),
            (0x0B2 | 0x11F, 0b010) => Some(Bus::Aux3),
            (0x0B2 | 0x11F, 0b011) => Some(Bus::Aux4),
            (0x0B4 | 0x11F, 0b111) => Some(Bus::StereoOut),
            (0x0B3, 0b000) => Some(Bus::Effect1),
            (0x0B3, 0b001) => Some(Bus::Effect2),
            _ => return None,
        };

        match offset {
            0x0B0 => Some(Self::BusEnable(bus.unwrap())),
            0x11C => Some(Self::EqEnable(EqChannel::Bus(bus.unwrap()))),
            _ => unreachable!(),
        }
    }
}