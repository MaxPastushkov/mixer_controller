function emitUpdate(obj) {
    console.log(obj);
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

    constructor(name, isEff1, isEff2) {
        this.name = name;
        if (!isEff1) this.#sends.Return1 = 0;
        if (!isEff2) this.#sends.Return2 = 0;
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

    setMaster(value) {
        emitUpdate({
            value,
            control: {
                Master: this.name,
            }
        });
    }
}

let mixerState = {
    stereoOut: new Bus(),
    aux1: new Bus(),
    aux2: new Bus(),
    aux3: new Bus(),
    aux4: new Bus(),
    // TODO: Bus1-4
    effect1: new Bus(true, false),
    effect2: new Bus(false, true),
};