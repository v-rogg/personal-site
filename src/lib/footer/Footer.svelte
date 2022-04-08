<script lang="ts">
  import { t } from "../i18n";
</script>

<footer class="container">
  <div class="legal">
    <a sveltekit:prefetch href="/legal">{$t("common.legal")}</a>
    <div class="copy"><i class="fa-regular fa-copyright" /><span><span>4</span>/<span>5</span>/<span>21</span></span></div>
  </div>

  <div class="links">
    <span class="overflow_hand">
      <span class="looking_for_cv">{@html $t("common.looking_for_cv")}</span>
      <span class="hand_arrow">
        <img src="arrow-down.svg">
      </span>
      <a href="https://www.linkedin.com/in/vrogg/"><i class="fa-brands fa-linkedin" /></a>
    </span>
    <a href="https://github.com/v-rogg"><i class="fa-brands fa-github" /></a>
  </div>
</footer>

<style>
  footer {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    font-size: 16px;
  }

  .links {
    font-size: 18px;
    display: flex;
    gap: 16px;
  }

  .legal {
    display: flex;
    gap: 16px;
    align-items: center;
  }

  .overflow_hand {
    position: relative;
  }

  .looking_for_cv {
    font-family: "Caveat", sans-serif;
    position: absolute;
    width: max-content;
    bottom: 100%;
    right: 0;
    transform: translate(24px, -24px) rotateZ(-4deg);
    transform-origin: right;
    text-align: center;
    line-height: 1;
    font-size: 18px;
  }

  .hand_arrow {
    position: absolute;
    bottom: 100%;
    right: 0;
    transform: translate(0, 4px);
  }

  .copy {
    display: flex;
    gap: 13px;
    align-items: center;
  }

  .copy span {
    font-size: 14px;
  }

  .copy span span {
    margin: 0 3px;
  }
</style>
