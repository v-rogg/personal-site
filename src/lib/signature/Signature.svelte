<script lang="ts">
  import { onMount } from 'svelte';
  import SignaturePad from 'signature_pad';
  // import trimCanvas from 'trim-canvas';

  let canvas: HTMLCanvasElement;
  let pad: HTMLDivElement;
  let signaturePad: SignaturePad;

  let oldWidth, oldHeight;
  let centeredData;

  let empty = true;

  function resizeCanvas() {
    // const d = signaturePad.toData();

    const ratio =  Math.max(window.devicePixelRatio || 1, 1);

    const pad = document.getElementById('pad');
    canvas.width = pad.offsetWidth * ratio;
    canvas.height = pad.offsetHeight * ratio;

    canvas.getContext("2d").scale(ratio, ratio);
    signaturePad.clear();

    oldWidth = pad.offsetWidth;

    const decenteredData = decenterSignature(centeredData);
    signaturePad.fromData(decenteredData);
    centeredData = centerSignature(signaturePad.toData());
  }

  function cloneCanvas(oldCanvas): HTMLCanvasElement {

    //create a new canvas
    let newCanvas = document.createElement('canvas');
    let context = newCanvas.getContext('2d');

    //set dimensions
    newCanvas.width = oldCanvas.width;
    newCanvas.height = oldCanvas.height;

    //apply the old canvas to the new one
    context.drawImage(oldCanvas, 0, 0);

    //return the new canvas
    return newCanvas;
  }

  function logCanvas() {
    const data = signaturePad.toData();
    console.log("old", data);

    console.log("centered", centerSignature(data));
    console.log("decentered", decenterSignature(centerSignature(data)));
  }

  function centerSignature(data) {
    const middleH = pad.offsetWidth / 2;
    const bottomV = pad.offsetHeight;

    let centeredData = [];

    for (let signature of data) {
      let centeredSignature = JSON.parse(JSON.stringify(signature))

      centeredSignature.points.forEach(point => {
        point.x = point.x - middleH;
        point.y = point.y - bottomV;
      })

      centeredData.push(centeredSignature);
    }

    return centeredData;
  }

  function decenterSignature(data) {
    const middleH = pad.offsetWidth / 2;
    const bottomV = pad.offsetHeight;

    let centeredData = [];

    for (let signature of data) {
      let centeredSignature = JSON.parse(JSON.stringify(signature))

      centeredSignature.points.forEach(point => {
        point.x = point.x + middleH;
        point.y = point.y + bottomV;
      })

      centeredData.push(centeredSignature);
    }

    return centeredData;
  }

  function clearCanvas() {
    // console.log(signaturePad.toData());
    // console.log(signaturePad.toDataURL("image/svg+xml"));
    signaturePad.clear();
    empty = signaturePad.isEmpty();
  }

  async function saveCanvas() {
    // const img = signaturePad.toDataURL("image/svg+xml");

    // trimCanvas(canvas);
    let can = cloneCanvas(canvas);
    // trimCanvas(can);
    can.toDataURL("image/png")
    const img = can.toDataURL("image/png");

    console.log(signaturePad.toData());

    // const json = JSON.stringify({
    //   uuid: crypto.randomUUID(),
    //   image: img
    // })

    // console.log(json);

    // await fetch(`${$page.url.origin}/api/storeSignature`, {
    //   method: "POST",
    //   body: json,
    // })

    can = null;
    clearCanvas();
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
      centeredData = centerSignature(signaturePad.toData());
    })

    pad = <HTMLDivElement>document.getElementById("pad");
    oldWidth = pad.offsetWidth;
    oldHeight = pad.offsetHeight;
    centeredData = centerSignature(signaturePad.toData());

    signaturePad.addEventListener("endStroke", () => {
      empty = signaturePad.isEmpty();
    })

    window.onresize = resizeCanvas
    resizeCanvas();
  })
</script>

<div id="pad">
  <canvas id="signature"></canvas>
  <div class="container overlay-container">
    <div class="overlay">
      <button id="log" on:click={() => {logCanvas()}} style="opacity: {empty ? 1 : 1}">
        <i class="fa-solid fa-code-simple"></i>
      </button>
      <button id="clear" on:click={() => {clearCanvas()}} style="opacity: {empty ? 1 : 1}">
        <i class="fa-solid fa-trash"></i>
      </button>
      <button id="save" on:click={() => {saveCanvas()}} style="opacity: {empty ? 1 : 1}">
        <i class="fa-solid fa-floppy-disk"></i>
      </button>
    </div>
  </div>
</div>

<style>
  #signature {
    /*width: 100%;*/
    /*height: 100%;*/
    z-index: 100;
    position: absolute;
    /*top: 2rem;*/
    /*left: 2rem;*/
    padding: 0;
    margin: 0;
  }

  #pad {
    width: 100%;
    height: 100%;
  }

  /*div {*/
  /*  width: 100%;*/
  /*  position: fixed;*/
  /*}*/
  .overlay-container {
    position: relative;
    height: 100%;
  }

  .overlay {
    position: absolute;
    right: 0;
    bottom: 2rem;
  }

  .overlay > button {
    background: var(--c-white);
    border: none;
    font-size: 1rem;
    position: relative !important;
    z-index: 250;
    /*left: calc(100% - 3rem);*/
    right: 0;
    box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);
    width: 3rem;
    height: 3rem;
    border-radius: 100%;
    opacity: 0;
    transition: opacity 500ms ease-in-out;
  }

  @media (max-width: 575.98px) {
    .overlay > button {
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
