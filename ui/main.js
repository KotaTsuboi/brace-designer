const {invoke} = window.__TAURI__.tauri;

const scene = new THREE.Scene();

let mesh;
let currentMesh;

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
        const type = el.tagName === "input" ? "input" : "change";
        el.addEventListener(type, async () => {
            await invoke("set_bolts", {
                material: boltMaterialListEl.value,
                diameter: boltDiameterListEl.value,
                num_bolts: parseInt(numBoltsSliderEl.value)
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

    // helper
    const axesHelper = new THREE.AxesHelper(1000);
    scene.add(axesHelper);

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
        const width = window.innerWidth;
        const height = window.innerHeight;

        renderer.setPixelRatio(window.devicePixelRatio);
        renderer.setSize(width, height);

        camera.aspect = width / height;
        camera.updateProjectionMatrix();
    }
}

async function modifyModelView() {
    console.log("modify model view");
    const listEl = document.querySelector("#section-list");
    const name = listEl.value;

    console.log("name: " + name);

    const shape = await getShape(name);

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

async function getShape(name) {
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

