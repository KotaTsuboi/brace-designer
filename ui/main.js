const {invoke} = window.__TAURI__.tauri;
import {initializeModelView, modifyModelView} from "./model.js";

const sectionListEl = document.querySelector("#section-list");
const materialListEl = document.querySelector("#material-list");

const boltMaterialListEl = document.querySelector("#bolt-material-list");
const boltDiameterListEl = document.querySelector("#bolt-diameter-list");
const numBoltsSliderEl = document.querySelector("#num-bolts-slider");

const gplMaterialListEl = document.querySelector("#gpl-material-list");
const gplThicknessSliderEl = document.querySelector("#gpl-thickness-slider");
const gplLgSliderEl = document.querySelector("#gpl-lg-slider");

const loadSliderEl = document.querySelector("#short-load-slider");

window.addEventListener("DOMContentLoaded", async () => {
    await initializeProperties();
    initializeModelView();
});

async function initializeProperties() {
    await addOptions(sectionListEl, "list_sections");
    await addOptions(materialListEl, "list_materials");
    await addOptions(boltMaterialListEl, "list_bolt_materials");
    await addOptions(boltDiameterListEl, "list_bolt_diameters");
    await addOptions(gplMaterialListEl, "list_materials");

    addEventListenerToProperties();

    modifyLabelValue(numBoltsSliderEl);
    modifyLabelValue(gplThicknessSliderEl);
    modifyLabelValue(gplLgSliderEl);
    modifyLabelValue(loadSliderEl);
}

async function addOptions(listEl, command) {
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
    sectionListEl.addEventListener("change", async () => {
        await invoke("set_section", {name: sectionListEl.value});
        modifyModelView();
    });

    materialListEl.addEventListener("change", async () => {
        await invoke("set_material", {name: materialListEl.value});
        modifyModelView();
    });

    const boltPropertiesEl = document.querySelectorAll(".bolt-properties");
    boltPropertiesEl.forEach((el) => {

        const type = el.tagName.toLowerCase() === "input" ? "input" : "change";
        el.addEventListener(type, async () => {
            await invoke("set_bolts", {
                materialName: boltMaterialListEl.value,
                diameterName: boltDiameterListEl.value,
                numBolts: parseInt(numBoltsSliderEl.value)
            });
            modifyModelView();
        });
    });

    numBoltsSliderEl.addEventListener("input", () => modifyLabelValue(numBoltsSliderEl));

    const gplPropertiesEl = document.querySelectorAll(".gpl-properties");
    gplPropertiesEl.forEach((el) => {

        const type = el.tagName.toLowerCase() === "input" ? "input" : "change";
        el.addEventListener(type, async () => {
            await invoke("set_gpl", {
                thickness: parseFloat(gplThicknessSliderEl.value),
                lg: parseFloat(gplLgSliderEl.value),
                materialName: gplMaterialListEl.value
            });
            modifyModelView();
        });
    });

    gplThicknessSliderEl.addEventListener("input", () => modifyLabelValue(gplThicknessSliderEl));
    gplLgSliderEl.addEventListener("input", () => modifyLabelValue(gplLgSliderEl));

    loadSliderEl.addEventListener("input", async () => {
        modifyLabelValue(loadSliderEl);
        await invoke("set_force_in_kn", {value: parseFloat(loadSliderEl.value)});
        modifyModelView();
    });
}

function modifyLabelValue(sliderEl) {
    const labelEl = sliderEl.parentNode.nextElementSibling.firstElementChild;
    labelEl.innerHTML = sliderEl.value;
}
