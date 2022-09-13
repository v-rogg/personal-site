<script>
  import { t, locale} from "$lib/_i18n";
</script>

<section class="container">
  <h1>Valentin Rogg</h1>

  {@html $t("bio.description").replace("project_placeholder", `<a href='/${$locale}' class="bold">Parley</a>`)}
  <a id="contact" href="https://www.linkedin.com/in/vrogg/" rel="noreferrer nofollow" target="_blank">{$t("bio.contact")}<i class="icon fa-solid fa-comment-smile"></i></a>
</section>

<style>
  section {
    text-align: center;
    padding-bottom: 3rem;
  }

  h1 {
    font-family: "Display", sans-serif;
    font-size: 32px;
    font-weight: 500;
    margin: 0 0 2rem;
  }

  #contact {
    display: block;
    width: max-content;
    margin: 2.5rem auto;
    font-size: 16px;
    border-radius: 4px;
    padding: .5rem 1rem;
    border: none;
    background: var(--c-secondary);
  }

  #contact .icon {
    margin-left: .5rem;
    color: var(--c-primary);
  }

  #contact:hover {
    cursor: pointer;
    background: var(--c-grey-10);
  }

  #contact:active {
    cursor: default;
    background: var(--c-grey-30);
  }
</style>
