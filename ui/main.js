const {invoke} = window.__TAURI__.tauri;

const scene = new THREE.Scene();

let mesh;
let currentMesh;

async function setSection() {
    const listEl = document.querySelector("#section-list");

    const name = listEl.value;

    await invoke("set_section", {name: name});
}

async function setMaterial() {
    const listEl = document.querySelector("#material-list");

    const name = listEl.value;

    await invoke("set_material", {name: name});
}

async function modifyModelView() {
    console.log("modify model view");
    const listEl = document.querySelector("#section-list");
    const name = listEl.value;

    console.log("name: " + name);

    const shape = await angleSteelShape(name);

    console.log("shape: " + shape);

    const geometry = extrudeGeometry(shape);

    const rate = await invoke("calculate");

    let color;
    if (rate > 1) {
        color = 0xFF0000
    } else {
        color = 0x999999;
    }

    const material = new THREE.MeshStandardMaterial({
        color: color,
        metalness: 1,
        roughness: 0.9,
    });

    let tmp;
    if (mesh !== undefined) {
        console.log("mesh is undefined.");
        tmp = mesh.rotation.y;
    }
    mesh = new THREE.Mesh(geometry, material);
    console.log("mesh set.");
    if (tmp !== undefined) {
        mesh.rotation.y = tmp;
    }

    if (currentMesh !== undefined) {
        scene.remove(currentMesh);
    }

    mesh.translateY(100);

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

async function addMaterialOptions() {
    console.log("add material options");
    const listEl = document.querySelector("#material-list");
    const list = await invoke("list_materials");

    for (const material of list) {
        console.log("material: " + material);
        const opt = document.createElement("option");
        opt.value = material;
        opt.innerHTML = material;
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
    camera.position.set(1000, 1000, 1000);

    scene.background = new THREE.Color(0xe0e0e0);
    scene.fog = new THREE.Fog(0xe0e0e0, 2000, 10000);

    // lights

    const hemiLight = new THREE.HemisphereLight(0xffffff, 0x444444);
    hemiLight.position.set(0, 2000, 0);
    scene.add(hemiLight);

    const dirLight = new THREE.DirectionalLight(0xffffff);
    dirLight.position.set(0, 2000, 1000);
    scene.add(dirLight);

    // ground

    const plane = new THREE.Mesh(new THREE.PlaneGeometry(200000, 200000), new THREE.MeshPhongMaterial({color: 0x999999, depthWrite: false}));
    plane.rotation.x = - Math.PI / 2;
    scene.add(plane);

    const grid = new THREE.GridHelper(20000, 100, 0x000000, 0x000000);
    grid.material.opacity = 0.2;
    grid.material.transparent = true;
    scene.add(grid);

    const controls = new THREE.OrbitControls(camera, canvasElement);
    // 滑らかにカメラコントロールを制御
    controls.enableDamping = true;
    controls.dampingFactor = 0.2;

    await modifyModelView();

    tick();

    function tick() {
        controls.update();
        //mesh.rotation.y += 0.01;
        renderer.render(scene, camera);
        requestAnimationFrame(tick);
    }

    onResize();

    window.addEventListener('resize', onResize);

    function onResize() {
        const width = window.innerWidth * 0.6;
        const height = window.innerHeight;

        renderer.setPixelRatio(window.devicePixelRatio);
        renderer.setSize(width, height);

        camera.aspect = width / height;
        camera.updateProjectionMatrix();
    }
}

async function setLoadValue() {
    const sliderEl = document.querySelector("#short-load-slider");
    const value = parseFloat(sliderEl.value);
    await invoke("set_force_in_kn", {value: value});
}

function addEventListenerToProperties() {
    const listEl = document.querySelector("#section-list");
    listEl.addEventListener("change", () => {
        setSection();
        modifyModelView();
    });

    const materialListEl = document.querySelector("#material-list");
    materialListEl.addEventListener("change", () => {
        setMaterial();
        modifyModelView();
    });

    const sliderEl = document.querySelector("#short-load-slider");
    sliderEl.addEventListener("input", () => {
        modifyLoadValue();
        setLoadValue();
        modifyModelView();
    });
}

function modifyLoadValue() {
    const sliderEl = document.querySelector("#short-load-slider");
    const labelEl = document.querySelector("#short-load-value");
    labelEl.innerHTML = sliderEl.value;
}

window.addEventListener("DOMContentLoaded", async () => {
    await addSectionOptions();
    await addMaterialOptions();
    addEventListenerToProperties();
    initializeModelView();
});

async function angleSteelShape(name) {
    const polyline = await invoke("get_section_in_mm", {name: name});

    const shape = new THREE.Shape();

    shape.moveTo(polyline.start_point[0], polyline.start_point[1]);

    for (point of polyline.next_points) {
        shape.lineTo(point[0], point[1]);
    }

    return shape;
}

function extrudeGeometry(shape) {
    const extrudeSettings = {
        depth: 1000,
        bevelEnabled: false
    };

    const geometry = new THREE.ExtrudeGeometry(shape, extrudeSettings);

    return geometry;
}

