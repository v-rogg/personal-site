<script lang="ts">
  import { onMount } from 'svelte';
  import SignaturePad from 'signature_pad';

  let canvas: HTMLCanvasElement;
  let signaturePad: SignaturePad;

  function resizeCanvas() {
    const d = signaturePad.toData();

    const ratio =  Math.max(window.devicePixelRatio || 1, 1);

    canvas.width = canvas.offsetWidth * ratio;
    canvas.height = canvas.offsetHeight * ratio;
    canvas.getContext("2d").scale(ratio, ratio);
    signaturePad.clear();
    signaturePad.fromData(d);
  }

  // function clearCanvas() {
  //   console.log(signaturePad.toData());
  //   console.log(signaturePad.toDataURL("image/svg+xml"));
  //   signaturePad.clear();
  // }

  onMount(() => {
    canvas = <HTMLCanvasElement>document.getElementById("signature");
    signaturePad = new SignaturePad(canvas, {
      penColor: "#0f1418",
      velocityFilterWeight: 0.3,
      minDistance: 2,
      throttle: 8
    });
    signaturePad.on();

    window.onresize = resizeCanvas
    resizeCanvas();
  })
</script>

<canvas id="signature"></canvas>
<!--<button on:click={() => {clearCanvas()}} class="overlay">-->
<!--  <i class="fa-solid fa-trash"></i>-->
<!--</button>-->

<style>
  #signature {
    width: 100%;
    height: 100%;
    z-index: 100;
    position: absolute;
    top: 0;
    left: 0;
  }

  /*button {*/
  /*  background: var(--c-white);*/
  /*  border: none;*/
  /*  width: 2rem;*/
  /*  height: 2rem;*/
  /*  border-radius: 2px;*/
  /*}*/

  /*button:hover {*/
  /*  cursor: pointer;*/
  /*}*/
</style>
