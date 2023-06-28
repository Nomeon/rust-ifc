<script lang="ts">
    // @ts-ignore
    import * as THREE from "three";
    // @ts-ignore
    import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
    import { onMount } from 'svelte';
    import { IFCLoader } from "web-ifc-three/IFCLoader";
    import { invoke } from '@tauri-apps/api/tauri';
    import { IFCRELDEFINESBYPROPERTIES } from "web-ifc/web-ifc-api";
    import type { IFCModel } from "web-ifc-three/IFC/components/IFCModel";

    let w: number;
    let h: number;
    let scene: THREE.Scene;
    let camera: THREE.PerspectiveCamera;
    let controls: OrbitControls;
    let renderer: THREE.WebGLRenderer;
    let ifcLoader: IFCLoader;
    let ifcModel: IFCModel;
    let properties: Properties[] = [];
    let name = 'world';
    
    onMount(async() => {
        console.log(await invoke('greet', { name }))
        setupThree();
        setupLoader();
        animate();
    })

    function setupThree() {
        scene = new THREE.Scene();

        const aspect = w / h;
        camera = new THREE.PerspectiveCamera(45, aspect, 0.1, 1000);
        camera.position.set(15, 10, 8);

        const lightColor = 0xffeeff;

        const ambientLight = new THREE.AmbientLight(lightColor, 0.5);
        scene.add(ambientLight);

        const directionalLight = new THREE.DirectionalLight(lightColor, 1);
        directionalLight.position.set(0, 10, 0);
        directionalLight.target.position.set(-5, 0, 0);
        scene.add(directionalLight);
        scene.add(directionalLight.target);

        const threeCanvas = <HTMLCanvasElement> document.getElementById('viewer-canvas');
        renderer = new THREE.WebGLRenderer({ canvas: threeCanvas, alpha: true, antialias: true });
        renderer.setSize(w, h);
        renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));

        const grid = new THREE.GridHelper(50, 30)
        scene.add(grid);

        controls = new OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        controls.target.set(-2, 0, 0);
    }

    async function setupLoader() {
        ifcLoader = new IFCLoader();
        ifcLoader.ifcManager.applyWebIfcConfig({
            COORDINATE_TO_ORIGIN: true,
            USE_FAST_BOOLS: true,
        });
    }

    const animate = () => {
        controls.update();
        renderer.render(scene, camera);
        requestAnimationFrame(animate);
    }
    
    async function loadIfc(url: string = "test.ifc") {
        const props: any[] = [];
        await new Promise<void>((resolve) => {
            ifcLoader.load(url, async (model) => {
            ifcModel = model;
            scene.add(model);
            const lines: any[] = await ifcLoader.ifcManager.getAllItemsOfType(0, IFCRELDEFINESBYPROPERTIES, false);
            const promises = lines.map(async (line) => {
                const relation = await ifcLoader.ifcManager.getItemProperties(0, line);
                const element = await ifcLoader.ifcManager.getItemProperties(0, relation.RelatedObjects[0].value);
                const property = (await ifcLoader.ifcManager.getItemProperties(0, relation.RelatingPropertyDefinition.value, true)).HasProperties.map((obj: { Name: { value: any; }; NominalValue: { value: any; }; expressID: any; }) => ({
                [obj.Name.value]: obj.NominalValue.value,
                id: obj.expressID
                }));
                const elemProp = { ExpressID: element.expressID, Name: element.Name.value, Type: element.constructor.name, properties: property };
                return flattenObject(elemProp);
            });
            const flattenedProps = await Promise.all(promises);
            props.push(...flattenedProps);
            properties = props;
            resolve();
            });
        });
    }

    function flattenObject(obj: any): {[key: string]: any} {
        return Object.entries(obj).reduce((result: {[key: string]: any}, [key, value]: [string, any]) => {
            if (Array.isArray(value)) {
                value.forEach((nestedObj: {[key: string]: any}) => {
                    Object.entries(nestedObj).forEach(([nestedKey, nestedValue]: [string, any]) => {
                        result[nestedKey] = nestedValue;
                    });
                });
            } else {
            result[key] = value;
            }
            return result;
        }, {});
    }

    function onResize() {
        camera.aspect = w / h;
        camera.updateProjectionMatrix();
        renderer.setSize(w, h);
    }
</script>
<h1>Test</h1>
<a href="/">Home</a>
<button on:click={() => loadIfc("test.ifc")}>Load IFC</button>
<div bind:clientWidth={w} bind:clientHeight={h} id='container'>
    <canvas id='viewer-canvas'></canvas>
</div>
<svelte:window on:resize={onResize} />

<style>
    #viewer-canvas {
        width: 100%;
        height: 100%;
    }

    #container {
        width: 100%;
        height: 600px;
    }
</style>