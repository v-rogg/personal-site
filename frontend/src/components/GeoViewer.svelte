<script lang="ts">
	import { onMount } from "svelte";
	import * as THREE from "three";
	import { OrbitControls } from "three/addons/controls/OrbitControls.js";
	import { OBJLoader } from "three/addons/loaders/OBJLoader.js";

	let canvas: HTMLCanvasElement;

	onMount(() => {
		const container = canvas.parentElement!;
		const scene = new THREE.Scene();
		scene.background = new THREE.Color(0xf7f4f2);

		const camera = new THREE.PerspectiveCamera(45, container.clientWidth / container.clientHeight, 0.1, 10000);
		const renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
		renderer.setSize(container.clientWidth, container.clientHeight);
		renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
		renderer.shadowMap.enabled = true;
		renderer.shadowMap.type = THREE.PCFSoftShadowMap;

		// Lighting
		const ambient = new THREE.AmbientLight(0xffffff, 0.6);
		scene.add(ambient);

		const sun = new THREE.DirectionalLight(0xffffff, 0.9);
		sun.position.set(150, 300, 200);
		sun.castShadow = true;
		sun.shadow.mapSize.width = 2048;
		sun.shadow.mapSize.height = 2048;
		sun.shadow.camera.near = 0.5;
		sun.shadow.camera.far = 2000;
		sun.shadow.camera.left = -500;
		sun.shadow.camera.right = 500;
		sun.shadow.camera.top = 500;
		sun.shadow.camera.bottom = -500;
		scene.add(sun);

		const fillLight = new THREE.DirectionalLight(0x8ec8f0, 0.3);
		fillLight.position.set(-100, 50, -100);
		scene.add(fillLight);

		// Controls
		const controls = new OrbitControls(camera, renderer.domElement);
		controls.enableDamping = true;
		controls.dampingFactor = 0.05;
		controls.minDistance = 20;
		controls.maxDistance = 2000;
		controls.maxPolarAngle = Math.PI / 2.05;

		// Ground plane
		const groundGeo = new THREE.PlaneGeometry(5000, 5000);
		const groundMat = new THREE.ShadowMaterial({ opacity: 0.08 });
		const ground = new THREE.Mesh(groundGeo, groundMat);
		ground.rotation.x = -Math.PI / 2;
		ground.receiveShadow = true;
		scene.add(ground);

		// Load OBJ
		const loader = new OBJLoader();
		loader.load("/files/kinsau.obj", (obj: THREE.Group) => {
			// Geo/CAD export has Z-up — rotate to Three.js Y-up
			obj.rotation.x = -Math.PI / 2;
			obj.updateMatrixWorld(true);

			const box = new THREE.Box3().setFromObject(obj);
			const center = box.getCenter(new THREE.Vector3());
			const size = box.getSize(new THREE.Vector3());
			obj.position.sub(center);
			obj.position.y -= box.min.y - center.y;

			const material = new THREE.MeshLambertMaterial({
				color: 0xcccccc,
				side: THREE.DoubleSide,
			});
			obj.traverse((child) => {
				if ((child as THREE.Mesh).isMesh) {
					(child as THREE.Mesh).material = material;
					child.castShadow = true;
					child.receiveShadow = true;
				}
			});

			scene.add(obj);

			const maxDim = Math.max(size.x, size.y, size.z);
			camera.position.set(maxDim * 0.8, maxDim * 0.6, maxDim * 0.8);
			controls.target.set(0, size.y * 0.3, 0);
			controls.update();

			sun.shadow.camera.left = -maxDim;
			sun.shadow.camera.right = maxDim;
			sun.shadow.camera.top = maxDim;
			sun.shadow.camera.bottom = -maxDim;
			sun.shadow.camera.updateProjectionMatrix();
			sun.target.position.set(0, size.y * 0.3, 0);
			scene.add(sun.target);
		});

		function animate() {
			requestAnimationFrame(animate);
			controls.update();
			renderer.render(scene, camera);
		}
		animate();

		const resizeObserver = new ResizeObserver(() => {
			const w = container.clientWidth;
			const h = container.clientHeight;
			camera.aspect = w / h;
			camera.updateProjectionMatrix();
			renderer.setSize(w, h);
		});
		resizeObserver.observe(container);

		return () => {
			resizeObserver.disconnect();
			renderer.dispose();
		};
	});
</script>

<section class="mt-20">
	<div class="mb-4 flex flex-col items-end gap-12 justify-end lg:flex-row">
		<p class="text-black">
			3D-Gebäudemodell aus Kinsau &mdash; exportiert mit <a href="/blog/qq" class="underline">qq</a>.
		</p>
		<h2
			class="-mb-[0.5em] block w-max text-[4rem] font-bold leading-none tracking-tight text-white-600 max-lg:-mt-10 max-md:pr-10 sm:text-[6rem] md:-mb-[0.14em] md:text-[10rem] lg:-ml-[8px] xl:-ml-[7px] 2xl:-ml-[12px] 2xl:text-[16rem]"
		>
			Geodaten
		</h2>
	</div>
	<div class="relative h-[500px] w-full overflow-hidden sm:rounded-xl bg-white-600">
		<canvas bind:this={canvas} class="h-full w-full block"></canvas>
	</div>
</section>
