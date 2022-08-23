<script>
  import { t, locale} from "$lib/_i18n";
</script>

<section class="container">
  <h1>Valentin Rogg</h1>

  {@html $t("bio.description").replace("project_placeholder", `<a href='/${$locale}' class="bold">Parley</a>`)}
  <button id="contact">{$t("bio.contact")}<i class="icon fa-solid fa-comment-smile"></i></button>
</section>

<style>
  section {
    text-align: center;
  }

  h1 {
    font-family: "Display", sans-serif;
    font-size: 32px;
    font-weight: 500;
    margin: 0 0 2rem;
  }

  #contact {
    /*font-weight: 600;*/
    /*height: 24px;*/
    margin-top: 1rem;
    font-size: 16px;
    border-radius: 4px;
    padding: .5rem 1rem;
    border: none;
    background: var(--c-off-white);
    /*color: var(--c-off-white);*/
    /*box-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);*/
    transition: 25ms;
  }

  #contact .icon {
    margin-left: .5rem;
    color: var(--c-primary);
  }

  #contact:hover {
    cursor: pointer;
    background: var(--c-grey-10);
    /*box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.08);*/
  }

  #contact:active {
    cursor: default;
    background: var(--c-grey-30);
  }
</style>
