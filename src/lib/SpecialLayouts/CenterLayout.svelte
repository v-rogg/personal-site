<svelte:head>
  <title>Wassererbe Augsburg</title>
</svelte:head>

<main class="container">
  <div class="app">
    <img class="ratio" src="sprinter.png" alt="resize" aria-hidden="true" />
    {#if $$slots.center}
      <div class="center abs">
        <slot name="center" />
      </div>
    {/if}
    {#if $$slots.top}
      <div class="top abs">
        <slot name="top" />
      </div>
    {/if}
    {#if $$slots.bottom}
      <div class="bottom abs">
        <slot name="bottom" />
      </div>
    {/if}
  </div>
</main>

<style>
  .container {
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    position: absolute;
    background: var(--c-white);
  }

  .app {
    max-width: 1920px;
    max-height: 1400px;
    position: relative;
    top: -17px;
  }

  .ratio {
    display: block;
    max-height: calc(100vh - 100px - 70px - 4rem);
    margin: auto;
    height: max-content;
    width: auto;
    opacity: 0;
    pointer-events: none;
  }

  .abs {
    position: absolute;
    display: flex;
    align-items: center;
  }

  .top {
    top: -70px;
    height: 60px;
  }

  .top,
  .bottom {
    left: 10%;
    width: 80%;
  }

  .bottom {
    bottom: -100px;
    height: 70px;
  }

  .center {
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
  }
</style>
