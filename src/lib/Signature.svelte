<script lang="ts">
  import { getContext, onMount } from "svelte";
  import { page } from "$app/stores";
  import SignaturePad from "signature_pad";

  const signatureRefs = getContext("signatureRefs");
  let currentSignature = getContext("currentSignature");

  let canvas: HTMLCanvasElement;
  let pad: HTMLDivElement;
  let signaturePad: SignaturePad;

  let oldWidth, oldHeight;
  let centeredData;

  let empty = true;

  function resizeCanvas() {
    // const d = signaturePad.toData();

    const ratio = Math.max(window.devicePixelRatio || 1, 1);

    const pad = document.getElementById("pad");
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
    let newCanvas = document.createElement("canvas");
    let context = newCanvas.getContext("2d");

    //set dimensions
    newCanvas.width = oldCanvas.width;
    newCanvas.height = oldCanvas.height;

    //apply the old canvas to the new one
    context.drawImage(oldCanvas, 0, 0);

    //return the new canvas
    return newCanvas;
  }

  async function logCanvas() {
    const data = signaturePad.toData();
    // console.log("old", data);

    console.log("centered", centerSignature(data));
    console.log("decentered", decenterSignature(centerSignature(data)));
  }

  function centerSignature(data) {
    const middleH = pad.offsetWidth / 2;
    const bottomV = pad.offsetHeight;

    let centeredData = [];

    for (let signature of data) {
      let centeredSignature = JSON.parse(JSON.stringify(signature));

      centeredSignature.points.forEach((point) => {
        point.x = point.x - middleH;
        point.y = point.y - bottomV;
      });

      centeredData.push(centeredSignature);
    }

    return centeredData;
  }

  function decenterSignature(data) {
    const middleH = pad.offsetWidth / 2;
    const bottomV = pad.offsetHeight;

    let centeredData = [];

    for (let signature of data) {
      let centeredSignature = JSON.parse(JSON.stringify(signature));

      centeredSignature.points.forEach((point) => {
        point.x = point.x + middleH;
        point.y = point.y + bottomV;
      });

      centeredData.push(centeredSignature);
    }

    return centeredData;
  }

  function clearCanvas() {
    signaturePad.clear();
    empty = signaturePad.isEmpty();
  }

  async function saveCanvas() {
    // const img = signaturePad.toDataURL("image/svg+xml");

    // trimCanvas(canvas);
    // let can = cloneCanvas(canvas);
    // trimCanvas(can);
    // can.toDataURL("image/png");
    // const img = can.toDataURL("image/png");

    // console.log(signaturePad.toData());

    // const json = JSON.stringify({
    //   uuid: crypto.randomUUID(),
    //   image: img
    // })

    // console.log(json);

    // await fetch(`${$page.url.origin}/api/storeSignature`, {
    //   method: "POST",
    //   body: json,
    // })

    const json = JSON.stringify({
      name: crypto.randomUUID(),
      signature: signaturePad.toData()
    });

    console.log(json);

    await fetch(`${$page.url.origin}/api/signature`, {
      method: "POST",
      body: json
    }).then(res => console.log(res));

    clearCanvas();
  }

  async function loadCanvas() {
    await fetch(`${$page.url.origin}/api/signature`, {
      method: "GET",
    })
      .then(res => res.json())
      .then(json => {
        // console.log(json);
        clearCanvas();
        json.sort((a, b) => {return 0.5 - Math.random()})
        signaturePad.fromData(json[0].data.signature)
      })
  }

  function next() {

  }

  function prev() {

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
    });

    pad = <HTMLDivElement>document.getElementById("pad");
    oldWidth = pad.offsetWidth;
    oldHeight = pad.offsetHeight;
    centeredData = centerSignature(signaturePad.toData());

    signaturePad.addEventListener("endStroke", () => {
      empty = signaturePad.isEmpty();
    });


    window.onresize = resizeCanvas;
    resizeCanvas();

    console.log("refs", signatureRefs);
    console.log("sig", currentSignature);
    signaturePad.fromData(currentSignature.data.signature);

    console.log(signaturePad.toData());
  });
</script>

<div id="pad">
  <canvas id="signature" />
  <div class="container overlay">
      <button id="next" on:click={() => next()}>
        <i class="fa-solid fa-angle-right"></i>
      </button>
      <button id="prev" on:click={() => prev()}>
        <i class="fa-solid fa-angle-left"></i>
      </button>
      <button
        id="log"
        on:click={() => {
          logCanvas();
        }}
        style="opacity: {empty ? 1 : 1}"
      >
        <i class="fa-solid fa-code-simple" />
      </button>
      <button
        id="clear"
        on:click={() => {
          clearCanvas();
        }}
        style="opacity: {empty ? 1 : 1}"
      >
        <i class="fa-solid fa-trash" />
      </button>
      <button
        id="save"
        on:click={() => {
          saveCanvas();
        }}
        style="opacity: {empty ? 1 : 1}"
      >
        <i class="fa-solid fa-floppy-disk" />
      </button>
      <button
        id="load"
        on:click={() => {
          loadCanvas();
        }}
        style="opacity: {empty ? 1 : 1}"
      >
        <i class="fa-solid fa-download" />
      </button>
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
    /*box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);*/
  }

  /*div {*/
  /*  width: 100%;*/
  /*  position: fixed;*/
  /*}*/

  .overlay {
    position: relative;
    height: 100%;
  }

  .overlay button {
    background: var(--c-white);
    border: none;
    font-size: 1rem;
    z-index: 250;
    /*right: 0;*/
    width: 3rem;
    height: 3rem;
    border-radius: 100%;
    /*opacity: 0;*/
    transition: opacity 500ms ease-in-out;
    position: absolute;
  }

  #next, #prev {
    top: 50%;
    transform: translateY(-50%);
  }

  #next {
    right: 0;
  }

  #prev {
    left: 0;
  }

  .overlay-container {
    position: relative;
    height: 100%;
  }

  /*.overlay {*/
  /*  position: absolute;*/
  /*  right: 0;*/
  /*  bottom: 2rem;*/
  /*}*/

  /*.overlay > button {*/
  /*  background: var(--c-white);*/
  /*  border: none;*/
  /*  font-size: 1rem;*/
  /*  position: relative !important;*/
  /*  z-index: 250;*/
  /*  !*left: calc(100% - 3rem);*!*/
  /*  right: 0;*/
  /*  !*box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);*!*/
  /*  width: 3rem;*/
  /*  height: 3rem;*/
  /*  border-radius: 100%;*/
  /*  opacity: 0;*/
  /*  transition: opacity 500ms ease-in-out;*/
  /*  !*box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);*!*/
  /*}*/

  @media (max-width: 575.98px) {
    /*.overlay > button {*/
    /*  left: calc(100% - 8rem);*/
    /*  position: absolute !important;*/
    /*  width: 4rem;*/
    /*  height: 4rem;*/
    /*  font-size: 1.25rem;*/
    /*  bottom: 6rem;*/
    /*}*/
  }

  button:hover {
    cursor: pointer;
  }
</style>
