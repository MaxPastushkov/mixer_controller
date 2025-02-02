use serde::{Serialize, Deserialize};

pub trait Controller {
    fn serialize(&self) -> [u8; 4];

    type ValueType;
    fn get_value(&self) -> Self::ValueType;
}

#[derive(Serialize, Deserialize)]
pub struct FaderControlVal {
    pub value: u8,
    pub control: FaderControl,
}
impl Controller for FaderControlVal {
    fn serialize(&self) -> [u8; 4] {
        let id = self.control.get_id();
        [0x10, id[0], id[1], self.value]
    }

    type ValueType = u8;
    fn get_value(&self) -> Self::ValueType {
        self.value.clamp(0x00, 0x7F)
    }
}
impl FaderControlVal {
    #[allow(dead_code)]
    pub fn new(control: FaderControl, value: u8) -> Self {
        Self {
            control,
            value,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum FaderControl {
    StereoOut(Channel),
    Master(MasterChannel),
    Aux1Send(Channel),
    Aux2Send(Channel),
    Aux3Send(Channel),
    Aux4Send(Channel),
    Effect1Send(Channel), // Does not have Return2
    Effect2Send(Channel), // Does not have Return1
}

impl FaderControl {
    fn get_id(&self) -> [u8; 2] {
        match self {
            FaderControl::StereoOut(c) => [0x00, c.value() + 0x0B],
            FaderControl::Master(c) => [0x00, c.value() + 0x1B],
            FaderControl::Aux1Send(c) => [0x00, c.value() + 0x26],
            FaderControl::Aux2Send(c) => [0x00, c.value() + 0x32],
            FaderControl::Aux3Send(c) => [0x00, c.value() + 0x3E],
            FaderControl::Aux4Send(c) => [0x00, c.value() + 0x4A],
            FaderControl::Effect1Send(c) => match c {
                Channel::Return2 => [0x01, 0x04],
                _ => [0x00, c.value() + 0x56]
            },
            FaderControl::Effect2Send(c) => match c {
                Channel::Return1 => [0x01, 0x05],
                _ => [0x00, c.value() + 0x62],
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct OnControlVal {
    pub value: bool,
    pub control: OnControl,
}
impl Controller for OnControlVal {
    fn serialize(&self) -> [u8; 4] {
        let (group, mut id) = self.control.get_id();
        if self.value { id |= 0b1000 }; // If tuning on, set id bit 3
        [0x40, 0x01, group, id]
    }

    type ValueType = bool;
    fn get_value(&self) -> Self::ValueType {
        self.value
    }
}
impl OnControlVal {
    #[allow(dead_code)]
    pub fn new(control: OnControl, value: bool) -> Self {
        Self {
            control,
            value,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum OnControl {
    Channel(Channel),
    Master(MasterChannel),
    BusToST(Bus)
}
impl OnControl {
    fn get_id(&self) -> (u8, u8) { // (group, id)
        match self {
            OnControl::Channel(c) => match c {
                Channel::CH1     => (0x30, 0b000),
                Channel::CH2     => (0x30, 0b001),
                Channel::CH3     => (0x30, 0b010),
                Channel::CH4     => (0x30, 0b011),
                Channel::CH5     => (0x30, 0b100),
                Channel::CH6     => (0x30, 0b101),
                Channel::CH7     => (0x30, 0b110),
                Channel::CH8     => (0x30, 0b111),
                Channel::CH9     => (0x31, 0b000),
                Channel::CH10    => (0x31, 0b001),
                Channel::CH11    => (0x31, 0b010),
                Channel::CH12    => (0x31, 0b011),
                Channel::CH1314  => (0x31, 0b100),
                Channel::CH1516  => (0x31, 0b101),
                Channel::Return1 => (0x31, 0b110),
                Channel::Return2 => (0x31, 0b111),
            },
            OnControl::Master(m) => match m {
                MasterChannel::Aux1      => (0x32, 0b000),
                MasterChannel::Aux2      => (0x32, 0b001),
                MasterChannel::Aux3      => (0x32, 0b010),
                MasterChannel::Aux4      => (0x32, 0b011),
                MasterChannel::Bus1      => (0x32, 0b100),
                MasterChannel::Bus2      => (0x32, 0b101),
                MasterChannel::Bus3      => (0x32, 0b110),
                MasterChannel::Bus4      => (0x32, 0b111),
                MasterChannel::Effect1   => (0x33, 0b000),
                MasterChannel::Effect2   => (0x33, 0b001),
                MasterChannel::StereoOut => (0x34, 0b111),
            },
            OnControl::BusToST(b) => (0x34, match b {
                Bus::Bus1 => 0b000,
                Bus::Bus2 => 0b001,
                Bus::Bus3 => 0b010,
                Bus::Bus4 => 0b011,
            })
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct EqControlVal {
    pub value: u8,
    pub control: EqControl,
}
impl Controller for EqControlVal {
    fn serialize(&self) -> [u8; 4] {
        match &self.control {
            EqControl::On(_) => [0; 4], // TODO: Make this work
            EqControl::Attenuator(_) => [0; 4],
            EqControl::Param { channel, band, knob } => {
                let mut id: u16 = u16::from(knob.value() * 0x58);
                id += u16::from(band.value() * 0x16);
                id += u16::from(channel.value());
                id += 0x120;
                [0x10, (id >> 7) as u8, (id & 0b0111_1111) as u8, self.get_value()]
            }
        }
    }

    type ValueType = u8;
    fn get_value(&self) -> Self::ValueType {
        let v = self.value.clamp(0x00, 0x7F);
        match &self.control {
            EqControl::On(_) => if v > 0 { 1 } else { 0 },
            EqControl::Attenuator(_) => if v > 0 { 1 } else { 0 },
            EqControl::Param { channel: _, band, knob } => match (band, knob) {
                (_, EqKnob::F) => (v as f64 * 0.937007875).round() as u8,


                (EqBand::High(EqSpecialMode::Normal) |
                 EqBand::Low (EqSpecialMode::Normal) |
                 EqBand::HiMid |
                 EqBand::LoMid,
                    EqKnob::Q) => (v as f64 * 0.314960630).round() as u8,

                (EqBand::High(EqSpecialMode::Normal | EqSpecialMode::Shelf) |
                 EqBand::Low (EqSpecialMode::Normal | EqSpecialMode::Shelf) |
                 EqBand::HiMid |
                 EqBand::LoMid,
                    EqKnob::G) => (v as f64 * 0.566929134).round() as u8,

                (EqBand::Low (EqSpecialMode::Filter) |
                 EqBand::High(EqSpecialMode::Filter),
                    EqKnob::G) => if v > 0 { 0x24 } else { 0x23 },

                (EqBand::Low(EqSpecialMode::Shelf),   EqKnob::Q) => 0x29,
                (EqBand::High(EqSpecialMode::Shelf),  EqKnob::Q) => 0x2A,
                (EqBand::High(EqSpecialMode::Filter), EqKnob::Q) => 0x2B,
                (EqBand::Low(EqSpecialMode::Filter),  EqKnob::Q) => 0x2C,
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum EqControl {
    On(EqChannel),
    Param {
        channel: EqChannel,
        band: EqBand,
        knob: EqKnob
    },
    Attenuator(Channel), // sans Returns
}

#[derive(Serialize, Deserialize)]
pub enum EqSpecialMode {
    Normal,
    Shelf,
    Filter,
}

#[derive(Serialize, Deserialize)]
pub enum EqBand {
    Low(EqSpecialMode),
    LoMid,
    HiMid,
    High(EqSpecialMode),
}
impl EqBand {
    pub fn value(&self) -> u8 {
        match self {
            EqBand::Low(_) => 0,
            EqBand::LoMid => 1,
            EqBand::HiMid => 2,
            EqBand::High(_) => 3,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum EqKnob {
    F,
    G,
    Q,
}
impl EqKnob {
    pub fn value(&self) -> u8 {
        match self {
            EqKnob::F => 0,
            EqKnob::G => 1,
            EqKnob::Q => 2,
        }
    }
}

#[derive(Serialize, Deserialize)]
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
    pub fn value(&self) -> u8 {
        match self {
            EqChannel::CH1 => 0,
            EqChannel::CH2 => 1,
            EqChannel::CH3 => 2,
            EqChannel::CH4 => 3,
            EqChannel::CH5 => 4,
            EqChannel::CH6 => 5,
            EqChannel::CH7 => 6,
            EqChannel::CH8 => 7,
            EqChannel::CH9 => 8,
            EqChannel::CH10 => 9,
            EqChannel::CH11 => 10,
            EqChannel::CH12 => 11,
            EqChannel::CH1314 => 12,
            EqChannel::CH1516 => 13,
            EqChannel::Return1 => 14,
            EqChannel::Return2 => 15,
            EqChannel::Aux1 => 16,
            EqChannel::Aux2 => 17,
            EqChannel::Aux3 => 18,
            EqChannel::Aux4 => 19,
            EqChannel::StereoOut => 20,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Channel {
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
}
impl Channel {
    pub fn value(&self) -> u8 {
        match self {
            Channel::CH1 => 1,
            Channel::CH2 => 2,
            Channel::CH3 => 3,
            Channel::CH4 => 4,
            Channel::CH5 => 5,
            Channel::CH6 => 6,
            Channel::CH7 => 7,
            Channel::CH8 => 8,
            Channel::CH9 => 9,
            Channel::CH10 => 10,
            Channel::CH11 => 11,
            Channel::CH12 => 12,
            Channel::CH1314 => 13,
            Channel::CH1516 => 14,
            Channel::Return1 => 15,
            Channel::Return2 => 16,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum MasterChannel {
    Aux1,
    Aux2,
    Aux3,
    Aux4,
    Bus1,
    Bus2,
    Bus3,
    Bus4,
    StereoOut,
    Effect1,
    Effect2,
}

impl MasterChannel {
    pub fn value(&self) -> u8 {
        match self {
            MasterChannel::Aux1 => 1,
            MasterChannel::Aux2 => 2,
            MasterChannel::Aux3 => 3,
            MasterChannel::Aux4 => 4,
            MasterChannel::Bus1 => 5,
            MasterChannel::Bus2 => 6,
            MasterChannel::Bus3 => 7,
            MasterChannel::Bus4 => 8,
            MasterChannel::StereoOut => 9,
            MasterChannel::Effect1 => 10,
            MasterChannel::Effect2 => 11,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Bus {
    Bus1,
    Bus2,
    Bus3,
    Bus4,
}