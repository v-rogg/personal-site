<script lang="ts">
  import { onMount } from 'svelte';
  import SignaturePad from 'signature_pad';

  let canvas: HTMLCanvasElement;
  let signaturePad: SignaturePad;

  let empty = true;

  function resizeCanvas() {
    const d = signaturePad.toData();

    const ratio =  Math.max(window.devicePixelRatio || 1, 1);

    canvas.width = canvas.offsetWidth * ratio;
    canvas.height = canvas.offsetHeight * ratio;
    canvas.getContext("2d").scale(ratio, ratio);
    signaturePad.clear();
    signaturePad.fromData(d);
  }

  function clearCanvas() {
    console.log(signaturePad.toData());
    console.log(signaturePad.toDataURL("image/svg+xml"));
    signaturePad.clear();
    empty = signaturePad.isEmpty();
  }

  onMount(() => {
    canvas = <HTMLCanvasElement>document.getElementById("signature");
    signaturePad = new SignaturePad(canvas, {
      penColor: "#0f1418",
      velocityFilterWeight: 0.3,
      minDistance: 2,
      throttle: 8
    });

    signaturePad.addEventListener("endStroke", () => {
      empty = signaturePad.isEmpty();
    })

    window.onresize = resizeCanvas
    resizeCanvas();
  })
</script>

<canvas id="signature"></canvas>
<div class="container">
  <button on:click={() => {clearCanvas()}} class="overlay" style="opacity: {empty ? 0 : 1}">
    <i class="fa-solid fa-trash"></i>
  </button>
</div>

<style>
  #signature {
    width: 100%;
    height: 100%;
    z-index: 100;
    position: absolute;
    top: 0;
    left: 0;
  }

  div {
    width: 100%;
  }

  button {
    background: var(--c-white);
    border: none;
    font-size: 1rem;
    position: relative !important;
    z-index: 250;
    left: calc(100% - 3rem);
    box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
    width: 3rem;
    height: 3rem;
    border-radius: 100%;
    opacity: 0;
    transition: opacity 500ms ease-in-out;
  }

  @media (max-width: 575.98px) {
    button {
      left: calc(100% - 8rem);
      position: absolute !important;
      width: 4rem;
      height: 4rem;
      font-size: 1.25rem;
      bottom: 6rem;
    }
  }

  button:hover {
    cursor: pointer;
  }
</style>
