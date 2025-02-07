
const uid = Math.floor(Math.random() * 1000000).toString();

class Busses {
    StereoOut;
    Aux1;
    Aux2;
    Aux3;
    Aux4;
    Effect1;
    Effect2;

    constructor(initValue) {
        this.StereoOut = initValue;
        this.Aux1 = initValue;
        this.Aux2 = initValue;
        this.Aux3 = initValue;
        this.Aux4 = initValue;
        this.Effect1 = initValue;
        this.Effect2 = initValue;
    }
}

class Channels {
    CH1;
    CH2;
    CH3;
    CH4;
    CH5;
    CH6;
    CH7;
    CH8;
    CH9;
    CH10;
    CH11;
    CH12;
    CH1314;
    CH1516;
    Return1;
    Return2;

    constructor(initValue, hasReturn1 = true, hasReturn2 = true) {
        this.CH1 = initValue;
        this.CH2 = initValue;
        this.CH3 = initValue;
        this.CH4 = initValue;
        this.CH5 = initValue;
        this.CH6 = initValue;
        this.CH7 = initValue;
        this.CH8 = initValue;
        this.CH9 = initValue;
        this.CH10 = initValue;
        this.CH11 = initValue;
        this.CH12 = initValue;
        this.CH1314 = initValue;
        this.CH1516 = initValue;

        if (hasReturn1)
            this.Return1 = initValue;
        else
            delete this.Return1;

        if (hasReturn2)
            this.Return2 = initValue;
        else
            delete this.Return2;
    }
}

class EqChannels extends Channels {
    StereoOut;
    Aux1;
    Aux2;
    Aux3;
    Aux4;

    constructor(initValue) {
        super(initValue);
        this.StereoOut = initValue;
        this.Aux1 = initValue;
        this.Aux2 = initValue;
        this.Aux3 = initValue;
        this.Aux4 = initValue;
    }
}

let mixerState = {
    BusSend: {
        StereoOut: new Channels(0),
        Aux1: new Channels(0),
        Aux2: new Channels(0),
        Aux3: new Channels(0),
        Aux4: new Channels(0),
        Effect1: new Channels(0, false, true),
        Effect2: new Channels(0, true, false)
    },
    BusMaster: new Busses(0),
    ChannelEnable: new Channels(false),
    BusEnable: new Busses(false),
    EqControl: {
        F: {
            Low:   new EqChannels(0),
            LoMid: new EqChannels(0),
            HiMid: new EqChannels(0),
            High:  new EqChannels(0)
        },
        G: {
            Low:   new EqChannels(0),
            LoMid: new EqChannels(0),
            HiMid: new EqChannels(0),
            High:  new EqChannels(0)
        },
        Q: {
            Low:   new EqChannels(0),
            LoMid: new EqChannels(0),
            HiMid: new EqChannels(0),
            High:  new EqChannels(0)
        },
        On: new EqChannels(false),
        Attenuator: new Channels(false, false, false)
    }
};
delete mixerState.BusSend.Effect1.Return1;
delete mixerState.BusSend.Effect2.Return2;
delete mixerState.EqControl.Attenuator.Return1;
delete mixerState.EqControl.Attenuator.Return2;

// Convert path to object
function index(obj, idxList, value) {
    if (idxList.length === 1 && value !== undefined)
        return obj[idxList[0]] = value;
    else if (idxList.length === 0)
        return obj;
    else
        return index(obj[idxList[0]], idxList.slice(1), value);
}

function getFirstKey(obj, arr) {

    if (typeof obj !== 'object') {
        arr.push(obj);
        return obj;
    }

    arr.push(Object.keys(obj)[0]);
    return getFirstKey(Object.values(obj)[0], arr);
}

function dotToObj(dot) {
    if (typeof dot == "string") dot = dot.split('.');
    if (dot.length < 2) return dot[0];

    let obj = {};
    obj[dot[0]] = dotToObj(dot.slice(1));
    return obj;
}

function updateState(obj) {
    let {control, value} = obj;

    let path = [];
    getFirstKey(control, path);
    index(mixerState, path, value);

    // TODO: Update fader min/max
}

// Called by SSE
function updateControl(obj) {
    updateState(obj);

    let dot_arr = [];
    getFirstKey(obj.control, dot_arr);

    let control = document.getElementById(dot_arr.join('.'));
    if (control) {
        if (typeof obj.value === "boolean") {
            control.checked = obj.value;
        } else {
            control.value = obj.value;        }
    } else {
        console.warn("Control not found: " + JSON.stringify(obj.control));
    }
}

// Called by control
function postValue(controlDot, value, endpoint) {
    let obj = {
        control: dotToObj(controlDot),
        value
    };
    updateState(obj);

    obj.client_id = uid;
    console.log(obj);

    fetch("http://127.0.0.1:8080/" + endpoint, {
        method: "POST",
        body: JSON.stringify(obj),
        headers: {
            "Content-Type": "application/json"
        }
    });
}

function initControls() {
    for (let control of document.getElementsByClassName("control")) {

        if (control.type === "range") {
            control.setAttribute("oninput", "postValue(this.id,parseInt(this.value),'u7')");
        } else if (control.type === "checkbox") {
            control.setAttribute("oninput", "postValue(this.id,this.checked,'bit')");
        }
    }
}

window.onload = initControls;

const evtSource = new EventSource("/events");
evtSource.onmessage = (event) => {

    let data;
    try { data = JSON.parse(event.data); } catch { return }

    if (data.client_id === uid) {
        return;
    }

    console.log("Received event: " + event.data);
    updateControl(data);
}