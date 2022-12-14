const {invoke} = window.__TAURI__.tauri;

const scene = new THREE.Scene();
const baseLength = 1000;

let jointLength;
let mesh;
let currentMesh;
let bolts = [];

export async function initializeModelView() {
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
    plane.translateY(-100);
    scene.add(plane);

    const grid = new THREE.GridHelper(20000, 100, 0x000000, 0x000000);
    grid.material.opacity = 0.2;
    grid.material.transparent = true;
    grid.translateY(-100);
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

export async function modifyModelView() {
    console.log("modify model view");
    await modifyBaseModel();
    await modifyBoltModel();
}

async function modifyBaseModel() {
    const shape = await getSectionShape();

    console.log("shape: " + shape);

    const geometry = await extrudeBase(shape);

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

    mesh = new THREE.Mesh(geometry, material);
    console.log("mesh set.");

    if (currentMesh !== undefined) {
        scene.remove(currentMesh);
    }

    mesh.translateZ(-baseLength);

    scene.add(mesh)
    currentMesh = mesh;
}

async function getSectionShape() {
    const polyline = await invoke("get_section_in_mm");

    console.log("polyline:");
    console.log(polyline)

    const shape = new THREE.Shape();

    shape.moveTo(polyline.start_point[0], polyline.start_point[1]);

    for (const point of polyline.next_points) {
        shape.lineTo(point[0], point[1]);
    }

    return shape;
}

async function extrudeBase(shape) {
    jointLength = await invoke("get_joint_length_in_mm");

    const extrudeSettings = {
        depth: baseLength + jointLength,
        bevelEnabled: false
    };

    const geometry = new THREE.ExtrudeGeometry(shape, extrudeSettings);

    return geometry;
}


async function modifyBoltModel() {
    const coordList = await invoke("get_bolt_coord_list_in_mm");
    console.log("coord list:")
    console.log(coordList);

    for (const bolt of bolts) {
        scene.remove(bolt);
    }

    for (const zy of coordList) {
        const geometry = await getBolt();
        console.log("geometry:")
        console.log(geometry);

        const material = new THREE.MeshStandardMaterial({
            color: 0x999999,
            metalness: 1,
            roughness: 0.9,
        });
        console.log("material:")
        console.log(material);

        const bolt = new THREE.Mesh(geometry, material);
        console.log("mesh:")
        console.log(bolt);

        const z = zy[0];
        console.log("z: " + z);
        const y = zy[1];
        console.log("y: " + y);
        bolt.translateZ(z);
        bolt.translateY(y);

        bolt.rotation.y += Math.PI / 2;

        scene.add(bolt);
        bolts.push(bolt);
    }
}

async function getBolt() {
    const dimensions = await invoke("get_bolt_dimension_in_mm");
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

    console.log("shape:");
    console.log(shape);

    const extrudeSettings = {
        depth: h,
        bavelEnabled: false
    };

    const geometry = new THREE.ExtrudeGeometry(shape, extrudeSettings);

    return geometry;
}
