pub trait Controller {
    fn serialize(&self) -> [u8; 4];

    type ValueType;
    fn set_value(&mut self, value: Self::ValueType);
}

pub struct FaderControlVal {
    pub value: u8,
    pub control: FaderControl,
}
impl Controller for FaderControlVal {
    fn serialize(&self) -> [u8; 4] {
        [0x10, 0x00, self.control.get_id(), self.value]
    }

    type ValueType = u8;
    fn set_value(&mut self, value: u8) {
        self.value = value;
    }
}

impl Controller for OnControlVal {
    fn serialize(&self) -> [u8; 4] {
        let mut id_with_on = self.control.get_id();
        if self.value { id_with_on |= 0b1000 };
        [0x40, 0x01, self.control.get_group(), id_with_on]
    }

    type ValueType = bool;
    fn set_value(&mut self, value: bool) {
        self.value = value;
    }
}

pub enum FaderControl {
    Channel(Channel),
    Master(MasterChannel),
    Aux1Send(Channel),
    Aux2Send(Channel),
    Aux3Send(Channel),
    Aux4Send(Channel),
    Effect1Send(Channel), // Does not have Return2
    Effect2Send(Channel), // Does not have Return1
}

impl FaderControl {
    fn get_id(&self) -> u8 {
        match self {
            FaderControl::Channel(c) => c.value() + 0x0B,
            FaderControl::Master(c) => c.value() + 0x1B,
            FaderControl::Aux1Send(c) => c.value() + 0x26,
            FaderControl::Aux2Send(c) => c.value() + 0x32,
            FaderControl::Aux3Send(c) => c.value() + 0x3E,
            FaderControl::Aux4Send(c) => c.value() + 0x4A,
            FaderControl::Effect1Send(c) => c.value() + 0x56,
            FaderControl::Effect2Send(c) => c.value() + 0x62,
        }
    }
}

pub struct OnControlVal {
    pub value: bool,
    pub control: OnControl,
}

pub enum OnControl {
    Channel(Channel),
    Master(MasterChannel),
    BusToST(Bus)
}
impl OnControl {
    fn get_id(&self) -> u8 {
        match self {
            OnControl::Channel(c) => match c {
                Channel::CH1     => 0b000,
                Channel::CH2     => 0b001,
                Channel::CH3     => 0b010,
                Channel::CH4     => 0b011,
                Channel::CH5     => 0b100,
                Channel::CH6     => 0b101,
                Channel::CH7     => 0b110,
                Channel::CH8     => 0b111,
                Channel::CH9     => 0b000,
                Channel::CH10    => 0b001,
                Channel::CH11    => 0b010,
                Channel::CH12    => 0b011,
                Channel::CH1314  => 0b100,
                Channel::CH1516  => 0b101,
                Channel::Return1 => 0b110,
                Channel::Return2 => 0b111,
            },
            OnControl::Master(m) => match m {
                MasterChannel::Aux1      => 0b000,
                MasterChannel::Aux2      => 0b001,
                MasterChannel::Aux3      => 0b010,
                MasterChannel::Aux4      => 0b011,
                MasterChannel::Bus1      => 0b100,
                MasterChannel::Bus2      => 0b101,
                MasterChannel::Bus3      => 0b110,
                MasterChannel::Bus4      => 0b111,
                MasterChannel::Effect1   => 0b000,
                MasterChannel::Effect2   => 0b001,
                MasterChannel::StereoOut => 0b111,
            },
            OnControl::BusToST(b) => match b {
                Bus::Bus1 => 0b000,
                Bus::Bus2 => 0b001,
                Bus::Bus3 => 0b010,
                Bus::Bus4 => 0b011,
            }
        }
    }
    fn get_group(&self) -> u8 {
        match self {
            OnControl::Channel(c) => match c.value() {
                1..=8 => 0x30,
                9..=16 => 0x31,
                _ => 0,
            },
            OnControl::Master(m) => match m.value() {
                1..=8 => 0x32,
                10..=11 => 0x33,
                9 => 0x34,
                _ => 0,
            },
            OnControl::BusToST(_) => 0x34,
        }
    }
}

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

pub enum Bus {
    Bus1,
    Bus2,
    Bus3,
    Bus4,
}