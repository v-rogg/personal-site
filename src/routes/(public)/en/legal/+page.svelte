<script lang="ts">
  import { t } from "$lib/_i18n";
</script>

<svelte:head>
  <title>{$t("common.title")} · {$t("legal.title")}</title>
</svelte:head>

<section class="container top-padding">
<!--  <div>-->
<!--    <h2>-->
<!--      {@html $t("legal.imprint.title")}-->
<!--    </h2>-->
<!--    {@html $t("legal.imprint.links")}-->
<!--    {@html $t("legal.imprint.copyright")}-->
<!--  </div>-->

  <div>
    <h2>
      {$t("legal.privacy.title")}
    </h2>
    {$t("legal.privacy.text")}
  </div>
</section>
