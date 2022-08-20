<script>
  import { t } from '$lib/i18n';

  const link = 'https://kit.svelte.dev'
</script>

<h1>{$t('common.title')}</h1>
<p>{@html $t('common.text', { link })}</p>
