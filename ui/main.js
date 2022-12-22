const {invoke} = window.__TAURI__.tauri;
import {initializeModelView, modifyModelView} from "./model.js";

window.addEventListener("DOMContentLoaded", async () => {
    await addOptions("#section-list", "list_sections");
    await addOptions("#material-list", "list_materials");
    await addOptions("#bolt-material-list", "list_bolt_materials");
    await addOptions("#bolt-diameter-list", "list_bolt_diameters");
    await addOptions("#gpl-material-list", "list_materials");
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
    numBoltsSliderEl.addEventListener("input", () => modifyLabelValue(numBoltsSliderEl));

    const gplPropertiesEl = document.querySelectorAll(".gpl-properties");
    gplPropertiesEl.forEach((el) => {
        const gplMaterialListEl = document.querySelector("#gpl-material-list");
        const gplThicknessSliderEl = document.querySelector("#gpl-thickness-slider");
        const gplLgSliderEl = document.querySelector("#gpl-lg-slider");

        const type = el.tagName.toLowerCase() === "input" ? "input" : "change";
        el.addEventListener(type, async () => {
            await invoke("set_gpl", {
                materialName: gplMaterialListEl.value,
                thickness: gplThicknessSliderEl.value,
                lg: gplLgSliderEl.value
            });
            modifyModelView();
        });
    });

    const gplThicknessSliderEl = document.querySelector("#gpl-thickness-slider");
    const gplLgSliderEl = document.querySelector("#gpl-lg-slider");
    gplThicknessSliderEl.addEventListener("input", () => modifyLabelValue(gplLgSliderEl));
    gplLgSliderEl.addEventListener("input", () => modifyLabelValue(gplLgSliderEl));

    const loadSliderEl = document.querySelector("#short-load-slider");
    loadSliderEl.addEventListener("input", async () => {
        modifyLabelValue(loadSliderEl);
        await invoke("set_force_in_kn", {value: parseFloat(loadSliderEl.value)});
        modifyModelView();
    });
}

function modifyLabelValue(el) {
    console.log("el:");
    console.log(el)

    console.log("parent:");
    console.log(el.parentNode);

    console.log("next:");
    console.log(el.parentNode.nextElementSibling);

    console.log("child:");
    console.log(el.parentNode.nextElementSibling.firstElementChild);

    const labelEl = el.parentNode.nextElementSibling.firstElementChild;
    labelEl.innerHTML = el.value;
}
