function emitUpdate(obj) {
    console.log(obj);

    fetch("http://127.0.0.1:8080/update_value", {
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

class Channel {
    constructor(hasOn, hasSolo, hasPan, hasEq) {
        if (hasOn) this.on = false;
        if (hasSolo) this.solo = false;
        if (hasPan) this.pan = 0;
        if (hasEq) this.eq = new Eq();
    }
}

class Bus {

    #masterVolume = 0;
    #sends = {
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

    constructor(name) {
        this.name = name;
        if (name !== "Return1") this.#sends.Return1 = 0;
        if (name !== "Return2") this.#sends.Return2 = 0;
    }

    setSend(channel, value) {
        if (channel in this.#sends) {
            this.#sends[channel] = value;
        } else {
            console.log("Warning: Invalid channel: " + channel);
            return;
        }

        emitUpdate({
            value,
            control: {
                Channel: channel
            }
        });
    }
    getSend(channel) {
        return this.#sends[channel];
    }

    setMaster(value) {
        this.#masterVolume = value;
        emitUpdate({
            value,
            control: {
                Master: this.name,
            }
        });
    }
    getMaster() {
        return this.#masterVolume;
    }
}

let mixerState = {
    stereoOut: new Bus("StereoOut"),
    aux1: new Bus("Aux1"),
    aux2: new Bus("Aux2"),
    aux3: new Bus("Aux3"),
    aux4: new Bus("Aux4"),
    // TODO: Bus1-4
    effect1: new Bus("Effect1"),
    effect2: new Bus("Effect2"),
};