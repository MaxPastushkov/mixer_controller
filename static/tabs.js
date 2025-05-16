function openTab(evt, name) {
    const tab_content = document.getElementsByClassName("tab-content");
    for (let i = 0; i < tab_content.length; i++) {
        tab_content[i].style.display = "none";
    }

    const tab_links = document.getElementsByClassName("tab-links");
    for (let i = 0; i < tab_links.length; i++) {
        tab_links[i].className = tab_links[i].className.replace(" active", "");
    }
    document.getElementById(name).style.display = "flex";
    evt.currentTarget.className += " active";
}

function createChannel(tab, label, fader_id, enable_id) {
    const template = document.querySelector("#channel-template");
    const clone = template.content.cloneNode(true);
    let channel = clone.querySelector(".channel");
    channel.querySelector(".channel-label").textContent = label;
    channel.querySelector(".fader").id = fader_id;
    if (enable_id) {
        channel.querySelector(".enable").id = enable_id;
    } else {
        channel.querySelector(".enable").remove();
    }

    if (tab !== "#home") {
        for (let elem of channel.querySelectorAll(".single")) {
            elem.remove();
        }
    }

    document.querySelector(tab).appendChild(clone);

    return channel;
}

function populateTab(tab, bus_id) {
    for (let chan = 1; chan <= 12; chan++) {
        createChannel(tab, chan, `BusSend.${bus_id}.CH${chan}`);
    }
    createChannel(tab, "13/14", `BusSend.${bus_id}.CH1314`);
    createChannel(tab, "15/16", `BusSend.${bus_id}.CH1516`);
    let master = createChannel(tab, "Master", `BusMaster.${bus_id}`, `BusEnable.${bus_id}`);
    master.querySelector(".fader").className += " master";
}

// Generate Home controls
for (let chan = 1; chan <= 12; chan++) {
    let channel = createChannel("#home", chan, `BusSend.StereoOut.CH${chan}`, `ChannelEnable.CH${chan}`);
    channel.querySelector(".effect1-knob").id = `BusSend.Effect1.CH${chan}`;
    channel.querySelector(".effect2-knob").id = `BusSend.Effect2.CH${chan}`;
}
let channel1314 = createChannel("#home", "13/14", "BusSend.StereoOut.CH1314", "ChannelEnable.CH1314");
channel1314.querySelector(".effect1-knob").id = `BusSend.Effect1.CH1314`;
channel1314.querySelector(".effect2-knob").id = `BusSend.Effect2.CH1314`;

let channel1516 = createChannel("#home", "15/16", "BusSend.StereoOut.CH1516", "ChannelEnable.CH1516");
channel1516.querySelector(".effect1-knob").id = `BusSend.Effect1.CH1516`;
channel1516.querySelector(".effect2-knob").id = `BusSend.Effect2.CH1516`;
console.log("here1");

let master = createChannel("#home", "Master", "BusMaster.StereoOut", "BusEnable.StereoOut");
master.querySelector(".fader").className += " master";
master.querySelectorAll(".knob-wrapper").forEach((e) => {e.remove()});

// Generate the rest
populateTab("#aux1", "Aux1");
populateTab("#aux2", "Aux2");
populateTab("#aux3", "Aux3");
populateTab("#aux4", "Aux4");