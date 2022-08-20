<script lang="ts">
  import { t } from "../i18n";
</script>

<footer class="container">
  <div class="legal overlay">
    <a sveltekit:prefetch href="/legal">{$t("common.legal")}</a>
    <div class="copy"><i class="fa-regular fa-copyright"></i><span><span>4</span>/<span>5</span>/<span>21</span></span></div>
  </div>

  <div class="links overlay">
    <span class="overflow_hand">
      <span class="looking_for_cv">{@html $t("common.looking_for_cv")}</span>
      <span class="hand_arrow">
        <svg width="9" height="17" viewBox="0 0 9 17" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M1.59524 0.236142C2.42858 0.747671 3.30952 1.28245 3.92857 2.00324C6.28572 4.7469 7 8.00209 6.5 11.513C6.33333 12.7221 5.92857 13.8847 5.59524 15.1635C6.7381 14.6055 7.85714 14.2799 9 15.117C7.5 15.303 6.26191 16.0238 5.04762 16.7446C4.90477 16.8143 4.78572 16.8841 4.64286 16.9306C4.04762 17.1166 3.66667 16.9306 3.5 16.3493C3.42858 16.1401 3.42857 15.9075 3.40476 15.6983C3.30952 13.9312 3.07143 12.2106 2.42858 10.5365C2.38096 10.397 2.45238 10.1645 2.47619 9.74594C3.7619 11.234 3.7619 12.9314 4.2619 14.559C5.61905 11.8153 5.92857 9.0949 5.09524 6.28149C4.30952 3.53783 3.07143 1.1662 0 0.236142C0.761905 -0.0428741 1 -0.112628 1.59524 0.236142Z" fill="var(--c-black)"/>
        </svg>
      </span>
      <a href="https://www.linkedin.com/in/vrogg/"><i class="fa-brands fa-linkedin"></i></a>
    </span>
    <a href="https://github.com/v-rogg"><i class="fa-brands fa-github"></i></a>
  </div>
</footer>

<style>
  footer {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    font-size: 16px;
    padding: 120px 0 2rem;
  }

  @media (max-width: 575.98px) {
    footer {
      flex-direction: column-reverse;
      align-items: center;
      gap: 1rem;
    }

    .links {
      font-size: 1.5rem !important;
    }

    .looking_for_cv {
      transform: translate(80px, -24px) rotateZ(-4deg) !important;
    }

    .hand_arrow {
      transform: translate(0, 7px) !important;
    }

    .legal {
      display: flex;
      gap: 16px;
      align-items: center;
      flex-direction: column;
    }
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
    font-size: 1em;
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
