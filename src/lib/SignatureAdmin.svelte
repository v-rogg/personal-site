<script lang="ts">
  function approve() {

  }

  function decline() {

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
    opacity: .3;
  }

  button:hover {
    opacity: 1;
    cursor: pointer;
  }

  #approve {
    background: var(--c-green)
  }

  #decline {
    background: var(--c-red)
  }
</style>
