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
    send = [0];
    on = [false];
    solo = [false];
    pan = [0];
    eq = new Eq();
}

class Bus {

    _masterVolume = [0];
    _isOn = [false];
    _sends;

    constructor(name) {
        this.name = name;

        this._sends = {
            CH1: [0],
            CH2: [0],
            CH3: [0],
            CH4: [0],
            CH5: [0],
            CH6: [0],
            CH7: [0],
            CH8: [0],
            CH9: [0],
            CH10: [0],
            CH11: [0],
            CH12: [0],
            CH1314: [0],
            CH1516: [0],
        };

        if (name !== "Effect1") this._sends.Return1 = [0];
        if (name !== "Effect2") this._sends.Return2 = [0];
    }

    // Called by Rust
    updateSend(channel, value) {
        if (!(channel in this._sends)) {
            console.log("Warning: Invalid channel: " + channel);
            return;
        }

        if (this._sends[channel].send) { // MasterBus
            this._sends[channel].send = value;
        } else {
            this._sends[channel] = value;
        }
    }
    updateMaster(value) {
        this._masterVolume = value;
    }

    setMaster(value) {
        this.updateMaster(value);

        let obj = {
            value,
            control: {
                BusMaster: this.name
            }
        };

        emitUpdate(obj, "u7")
    }

    // setSend(channel, value) {
    //     if (!(channel in this._sends)) {
    //         console.log("Warning: Invalid channel: " + channel);
    //         return;
    //     }
    //
    //     if (this._sends[channel].send) { // MasterBus
    //         this._sends[channel].send = value;
    //     } else {
    //         this._sends[channel] = value;
    //     }
    //
    //     let obj = {
    //         value,
    //         control: {
    //             BusSend: {}
    //         }
    //     };
    //     obj.control.BusSend[this.name] = channel;
    //
    //     emitUpdate(obj, "u7");
    // }
    // getSend(channel) {
    //     return this._sends[channel].send ? this._sends[channel].send : this._sends[channel];
    // }
    //
    // setMaster(value) {
    //     this._masterVolume = value;
    //     emitUpdate({
    //         value,
    //         control: {
    //             BusMaster: this.name,
    //         }
    //     }, "u7");
    // }
    // getMaster() {
    //     return this._masterVolume;
    // }
    //
    // setMasterOn(on) {
    //     this._isOn = on;
    //     emitUpdate({
    //         value: on,
    //         control: {
    //             Channel: this.name
    //         }
    //     }, "bit");
    // }
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

    // setOn(channel, on) {
    //     if (!(channel in this._sends)) {
    //         console.log("Warning: Invalid channel: " + channel);
    //         return;
    //     }
    //
    //     this._sends[channel].on = on;
    //     emitUpdate({
    //         value: on,
    //         control: {
    //             Channel: channel
    //         }
    //     }, "on");
    // }
}

let mixerState = {
    StereoOut: new StereoOut(),
    Aux1: new Bus("Aux1"),
    Aux2: new Bus("Aux2"),
    Aux3: new Bus("Aux3"),
    Aux4: new Bus("Aux4"),
    // TODO: Bus1-4
    Effect1: new Bus("Effect1"),
    Effect2: new Bus("Effect2"),
};

function sseEvent(obj) {
    updateValue(obj);

    let control = document.getElementById(JSON.stringify(obj.control));
    if (control) {
        control.value = obj.value;
    }
}

function updateValue(obj) {
    if (!obj.control || !obj.value) return;

    if (obj.control.BusSend) {
        let send = obj.control.BusSend;
        mixerState[Object.keys(send)[0]].updateSend(Object.values(send)[0], obj.value);
    } else if (BusMaster in obj.control) {
        mixerState[Object.keys(obj.control.BusMaster)[0]].updateMaster(obj.value);
    } else {
        alert("Unknown control value");
    }
}

function controlChange(control, value, type) {
    let obj = {
        control,
        value: parseInt(value)
    };

    updateValue(obj);
    emitUpdate(obj, type);
}

function initControls() {
    for (let control of document.getElementsByClassName("control")) {

        let type = "";
        if (control.type === "range") type = "u7"
        if (control.type === "checkbox") type = "bit"

        control.setAttribute("oninput", "controlChange(" + control.id + ",this.value,\"" + type + "\")");
    }
}