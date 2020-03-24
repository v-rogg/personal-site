<script>

    import * as THREE from 'three';

    import Stats from "stats.js";

    import {OrbitControls} from 'three/examples/jsm/controls/OrbitControls';


    import {onMount} from "svelte";

    onMount(() => {

        // var container, stats;
        //
        // var camera, scene, renderer;
        //
        // init();
        // animate();
        //
        // function init() {
        //
        //     container = document.createElement( 'div' );
        //     document.body.appendChild( container );
        //
        //     camera = new THREE.PerspectiveCamera( 60, window.innerWidth / window.innerHeight, 0.1, 10 );
        //     camera.position.set( 0, 0, 1 );
        //
        //     scene = new THREE.Scene();
        //
        //     var colorArray = [ new THREE.Color( 0xff0080 ), new THREE.Color( 0xffffff ), new THREE.Color( 0x8000ff ) ];
        //     var positions = [];
        //     var colors = [];
        //
        //     for ( var i = 0; i < 100; i ++ ) {
        //
        //         positions.push( Math.random() - 0.5, Math.random() - 0.5, Math.random() - 0.5 );
        //
        //         var clr = colorArray[ Math.floor( Math.random() * colorArray.length ) ];
        //
        //         colors.push( clr.r, clr.g, clr.b );
        //
        //     }
        //
        //     var geometry = new THREE.BufferGeometry();
        //     geometry.setAttribute( 'position', new THREE.Float32BufferAttribute( positions, 3 ) );
        //     geometry.setAttribute( 'color', new THREE.Float32BufferAttribute( colors, 3 ) );
        //
        //     var material = new THREE.PointsMaterial( { size: 4, vertexColors: THREE.VertexColors, depthTest: false, sizeAttenuation: false } );
        //
        //     var mesh = new THREE.Points( geometry, material );
        //     scene.add( mesh );
        //
        //     renderer = new THREE.WebGLRenderer( { preserveDrawingBuffer: true } );
        //     renderer.setPixelRatio( window.devicePixelRatio );
        //     renderer.setSize( 1200, 400 );
        //     renderer.autoClearColor = false;
        //     document.getElementById("view").appendChild( renderer.domElement );
        //
        //     // stats = new Stats();
        //     // document.getElementById("view").appendChild( stats.dom);
        //
        //     //
        //
        //     window.addEventListener( 'resize', onWindowResize, false );
        //
        // }
        //
        // function onWindowResize() {
        //
        //     camera.aspect = window.innerWidth / window.innerHeight;
        //     camera.updateProjectionMatrix();
        //
        //     renderer.setSize( window.innerWidth, window.innerHeight );
        //
        // }
        //
        // //
        //
        // function animate() {
        //
        //     requestAnimationFrame( animate );
        //
        //     render();
        //     // stats.update();
        //
        // }
        //
        // function render() {
        //
        //     scene.rotation.y = Date.now() / 2000;
        //
        //     renderer.render( scene, camera );
        //
        // }

        var camera, scene, renderer, stats;
        var pointLight, pointLight2;

        init();
        animate();

        function init() {

            camera = new THREE.PerspectiveCamera( 20, 1200 / 400, 1, 1000 );
            camera.position.set( 0, 10, 40 );

            scene = new THREE.Scene();
            scene.add( new THREE.AmbientLight( 0x111122 ) );

            // lights

            function createLight( color ) {

                var intensity = 1.5;

                var pointLight = new THREE.PointLight( color, intensity, 20 );
                pointLight.castShadow = true;
                pointLight.shadow.camera.near = 1;
                pointLight.shadow.camera.far = 60;
                pointLight.shadow.bias = - 0.005; // reduces self-shadowing on double-sided objects

                var geometry = new THREE.SphereBufferGeometry( 0.3, 12, 6 );
                var material = new THREE.MeshBasicMaterial( { color: color } );
                material.color.multiplyScalar( intensity );
                var sphere = new THREE.Mesh( geometry, material );
                pointLight.add( sphere );

                var texture = new THREE.CanvasTexture( generateTexture() );
                texture.magFilter = THREE.NearestFilter;
                texture.wrapT = THREE.RepeatWrapping;
                texture.wrapS = THREE.RepeatWrapping;
                texture.repeat.set( 1, 4.5 );

                var geometry = new THREE.SphereBufferGeometry( 2, 32, 8 );
                var material = new THREE.MeshPhongMaterial( {
                    side: THREE.DoubleSide,
                    alphaMap: texture,
                    alphaTest: 0.5
                } );

                var sphere = new THREE.Mesh( geometry, material );
                sphere.castShadow = true;
                sphere.receiveShadow = true;
                pointLight.add( sphere );

                // custom distance material
                var distanceMaterial = new THREE.MeshDistanceMaterial( {
                    alphaMap: material.alphaMap,
                    alphaTest: material.alphaTest
                } );
                sphere.customDistanceMaterial = distanceMaterial;

                return pointLight;

            }

            pointLight = createLight( 0xff6978 );
            scene.add( pointLight );

            pointLight2 = createLight( 0x340068 );
            scene.add( pointLight2 );
            //

            var geometry = new THREE.BoxBufferGeometry( 30, 30, 30 );

            var material = new THREE.MeshPhongMaterial( {
                color: 0xa0adaf,
                shininess: 10,
                specular: 0x111111,
                side: THREE.BackSide
            } );

            var mesh = new THREE.Mesh( geometry, material );
            mesh.position.y = 10;
            mesh.receiveShadow = true;
            scene.add( mesh );

            //

            renderer = new THREE.WebGLRenderer( { antialias: true } );
            renderer.setPixelRatio( window.devicePixelRatio );
            renderer.setSize( 1200, 400 );
            renderer.shadowMap.enabled = true;
            renderer.shadowMap.type = THREE.BasicShadowMap;
            // document.body.appendChild( renderer.domElement );
            document.getElementById("view").appendChild( renderer.domElement );

            var controls = new OrbitControls( camera, renderer.domElement );
            controls.target.set( 0, 10, 0 );
            controls.update();

            stats = new Stats();
            // document.body.appendChild( stats.dom );

            //

            window.addEventListener( 'resize', onWindowResize, false );

        }

        function onWindowResize() {

            camera.aspect = 1200 / 400;
            camera.updateProjectionMatrix();

            renderer.setSize( 1200, 400 );

        }

        function generateTexture() {

            var canvas = document.createElement( 'canvas' );
            canvas.width = 2;
            canvas.height = 2;

            var context = canvas.getContext( '2d' );
            context.fillStyle = 'white';
            context.fillRect( 0, 1, 2, 1 );

            return canvas;

        }

        function animate() {

            requestAnimationFrame( animate );
            render();

        }

        function render() {

            var time = performance.now() * 0.001;

            pointLight.position.x = Math.sin( time * 0.6 ) * 9;
            pointLight.position.y = Math.sin( time * 0.7 ) * 9 + 6;
            pointLight.position.z = Math.sin( time * 0.8 ) * 9;

            pointLight.rotation.x = time;
            pointLight.rotation.z = time;

            time += 10000;

            pointLight2.position.x = Math.sin( time * 0.6 ) * 9;
            pointLight2.position.y = Math.sin( time * 0.7 ) * 9 + 6;
            pointLight2.position.z = Math.sin( time * 0.8 ) * 9;

            pointLight2.rotation.x = time;
            pointLight2.rotation.z = time;

            renderer.render( scene, camera );

            stats.update();

        }
    });

</script>

<style>
    .renderBox {
        /*width: 1200px;*/
        height: 400px;
        overflow: hidden;
        position: relative;
    }

    .render {
        width: 1200px;
        height: 400px;
        position: absolute;
        top: -9999px;
        bottom: -9999px;
        left: -9999px;
        right: -9999px;
        margin: auto;
    }
</style>

<div class="container renderBox">
    <div id="view" class="render">

    </div>
</div>