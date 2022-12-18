const {invoke} = window.__TAURI__.tauri;
import {initializeModelView, modifyModelView} from "./model.js";

window.addEventListener("DOMContentLoaded", async () => {
    await addOptions("#section-list", "list_sections");
    await addOptions("#material-list", "list_materials");
    await addOptions("#bolt-material-list", "list_bolt_materials");
    await addOptions("#bolt-diameter-list", "list_bolt_diameters");
    addEventListenerToProperties();
    initializeModelView();
});

async function addOptions(id, command) {
    const listEl = document.querySelector(id);
    const list = await invoke(command);
    appendChildToList(listEl, list);
}

function appendChildToList(listEl, list) {
    for (const value of list) {
        console.log("child: " + value);
        const opt = document.createElement("option");
        opt.value = value;
        opt.innerHTML = value;
        listEl.appendChild(opt);
    }
}

function addEventListenerToProperties() {
    const sectionListEl = document.querySelector("#section-list");
    sectionListEl.addEventListener("change", async () => {
        await invoke("set_section", {name: sectionListEl.value});
        modifyModelView();
    });

    const materialListEl = document.querySelector("#material-list");
    materialListEl.addEventListener("change", async () => {
        await invoke("set_material", {name: materialListEl.value});
        modifyModelView();
    });

    const boltPropertiesEl = document.querySelectorAll(".bolt-properties");
    boltPropertiesEl.forEach((el) => {
        const boltMaterialListEl = document.querySelector("#bolt-material-list");
        const boltDiameterListEl = document.querySelector("#bolt-diameter-list");
        const numBoltsSliderEl = document.querySelector("#num-bolts-slider");
        const type = el.tagName.toLowerCase() === "input" ? "input" : "change";
        console.log("tagname:");
        console.log(el.tagName);
        el.addEventListener(type, async () => {
            await invoke("set_bolts", {
                materialName: boltMaterialListEl.value,
                diameterName: boltDiameterListEl.value,
                numBolts: parseInt(numBoltsSliderEl.value)
            });
            modifyModelView();
        });
    });

    const numBoltsSliderEl = document.querySelector("#num-bolts-slider");
    numBoltsSliderEl.addEventListener("input", () => modifyBoltNum());

    const loadSliderEl = document.querySelector("#short-load-slider");
    loadSliderEl.addEventListener("input", async () => {
        modifyLoadValue();
        await invoke("set_force_in_kn", {value: parseFloat(loadSliderEl.value)});
        modifyModelView();
    });
}

function modifyBoltNum() {
    const sliderEl = document.querySelector("#num-bolts-slider");
    const labelEl = document.querySelector("#num-bolts-value");
    labelEl.innerHTML = sliderEl.value;
}

function modifyLoadValue() {
    const sliderEl = document.querySelector("#short-load-slider");
    const labelEl = document.querySelector("#short-load-value");
    labelEl.innerHTML = sliderEl.value;
}

