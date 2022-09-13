<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import SignaturePad from "signature_pad";
  import { currentSignatureStore, dark_mode, refIndexStore, signatureRefsStore } from "../stores";
  import { t } from "$lib/_i18n";
  import xss from "xss";
  import type { signatureData } from "$lib/types";

  let canvas: HTMLCanvasElement;

  let pad: HTMLDivElement;
  let signaturePad: SignaturePad;

  let currentSignature: signatureData;

  let empty = true;

  let drawModeActive = false;

  function clearCanvas() {
    signaturePad.clear();
    empty = signaturePad.isEmpty();
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

  function uncenterSignature(data) {
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

  async function loadDelta(delta) {
    drawModeActive = false;
    await fetch(`${$page.url.origin}/_api/signature?ref=${$signatureRefsStore[$refIndexStore + delta]["@ref"].id}`, {
      method: "GET",
    })
      .then(res => res.json())
      .then(json => {
        $refIndexStore += delta;
        $currentSignatureStore = json;
        return json;
      })
  }

  function resizeCanvas() {
    canvas.width = pad.offsetWidth;
    canvas.height = pad.offsetHeight;

    clearCanvas();

    const currentSignature = uncenterSignature((<signatureData>$currentSignatureStore).data.signature);
    signaturePad.fromData(currentSignature);
  }

  async function saveCanvas() {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    let name = xss(prompt($t("signature.prompt"), ""));

    const json = JSON.stringify({
      name,
      signature: centerSignature(signaturePad.toData())
    });

    const newSignature = await fetch(`${$page.url.origin}/_api/signature`, {
      method: "POST",
      body: json
    }).then(res => res.json())
      .then(json => {
        return json[0]
      });

    let signatureRefs = $signatureRefsStore
    signatureRefs.push(newSignature.ref)
    $signatureRefsStore = signatureRefs;
    $currentSignatureStore = newSignature;
    $refIndexStore = signatureRefs.length - 1;

    drawModeActive = false;
    signaturePad.off();
    // clearCanvas();
  }

  function newCanvas() {
    drawModeActive = true;

    $currentSignatureStore = {
      ref: null,
      ts: null,
      data: {
        name: null,
        signature: []
      }
    }

    signaturePad.on();
  }

  onMount(() => {

    // Init
    canvas = <HTMLCanvasElement>document.getElementById("signature");
    signaturePad = new SignaturePad(canvas, {
      penColor: "#0f1418",
      velocityFilterWeight: 0.3,
      minDistance: 2,
      throttle: 8
    });
    signaturePad.off();
    pad = <HTMLDivElement>document.getElementById("pad");

    signaturePad.addEventListener("endStroke", () => {
      currentSignature = $currentSignatureStore;
      currentSignature.data.signature = centerSignature(signaturePad.toData());
      $currentSignatureStore = currentSignature;
      empty = signaturePad.isEmpty();
    });

    window.onresize = resizeCanvas;
    resizeCanvas();

    currentSignatureStore.subscribe((data: signatureData) => {

      if (data != undefined) {
        console.log(data);
        if (Object.keys(data).length > 0) {
          signaturePad.fromData(uncenterSignature(data.data.signature))
          // signaturePad.fromData(uncenterSignature(currentSignature.data.signature));
          // centeredData = currentSignature;
        }
      }

    });
  });
</script>

<div id="pad">
  <canvas id="signature" class:dark={$dark_mode}></canvas>
  <div class="container overlay">
    {#if !drawModeActive && $signatureRefsStore}
      {#if $signatureRefsStore.length - $refIndexStore - 1 > 0}
        <button id="next" on:click={() => loadDelta(1)}>
          <i class="fa-solid fa-angle-right"></i>
        </button>
      {/if}
      {#if $refIndexStore > 0}
        <button id="prev" on:click={() => loadDelta(-1)}>
          <i class="fa-solid fa-angle-left"></i>
        </button>
      {/if}
    {/if}

    {#if drawModeActive}
      <button id="save" on:click={() => saveCanvas()}>
        <i class="fa-solid fa-floppy-disk"></i>
      </button>
      <button id="clear" on:click={() => clearCanvas()}>
        <i class="fa-duotone fa-trash"></i>
      </button>
    {:else}
      <button id="new" on:click={() => newCanvas()}>
        <i class="fa-solid fa-pen"></i>
      </button>
    {/if}
  </div>
</div>

<style>
  #signature {
    z-index: 100;
    position: absolute;
    padding: 0;
    margin: 0;
  }

  #pad {
    width: 100%;
    height: 100%;
    position: absolute;
  }

  .overlay {
    position: relative;
    height: 100%;
  }

  .overlay button {
    background: var(--c-bg);
    border: none;
    font-size: 1rem;
    z-index: 250;
    width: 3rem;
    height: 3rem;
    border-radius: 100%;
    transition: opacity 500ms ease-in-out;
    position: absolute;
  }

  .dark {
    filter: brightness(100) brightness(0.87);
  }

  #next, #prev {
    width: 42px;
    height: 42px;
    top: 50%;
    transform: translateY(-50%);
  }

  #next {
    right: 0;
  }

  #prev {
    left: 0;
  }

  #new, #save {
    right: 0;
    bottom: 2.5rem;
    width: 64px;
    height: 64px;
    font-size: 20px;
  }

  #clear {
    right: 42px;
    bottom: calc(1.5rem + 2px);
    background: var(--c-secondary);
    width: 38px;
    height: 38px;
    font-size: 14px;
  }

  :global(#clear .fa-secondary) {
    opacity: 1 !important;
  }

  :global(#clear .fa-primary) {
    transform: translateY(-5%);
    transition: transform 200ms;
    transform-origin: right;
  }

  :global(#clear:hover .fa-primary) {
    transform: translateY(-10%) rotate(12deg);
  }

  /*.overlay {*/
  /*  position: absolute;*/
  /*  right: 0;*/
  /*  bottom: 2rem;*/
  /*}*/

  button:hover {
    cursor: pointer;
  }
</style>
