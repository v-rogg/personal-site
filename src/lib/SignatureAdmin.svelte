<script lang="ts">
  import { page } from "$app/stores";
  import { currentSignatureStore } from "$lib/../stores";

  async function approve() {
    await fetch(`${$page.url.origin}/_api/signature`, {
      method: "PUT",
      body: JSON.stringify({
        ref: $currentSignatureStore.ref["@ref"].id,
        status: 'approved'
      })
    })
      .then(res => res.json())
      .then(json => {
        return json;
      })
  }

  async function decline() {
    await fetch(`${$page.url.origin}/_api/signature`, {
      method: "PUT",
      body: JSON.stringify({
        ref: $currentSignatureStore.ref["@ref"].id,
        status: 'declined'
      })
    })
      .then(res => res.json())
      .then(json => {
        return json;
      })
  }
</script>

<div class="container">
  <div class="admin-panel">
    <button id="approve" on:click={() => approve()}>
      <i class="fa-regular fa-check"></i>
    </button>
    <button id="decline" on:click={() => decline()}>
      <i class="fa-regular fa-xmark"></i>
    </button>
  </div>
</div>

<style>
  .admin-panel {
    position: absolute;
    bottom: 2rem;
    z-index: 800;
    left: 50%;
    transform: translateX(-50%);
    display: flex;
    gap: 2rem;
  }

  button {
    border: none;
    font-size: 2rem;
    z-index: 250;
    min-width: 5rem;
    min-height: 5rem;
    border-radius: 100%;
    position: relative;
    /*position: absolute;*/
    color: var(--c-white);
    background: none;
    opacity: .8;
  }

  button:hover {
    opacity: 1;
    cursor: pointer;
  }

  #approve {
    background: var(--c-green);
    /*color: var(--c-green)*/
  }

  #decline {
    background: var(--c-red);
    /*color: var(--c-red)*/
  }
</style>
