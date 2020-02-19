<script>

    import * as THREE from 'three';

    // import Stats from "stats.js";


    import {onMount} from "svelte";

    onMount(() => {

        var container, stats;

        var camera, scene, renderer;

        init();
        animate();

        function init() {

            container = document.createElement( 'div' );
            document.body.appendChild( container );

            camera = new THREE.PerspectiveCamera( 60, window.innerWidth / window.innerHeight, 0.1, 10 );
            camera.position.set( 0, 0, 1 );

            scene = new THREE.Scene();

            var colorArray = [ new THREE.Color( 0xff0080 ), new THREE.Color( 0xffffff ), new THREE.Color( 0x8000ff ) ];
            var positions = [];
            var colors = [];

            for ( var i = 0; i < 100; i ++ ) {

                positions.push( Math.random() - 0.5, Math.random() - 0.5, Math.random() - 0.5 );

                var clr = colorArray[ Math.floor( Math.random() * colorArray.length ) ];

                colors.push( clr.r, clr.g, clr.b );

            }

            var geometry = new THREE.BufferGeometry();
            geometry.setAttribute( 'position', new THREE.Float32BufferAttribute( positions, 3 ) );
            geometry.setAttribute( 'color', new THREE.Float32BufferAttribute( colors, 3 ) );

            var material = new THREE.PointsMaterial( { size: 4, vertexColors: THREE.VertexColors, depthTest: false, sizeAttenuation: false } );

            var mesh = new THREE.Points( geometry, material );
            scene.add( mesh );

            renderer = new THREE.WebGLRenderer( { preserveDrawingBuffer: true } );
            renderer.setPixelRatio( window.devicePixelRatio );
            renderer.setSize( 1200, 400 );
            renderer.autoClearColor = false;
            document.getElementById("view").appendChild( renderer.domElement );

            // stats = new Stats();
            // document.getElementById("view").appendChild( stats.dom);

            //

            window.addEventListener( 'resize', onWindowResize, false );

        }

        function onWindowResize() {

            camera.aspect = window.innerWidth / window.innerHeight;
            camera.updateProjectionMatrix();

            renderer.setSize( window.innerWidth, window.innerHeight );

        }

        //

        function animate() {

            requestAnimationFrame( animate );

            render();
            // stats.update();

        }

        function render() {

            scene.rotation.y = Date.now() / 2000;

            renderer.render( scene, camera );

        }
    });

</script>

<style>
    .renderBox {
        width: 1200px;
        height: 400px;
    }
</style>

<div id="view" class="container renderBox">

</div>