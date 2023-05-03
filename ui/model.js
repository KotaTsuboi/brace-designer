const {invoke} = window.__TAURI__.tauri;

const scene = new THREE.Scene();
const baseLength = 1;
const gl = -0.3;

let jointLength;
let mesh;
let currentMesh;
let bolts = [];
let gpl;
let currentGpl;

export async function initializeModelView() {
    console.log("initialize model view");
    const canvasElement = document.querySelector("canvas#model-view");
    const renderer = new THREE.WebGLRenderer({
        canvas: canvasElement,
        antialias: true
    });
    renderer.shadowMap.enabled = true;

    const camera = new THREE.PerspectiveCamera(45, 1.0, 0.001, 50);
    camera.position.set(1, 1, 1);
    //const cameraHelper = new THREE.CameraHelper(camera);
    //scene.add(cameraHelper);

    scene.background = new THREE.Color(0xe0e0e0);
    scene.fog = new THREE.Fog(0xe0e0e0, 2, 10);

    // lights

    const hemiLight = new THREE.HemisphereLight(0xffffff, 0x444444, 1);
    hemiLight.position.set(0, 1, 0);
    scene.add(hemiLight);
    //const hemiLightHelper = new THREE.HemisphereLightHelper(hemiLight, 1);
    //scene.add(hemiLightHelper);
    //const ambientLight = new THREE.AmbientLight(0xffffff);
    //scene.add(ambientLight);

    const dirLight = new THREE.DirectionalLight(0xffffff);
    dirLight.position.set(1, 1, -0.5);
    dirLight.castShadow = true;
    dirLight.shadow.mapSize.width = 512;
    dirLight.shadow.mapSize.height = 512;
    dirLight.shadow.camera.top = 0.5;
    dirLight.shadow.camera.bottom = -0.5;
    dirLight.shadow.camera.left = -1;
    dirLight.shadow.camera.right = 1;
    dirLight.shadow.camera.near = 1.0;
    dirLight.shadow.camera.far = 3.0;
    scene.add(dirLight);

    /*
    const dirLightShadowHelper = new THREE.CameraHelper(dirLight.shadow.camera);
    scene.add(dirLightShadowHelper);
    const dirLightHelper = new THREE.DirectionalLightHelper(dirLight);
    scene.add(dirLightHelper);
    */

    // ground

    const plane = new THREE.Mesh(
        new THREE.PlaneGeometry(10, 10),
        new THREE.MeshPhongMaterial({
            color: 0x999999,
            depthWrite: false
        })
    );
    plane.rotation.x = - Math.PI / 2;
    plane.translateZ(gl);
    plane.receiveShadow = true;
    scene.add(plane);

    const grid = new THREE.GridHelper(10, 50, 0x000000, 0x000000);
    grid.material.opacity = 0.2;
    grid.material.transparent = true;
    grid.translateY(gl);
    scene.add(grid);

    const controls = new THREE.OrbitControls(camera, canvasElement);
    // 滑らかにカメラコントロールを制御
    controls.enableDamping = true;
    controls.dampingFactor = 0.2;

    // helper
    const axesHelper = new THREE.AxesHelper(1);
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

export async function modifyModelView() {
    console.log("modify model view");
    await modifyBaseModel();
    await modifyBoltModel();
    await modifyGplModel();
}

function getColor(rate) {
    if (rate < 0) {
        throw "Rate is negative";
    } else if (rate == 0) {
        return new THREE.Color(0.5, 0.5, 0.5);
    } else if (rate < 0.33) {
        return new THREE.Color(0, rate / 0.33, 1);
    } else if (rate < 0.67) {
        return new THREE.Color(0, 1, (0.67 - rate) / 0.33);
    } else if (rate < 1) {
        return new THREE.Color((rate - 0.67) / 0.33, 1, 0);
    } else {
        return new THREE.Color(1, 0, 0);
    }
}

function getSteelMaterial(color) {
    return new THREE.MeshStandardMaterial({
        color: color,
        metalness: 0.9,
        roughness: 0.6,
        emissive: 0x222222,
    });

}

async function modifyBaseModel() {
    const shape = await getSectionShape();

    console.log("shape: " + shape);

    const geometry = await extrudeBase(shape);

    const result = await invoke("calculate_base");

    const rate = result.gamma;

    const color = getColor(rate);

    const material = getSteelMaterial(color);

    mesh = new THREE.Mesh(geometry, material);
    console.log("mesh set.");

    if (currentMesh !== undefined) {
        scene.remove(currentMesh);
    }

    mesh.translateZ(-baseLength);
    mesh.castShadow = true;

    scene.add(mesh)
    currentMesh = mesh;
}

async function getSectionShape() {
    const polyline = await invoke("get_section_in_m");

    const shape = new THREE.Shape();

    shape.moveTo(polyline.start_point[0], polyline.start_point[1]);

    for (const point of polyline.next_points) {
        shape.lineTo(point[0], point[1]);
    }

    return shape;
}

async function extrudeBase(shape) {
    jointLength = await invoke("get_joint_length_in_m");

    const extrudeSettings = {
        depth: baseLength + jointLength,
        bevelEnabled: false
    };

    const geometry = new THREE.ExtrudeGeometry(shape, extrudeSettings);

    return geometry;
}


async function modifyBoltModel() {
    const coordList = await invoke("get_bolt_coord_list_in_m");

    for (const bolt of bolts) {
        scene.remove(bolt);
    }

    const result = await invoke("calculate_bolts");

    const rate = result.gamma;

    const color = getColor(rate);

    for (const zy of coordList) {
        const geometry = await getBolt();
        const t = await invoke("get_section_thickness_in_m");

        const material = getSteelMaterial(color);

        const bolt = new THREE.Mesh(geometry, material);

        const z = zy[0];
        const y = zy[1];
        bolt.translateZ(z);
        bolt.translateY(y);
        bolt.translateX(t);

        bolt.rotation.y += Math.PI / 2;

        scene.add(bolt);
        bolts.push(bolt);
    }
}

async function getBolt() {
    const dimensions = await invoke("get_bolt_dimension_in_m");
    const h = dimensions[0];
    const b = dimensions[1];
    console.log("h: " + h);
    console.log("b: " + b);

    const shape = new THREE.Shape();
    shape.moveTo(b / 2, 0);
    shape.lineTo(b / 2, b / 2 / Math.sqrt(3));
    shape.lineTo(0, b / 2);
    shape.lineTo(-b / 2, b / 2 / Math.sqrt(3));
    shape.lineTo(-b / 2, -b / 2 / Math.sqrt(3));
    shape.lineTo(0, -b / 2);
    shape.lineTo(b / 2, -b / 2 / Math.sqrt(3));
    shape.lineTo(b / 2, 0);

    const extrudeSettings = {
        depth: h,
        bevelEnabled: false,
    };

    const geometry = new THREE.ExtrudeGeometry(shape, extrudeSettings);

    return geometry;
}

async function getGplShape() {
    const polyline = await invoke("get_gpl_shape_in_m");

    const shape = new THREE.Shape();

    shape.moveTo(polyline.start_point[0], polyline.start_point[1]);

    for (const point of polyline.next_points) {
        shape.lineTo(point[0], point[1]);
    }

    return shape;
}


async function extrudeGpl(shape) {
    let t = await invoke("get_gpl_thickness_in_m");

    const extrudeSettings = {
        depth: t,
        bevelEnabled: false
    };

    const geometry = new THREE.ExtrudeGeometry(shape, extrudeSettings);

    return geometry;
}

async function modifyGplModel() {
    const shape = await getGplShape();

    const geometry = await extrudeGpl(shape);

    const rate = await invoke("calculate_gpl");

    const color = getColor(rate);

    const material = getSteelMaterial(color);

    gpl = new THREE.Mesh(geometry, material);

    if (currentGpl !== undefined) {
        scene.remove(currentGpl);
    }

    gpl.rotation.y -= Math.PI / 2;
    gpl.castShadow = true;

    scene.add(gpl)
    currentGpl = gpl;
}
