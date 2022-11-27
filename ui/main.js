const {invoke} = window.__TAURI__.tauri;

const scene = new THREE.Scene();

let mesh;
let currentMesh;

async function setSection() {
    const listEl = document.querySelector("#section-list");
    //const id = listEl.selectedIndex;
    const name = listEl.value;

    const isAngle = await invoke("is_angle_section", {name: name});
    if (isAngle) {
        invoke("set_angle_section", {name: name});
        return;
    }

    const isChannel = await invoke("is_channel_section", {name: name});
    if (isChannel) {
        invoke("set_channel_section", {name: name});
        return;
    }
}

async function modifyModelView() {
    console.log("modify model view");
    const listEl = document.querySelector("#section-list");
    const name = listEl.value;

    console.log("name: " + name);

    const isAngle = await invoke("is_angle_section", {name: name});

    if (!isAngle) {
        return;
    }

    const a = await invoke("get_angle_a_as_mm");
    const b = await invoke("get_angle_b_as_mm");
    const t = await invoke("get_angle_t_as_mm");
    console.log("a: " + a + ", b: " + b + ", t: " + t);

    const l = 1000;

    const geometry = angleSteel(a, b, t, l);

    const material = new THREE.MeshNormalMaterial();
    mesh = new THREE.Mesh(geometry, material);

    if (typeof currentMesh !== undefined) {
        scene.remove(currentMesh);
    }

    scene.add(mesh)
    currentMesh = mesh;
}

async function addSectionOptions() {
    console.log("add section options");
    const listEl = document.querySelector("#section-list");
    const list = await invoke("list_sections");

    for (const section of list) {
        console.log("section: " + section);
        const opt = document.createElement("option");
        opt.value = section;
        opt.innerHTML = section;
        listEl.appendChild(opt);
    }
}

async function initializeModelView() {
    console.log("initialize model view");
    const canvasElement = document.querySelector("canvas#model-view");
    const renderer = new THREE.WebGLRenderer({
        canvas: canvasElement,
        antialias: true
    });

    const camera = new THREE.PerspectiveCamera(45, 1.0, 1, 1000000);
    camera.position.set(0, 0, +1000);

    const controls = new THREE.OrbitControls(camera, canvasElement);
    // 滑らかにカメラコントロールを制御
    controls.enableDamping = true;
    controls.dampingFactor = 0.2;

    const light = new THREE.DirectionalLight(0xFFFFFF, 1);
    light.position.set(0, 0, 1000000);
    scene.add(light);
    const ambientLight = new THREE.AmbientLight(0xFFFFFF, 0.3);
    scene.add(ambientLight);

    await modifyModelView();

    tick();

    function tick() {
        controls.update();
        mesh.rotation.y += 0.01;
        renderer.render(scene, camera);
        requestAnimationFrame(tick);
    }

    onResize();

    window.addEventListener('resize', onResize);

    function onResize() {
        const width = window.innerWidth;
        const height = window.innerHeight;

        renderer.setPixelRatio(window.devicePixelRatio);
        renderer.setSize(width, height);

        camera.aspect = width / height;
        camera.updateProjectionMatrix();
    }
}

function addEventListenerToSelect() {
    const listEl = document.querySelector("#section-list");
    listEl.addEventListener("change", () => {
        setSection();
        modifyModelView();
    });
    const sliderEl = document.querySelector("#short-load");
    sliderEl.addEventListener("input", () => {
        modifyModelView();
    });
}

window.addEventListener("DOMContentLoaded", async () => {
    await addSectionOptions();
    addEventListenerToSelect();
    initializeModelView();
});

function angleSteelShape(a, b, t) {
    const shape = new THREE.Shape();

    shape.moveTo(0, 0);
    shape.lineTo(a, 0);
    shape.lineTo(a, t);
    shape.lineTo(t, t);
    shape.lineTo(t, b);
    shape.lineTo(0, b);

    return shape;
}

function angleSteel(a, b, t, l) {
    const shape = angleSteelShape(a, b, t);

    const extrudeSettings = {
        depth: l,
        bevelEnabled: false
    };

    const geometry = new THREE.ExtrudeGeometry(shape, extrudeSettings);

    return geometry;
}

