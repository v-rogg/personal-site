<script>
  import {createEventDispatcher} from "svelte";

  const dispatch = createEventDispatcher();

  function setEN() {
      dispatch('language', {
          lang: "en"
      });
  }

  function setDE() {
      dispatch('language', {
          lang: "de"
      });
  }
</script>

<style>
    h1 {
        text-transform: uppercase;
        font-size: 2rem;
        margin-top: 0;
        margin-bottom: 0;
        font-weight: 700;
    }

    button {
        border: none;
        background: none;
        margin: 0;
        cursor: pointer;
        font-weight: 700;
    }

    .lang {
        display: flex;
        justify-content: space-between;
    }
</style>

<div class="lang">
    <h1>Valentin Rogg</h1>
    <div>
        <button on:click={setDE} >de</button>
        /
        <button on:click={setEN} >en</button>
    </div>
</div>
