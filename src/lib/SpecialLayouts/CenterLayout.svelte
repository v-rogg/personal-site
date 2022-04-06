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
    {#if $$slots.center_large}
      <div class="center-large abs">
        <slot name="center_large" />
      </div>
    {/if}
    {#if $$slots.top_left}
      <div class="top left abs">
        <slot name="top_left" />
      </div>
    {/if}
    {#if $$slots.top_mid}
      <div class="top h-mid abs">
        <slot name="top_mid" />
      </div>
    {/if}
    {#if $$slots.mid_left}
      <div class="v-mid left abs">
        <slot name="mid_left" />
      </div>
    {/if}
    {#if $$slots.mid_right}
      <div class="v-mid right abs">
        <slot name="mid_right" />
      </div>
    {/if}
    {#if $$slots.bottom_mid}
      <div class="bottom h-mid abs">
        <slot name="bottom_mid" />
      </div>
    {/if}
    {#if $$slots.bottom_left}
      <div class="bottom left abs">
        <slot name="bottom_left" />
      </div>
    {/if}
    {#if $$slots.bottom_right}
      <div class="bottom right abs">
        <slot name="bottom_right" />
      </div>
    {/if}
  </div>
</main>

<style>
  :root {
    --side-width: 300px;
  }

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
    max-width: calc(100vw - 8rem - (var(--side-width) * 2));
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

  .left {
    left: var(--side-width);
    width: var(--side-width);
  }

  .right {
    right: var(--side-width);
    width: var(--side-width);
  }

  .top {
    top: -70px;
    height: 60px;
  }

  .v-mid {
    top: 0;
    height: 100%;
  }

  .h-mid {
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

  .center-large {
    top: -70px;
    bottom: -100px;
    width: 100%;
    height: calc(100% + 70px + 100px);
  }
</style>
