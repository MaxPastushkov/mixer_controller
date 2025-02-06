
class Bus {
    CH1 = 0;
    CH2 = 0;
    CH3 = 0;
    CH4 = 0;
    CH5 = 0;
    CH6 = 0;
    CH7 = 0;
    CH8 = 0;
    CH9 = 0;
    CH10 = 0;
    CH11 = 0;
    CH12 = 0;
    CH1314 = 0;
    CH1516 = 0;
    Return1 = 0;
    Return2 = 0;
}

class EqControl {

}

class EqBus {
    CH1 = 0;
    CH2 = 0;
    CH3 = 0;
    CH4 = 0;
    CH5 = 0;
    CH6 = 0;
    CH7 = 0;
    CH8 = 0;
    CH9 = 0;
    CH10 = 0;
    CH11 = 0;
    CH12 = 0;
    CH1314 = 0;
    CH1516 = 0;
    Return1 = 0;
    Return2 = 0;
    StereoOut = 0;
    Aux1 = 0;
    Aux2 = 0;
    Aux3 = 0;
    Aux4 = 0;
}

class OnControl {
    CH1 = false;
    CH2 = false;
    CH3 = false;
    CH4 = false;
    CH5 = false;
    CH6 = false;
    CH7 = false;
    CH8 = false;
    CH9 = false;
    CH10 = false;
    CH11 = false;
    CH12 = false;
    CH1314 = false;
    CH1516 = false;
    Return1 = false;
    Return2 = false;
    StereoOut = false;
    Aux1 = false;
    Aux2 = false;
    Aux3 = false;
    Aux4 = false;
}

let mixerState = {
    BusSend: {
        StereoOut: new Bus(),
        Aux1: new Bus(),
        Aux2: new Bus(),
        Aux3: new Bus(),
        Aux4: new Bus(),
        Effect1: new Bus(),
        Effect2: new Bus()
    },
    BusMaster: {
        StereoOut: 0,
        Aux1: 0,
        Aux2: 0,
        Aux3: 0,
        Aux4: 0,
        Effect1: 0,
        Effect2: 0
    },
    Channel: new OnControl(),
    Effect1Send: false,
    Effect2Send: false
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
        control.value = obj.value;
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