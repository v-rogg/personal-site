<script>
    export let lang;
</script>

<style>
    h2 {
        text-transform: uppercase;
        margin-top: .5rem;
        margin-bottom: 0;
        font-weight: 400;
        line-height: 1.5rem;
    }

    b {
        font-weight: 700;
    }

    .flex {
        display: flex;
        justify-content: space-between;
        margin-bottom: 2rem;
    }

    .flex-column {
        display: flex;
        flex-direction: column;
        justify-content: end;
    }

    .info {
        text-align: right;
        margin-top: 0;
        margin-bottom: 0;
        font-weight: 400;
        font-size: 1.2rem;
        color: black;
        text-decoration: none;
    }
</style>

<div class="flex">
  {#if lang === "de" }
      <h2><b>Entwickler</b><br>mit Auge<br>für Design,<br><b>Macher</b></h2>
  {/if}
  {#if lang === "en"}
      <h2><b>Developer</b><br>with an eye<br>for Design,<br><b>Entrepreneur</b></h2>
  {/if}
    <div class="flex-column">
        <a class="info" href="https://linkedin.com/in/vrogg">linkedin.com/in/vrogg</a>
        <a class="info" href="tel:+49 (175) 7090991">+49 (175) 70 90 991</a>
        <a class="info" href="mailto:valentin.rogg@hs-augsburg.de">valentin.rogg@hs-augsburg.de</a>
    </div>
</div>