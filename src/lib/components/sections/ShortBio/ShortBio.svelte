<script lang="ts">
	import { t, locale } from "$lib/_i18n";
</script>

<section>
	<h1 class="text-4xl text-center font-semibold mb-4 font-sans">Valentin Rogg</h1>
	<p class="text-center my-4">
		🇦🇹🇧🇪🇨🇦🇨🇿🇩🇰🇪🇬🇪🇪🇫🇴🇫🇮🇫🇷🇩🇪🇬🇱🇮🇸🇮🇪🇮🇹🇱🇻🇲🇨🇳🇱🇳🇴🇵🇱🇵🇹🇷🇺🇪🇸🇸🇪🇨🇭🇹🇷🏴󠁧󠁢󠁥󠁮󠁧󠁿🏴󠁧󠁢󠁳󠁣󠁴󠁿🏴󠁧󠁢󠁷󠁬󠁳󠁿🇺🇸<br/>
<!--		<span class="font-sans">30/193</span>-->
	</p>
	<p class="text-center font-serif text-md-lg">
	<!-- eslint-disable-next-line svelte/no-at-html-tags -->
		{@html $t("bio.description").replace(
			"project_placeholder",
			`<a href='/${$locale}' class="bold">Parley</a>`
		)}
	</p>
<!--	<p class="text-center my-4 flex flex-col items-start justify-center max-w-max mx-auto">-->
<!--		<a href="mailto:{$t('bio.mail')}"><i class="fa-regular fa-envelope mr-2"></i><span class="text-skin-dark">{$t("bio.mail")}</span></a>-->
<!--		<a href="https://www.instagram.com/valentin_rogg/"><i class="fa-brands fa-instagram mr-2"></i><span class="text-skin-dark">valentin_rogg</span></a>-->
<!--	</p>-->
</section>
