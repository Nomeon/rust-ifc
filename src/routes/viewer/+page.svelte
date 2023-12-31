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
    let ifcModels: IFCModel[] = [];
    let properties: ModelProperties[] = [];
    let fileInput: HTMLInputElement;

    onMount(async() => {
        setupThree();
        await setupLoader();
        animate();
    })

    function setupThree() {
        scene = new THREE.Scene();

        camera = new THREE.PerspectiveCamera(45, (w/h), 0.1, 1000);
        camera.position.set(15, 10, 8);

        const lightColor = 0xffeeff;
        const directionalLight = new THREE.DirectionalLight(lightColor, 1);
        directionalLight.position.set(0, 10, 0);
        directionalLight.target.position.set(-5, 0, 0);
        scene.add(directionalLight);
        scene.add(directionalLight.target);
        scene.add(new THREE.AmbientLight(lightColor, 0.5));

        const threeCanvas = <HTMLCanvasElement> document.getElementById('viewer-canvas');
        renderer = new THREE.WebGLRenderer({ canvas: threeCanvas, alpha: true, antialias: true });
        renderer.setSize(w, h);
        renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));

        scene.add(new THREE.GridHelper(50, 30));

        controls = new OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;
        controls.target.set(-2, 0, 0);
    }

    async function setupLoader() {
        ifcLoader = new IFCLoader();
        await ifcLoader.ifcManager.applyWebIfcConfig({
            COORDINATE_TO_ORIGIN: false,
            USE_FAST_BOOLS: true,
        });
        await ifcLoader.ifcManager.useWebWorkers(true, "IFCWorker.js")
    }

    const animate = () => {
        controls.update();
        renderer.render(scene, camera);
        requestAnimationFrame(animate);
    }
    
    async function parseIfc(url: string) {
        await new Promise<void>((resolve) => {
            ifcLoader.load(url, async (ifcModel) => {
                ifcModels.push(ifcModel);
                scene.add(ifcModel);
                ifcModel.translateX(-10);
                ifcModel.translateZ(10);
                ifcModel.translateY(0.3);
                ifcModel.updateMatrixWorld(true);

                properties.push(await getProperties(ifcModel.modelID))
                resolve();
            });
        });
    }

    async function getProperties(id: number): Promise<ModelProperties> {
        const lines: any[] = await ifcLoader.ifcManager.getAllItemsOfType(id, IFCRELDEFINESBYPROPERTIES, false);
        const promises = lines.map(async (line) => {
            const relation = await ifcLoader.ifcManager.getItemProperties(id, line);
            const element = await ifcLoader.ifcManager.getItemProperties(id, relation.RelatedObjects[0].value);
            const property = (await ifcLoader.ifcManager.getItemProperties(id, relation.RelatingPropertyDefinition.value, true)).HasProperties.reduce((obj: { [x: string]: any; }, curr: { Name: { value: string | number; }; NominalValue: { value: any; }; }) => {
                obj[curr.Name.value] = curr.NominalValue.value;
                return obj;
            }, {});
            const prop: Properties = { 
                express_id: element.expressID, 
                name: element.Name.value, 
                ifc_type: element.constructor.name,
                productcode: property.Productcode,
                projectnummer: property.Projectnummer,
                klant: property.Klant,
                station: property.Station,
                bouwnummer: property.Bouwnummer,
                modulenaam: property.Modulenaam,
                moduletype: property.Moduletype,
                categorie: property.Categorie,
                materiaal: property.Materiaal,
                dikte: property.Dikte,
                breedte: property.Breedte,
                lengte: property.Lengte,
                gewicht: property.Gewicht,
                volume: property.Volume,
                eenheid: property.Eenheid,
                ifc_bestand: property["IFC bestand"], 
                aantal: property.Aantal,
            };
            return prop;
        });
        const props = await Promise.all(promises);
        const modelProperties = { model_id: id, properties: props };
        return modelProperties;
    }

    const loadIFCs = async (event: any) => {
        const files = event.target.files;
        const promises = [];
        for (let i = 0; i < files.length; i++) {
            let ifcURL = URL.createObjectURL(files[i]);
            promises.push(await parseIfc(ifcURL));
        }
        await Promise.all(promises);
        const propertiesArray = properties.flatMap(obj => obj.properties);
        const json = JSON.stringify(propertiesArray);
        const path = "C:/Users/DN51/OneDrive - TBI Holding/Bureaublad/rust/polars-test/"
        if (propertiesArray.length > 0) {
            console.log(await invoke('parse_json', { json, path }))
        }
    }

    function onResize() {
        camera.aspect = w / h;
        camera.updateProjectionMatrix();
        renderer.setSize(w, h);
    }
</script>
<h1>Test</h1>
<a href="/">Home</a>
<input style="display:none" type="file" multiple accept=".ifc" on:change={(e)=>loadIFCs(e)} bind:this={fileInput} >
<button type='button' on:click={()=>{fileInput.click()}}>Load IFCs</button>
<div bind:clientWidth={w} bind:clientHeight={h} id='container' class='w-full max-h-max'>
    <canvas id='viewer-canvas' class='w-full h-full'></canvas>
</div>
<svelte:window on:resize={onResize} />
