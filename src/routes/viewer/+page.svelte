<script lang="ts">
    // @ts-ignore
    import * as THREE from "three";
    // @ts-ignore
    import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
    import { onMount } from 'svelte';
    import { IFCLoader } from "web-ifc-three/IFCLoader";
    import { invoke } from '@tauri-apps/api/tauri';
    import type { IFCModel } from 'web-ifc-three/IFC/components/IFCModel';

    let w: number;
    let h: number;
    let scene: THREE.Scene;
    let camera: THREE.PerspectiveCamera;
    let controls: OrbitControls;
    let renderer: THREE.WebGLRenderer;
    let ifcLoader: IFCLoader;
    let ifcModel: IFCModel;
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
        renderer = new THREE.WebGLRenderer({ canvas: threeCanvas, alpha: true });
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
    
    const loadIfc = async (event: any) => {
        // const file = event.target.files[0];
        console.log(ifcLoader)
        ifcLoader.load("test.ifc", (model) => {
            scene.add(model);
        });
    }
    
    function onResize() {
        camera.aspect = w / h;
        camera.updateProjectionMatrix();
        renderer.setSize(w, h);
    }
</script>
<h1>Test</h1>
<a href="/">Home</a>
<button on:click={loadIfc}>Load IFC</button>
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