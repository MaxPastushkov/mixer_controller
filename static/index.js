
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

    constructor(initValue) {
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
        this.Return1 = initValue;
        this.Return2 = initValue;
    }
}

let mixerState = {
    BusSend: {
        StereoOut: new Channels(0),
        Aux1: new Channels(0),
        Aux2: new Channels(0),
        Aux3: new Channels(0),
        Aux4: new Channels(0),
        Effect1: new Channels(0),
        Effect2: new Channels(0)
    },
    BusMaster: new Busses(0),
    ChannelEnable: new Channels(false),
    BusEnable: new Busses(false)
};
delete mixerState.BusSend.Effect1.Return1;
delete mixerState.BusSend.Effect2.Return2;

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

    let control = document.getElementById(JSON.stringify(obj.control));
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
function postValue(control, value, endpoint) {
    const obj = {control, value};
    updateState(obj);

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
            control.setAttribute("oninput", "postValue(" + control.id + ",parseInt(this.value),'u7')");
        } else if (control.type === "checkbox") {
            control.setAttribute("oninput", "postValue(" + control.id + ",this.checked,'bit')");
        }
    }
}

window.onload = initControls;

const evtSource = new EventSource("/events");
evtSource.onmessage = (event) => {
    try {
        updateControl(JSON.parse(event.data));
        console.log("Received event: " + event.data);
    } catch {}
}