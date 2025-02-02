function emitUpdate(obj, endpoint) {
    console.log(obj);

    fetch("http://127.0.0.1:8080/" + endpoint, {
        method: "POST",
        body: JSON.stringify(obj),
        headers: {
            "Content-Type": "application/json"
        }
    });
}

class Eq {
    constructor() {
        this.lowBand = {
            mode: "Normal",
            freq: 0,
            ampl: 0,
            Q: 0,
        };
        this.loMidBand = {
            freq: 0,
            ampl: 0,
            Q: 0,
        };
        this.hiMidBand = {
            freq: 0,
            ampl: 0,
            Q: 0,
        };
        this.highBand = {
            mode: "Normal",
            freq: 0,
            ampl: 0,
            Q: 0,
        };
    }
}

class MasterChannel {
    send = 0;
    on = false;
    solo = false;
    pan = 0;
    eq = new Eq();
}

class Bus {

    _masterVolume = 0;
    _isOn = false;
    _sends;

    constructor(name) {
        this.name = name;

        this._sends = {
            CH1: 0,
            CH2: 0,
            CH3: 0,
            CH4: 0,
            CH5: 0,
            CH6: 0,
            CH7: 0,
            CH8: 0,
            CH9: 0,
            CH10: 0,
            CH11: 0,
            CH12: 0,
            CH1314: 0,
            CH1516: 0,
        };

        if (name !== "Effect1") this._sends.Return1 = 0;
        if (name !== "Effect2") this._sends.Return2 = 0;
    }

    setSend(channel, value) {
        if (!(channel in this._sends)) {
            console.log("Warning: Invalid channel: " + channel);
            return;
        }

        if (this._sends[channel].send) { // MasterBus
            this._sends[channel].send = value;
        } else {
            this._sends[channel] = value;
        }

        let obj = {
            value,
            control: {}
        };
        obj.control[this.name] = channel;

        emitUpdate(obj, "fader");
    }
    getSend(channel) {
        return this._sends[channel].send ? this._sends[channel].send : this._sends[channel];
    }

    setMaster(value) {
        this._masterVolume = value;
        emitUpdate({
            value,
            control: {
                Master: this.name,
            }
        }, "fader");
    }
    getMaster() {
        return this._masterVolume;
    }

    setMasterOn(on) {
        this._isOn = on;
        emitUpdate({
            value: on,
            control: {
                Master: this.name
            }
        }, "on");
    }
}

class StereoOut extends Bus {
    constructor() {
        super("StereoOut");
        this._sends = {
            CH1: new MasterChannel(),
            CH2: new MasterChannel(),
            CH3: new MasterChannel(),
            CH4: new MasterChannel(),
            CH5: new MasterChannel(),
            CH6: new MasterChannel(),
            CH7: new MasterChannel(),
            CH8: new MasterChannel(),
            CH9: new MasterChannel(),
            CH10: new MasterChannel(),
            CH11: new MasterChannel(),
            CH12: new MasterChannel(),
            CH1314: new MasterChannel(),
            CH1516: new MasterChannel(),
            Effect1: new MasterChannel(),
            Effect2: new MasterChannel(),
        };
    }

    setOn(channel, on) {
        if (!(channel in this._sends)) {
            console.log("Warning: Invalid channel: " + channel);
            return;
        }

        this._sends[channel].on = on;
        emitUpdate({
            value: on,
            control: {
                Channel: channel
            }
        }, "on");
    }
}

let mixerState = {
    stereoOut: new StereoOut(),
    aux1: new Bus("Aux1"),
    aux2: new Bus("Aux2"),
    aux3: new Bus("Aux3"),
    aux4: new Bus("Aux4"),
    // TODO: Bus1-4
    effect1: new Bus("Effect1"),
    effect2: new Bus("Effect2"),
};