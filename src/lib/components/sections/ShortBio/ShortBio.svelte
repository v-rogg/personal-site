<script lang="ts">
	import { t } from "$lib/_i18n";
	import { sendClientEvent } from "$lib/posthog";

	export let blok;
</script>


<div class="px-16 flex flex-col justify-center min-h-[25rem]">
	<h1 class="text-4xl font-semibold mb-4 font-sans text-black dark:text-white-500" class:text-center={blok.content.align === "center"} class:text-right={blok.content.align === "right"}>Valentin Rogg</h1>
	{#if blok.content.flags}
	<p class="my-4 flex flex-wrap gap-0.5 max-sm:mx-10" id="flags" class:justify-center={blok.content.align === "center"} class:justify-end={blok.content.align === "right"}>
		<img
			class="emoji"
			alt="🇦🇹"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1e6-1f1f9.png" />
		<img
			class="emoji"
			alt="🇧🇪"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1e7-1f1ea.png" />
		<img
			class="emoji"
			alt="🇨🇦"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1e8-1f1e6.png" />
		<img
			class="emoji"
			alt="🇨🇿"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1e8-1f1ff.png" />
		<img
			class="emoji"
			alt="🇩🇰"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1e9-1f1f0.png" />
		<img
			class="emoji"
			alt="🇪🇬"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1ea-1f1ec.png" />
		<img
			class="emoji"
			alt="🇪🇪"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1ea-1f1ea.png" />
		<img
			class="emoji"
			alt="🇫🇴"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1eb-1f1f4.png" />
		<img
			class="emoji"
			alt="🇫🇮"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1eb-1f1ee.png" />
		<img
			class="emoji"
			alt="🇫🇷"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1eb-1f1f7.png" />
		<img
			class="emoji"
			alt="🇩🇪"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1e9-1f1ea.png" />
		<img
			class="emoji"
			alt="🇬🇱"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1ec-1f1f1.png" />
		<img
			class="emoji"
			alt="🇮🇸"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1ee-1f1f8.png" />
		<img
			class="emoji"
			alt="🇮🇪"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1ee-1f1ea.png" />
		<img
			class="emoji"
			alt="🇮🇹"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1ee-1f1f9.png" />
		<img
			class="emoji"
			alt="🇱🇻"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1f1-1f1fb.png" />
		<img
			class="emoji"
			alt="🇲🇨"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1f2-1f1e8.png" />
		<img
			class="emoji"
			alt="🇳🇱"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1f3-1f1f1.png" />
		<img
			class="emoji"
			alt="🇳🇴"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1f3-1f1f4.png" />
		<img
			class="emoji"
			alt="🇵🇱"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1f5-1f1f1.png" />
		<img
			class="emoji"
			alt="🇵🇹"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1f5-1f1f9.png" />
		<img
			class="emoji"
			alt="🇷🇺"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1f7-1f1fa.png" />
		<img
			class="emoji"
			alt="🇪🇸"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1ea-1f1f8.png" />
		<img
			class="emoji"
			alt="🇸🇪"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1f8-1f1ea.png" />
		<img
			class="emoji"
			alt="🇨🇭"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1e8-1f1ed.png" />
		<img
			class="emoji"
			alt="🇹🇷"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1f9-1f1f7.png" />
		<img
			class="emoji"
			alt="🏴󠁧󠁢󠁥󠁮󠁧󠁿"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f3f4-e0067-e0062-e0065-e006e-e0067-e007f.png" /><img
			class="emoji"
			alt="🏴󠁧󠁢󠁳󠁣󠁴󠁿"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f3f4-e0067-e0062-e0073-e0063-e0074-e007f.png" /><img
			class="emoji"
			alt="🏴󠁧󠁢󠁷󠁬󠁳󠁿"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f3f4-e0067-e0062-e0077-e006c-e0073-e007f.png" /><img
			class="emoji"
			alt="🇺🇸"
			src="https://cdn.jsdelivr.net/gh/twitter/twemoji@14.0.2/assets/72x72/1f1fa-1f1f8.png" />
	</p>
	{/if}
	<p class="font-serif text-md-lg whitespace-pre-wrap max-w-xl text-black dark:text-white-500" class:mx-auto={blok.content.align === "center"} class:ml-auto={blok.content.align === "right"} class:text-center={blok.content.align === "center"} class:text-right={blok.content.align === "right"}>
		{@html blok.content.short_bio}
	</p>

	<div class="w-max mt-16 mb-6 text-2xl text-black dark:text-white-500" class:mx-auto={blok.content.align === "center"} class:ml-auto={blok.content.align === "right"}>
		<div class="flex gap-4">
			<a
				href="https://www.komoot.de/user/2406562570904"
				rel="noreferrer nofollow"
				target="_blank"
				aria-label="Komoot"
				on:click="{() => sendClientEvent('open-social', document, { social: 'komoot' })}">
				<i class="fa-kit fa-komoot"></i>
			</a>
			<a
				href="https://www.instagram.com/valentin_rogg/"
				rel="noreferrer nofollow"
				target="_blank"
				aria-label="Instagram"
				on:click="{() => sendClientEvent('open-social', document, { social: 'instagram' })}"
				><i class="fa-brands fa-instagram"></i></a>
			<a
				href="https://www.youtube.com/@roggnroll"
				rel="noreferrer nofollow"
				target="_blank"
				aria-label="YouTube"
				on:click="{() => sendClientEvent('open-social', document, { social: 'youtube' })}"
				><i class="fa-brands fa-youtube"></i></a>
			<span class="overflow_hand cv">
				<span class="looking_for_cv font-handwriting">{@html $t("common.looking_for_cv", {locale: "de"}) || ""}</span>
				<span class="hand_arrow">
					<svg width="9" height="17" viewBox="0 0 9 17" fill="none" class="arrow" xmlns="http://www.w3.org/2000/svg">
						<path
							d="M1.59524 0.236142C2.42858 0.747671 3.30952 1.28245 3.92857 2.00324C6.28572 4.7469 7 8.00209 6.5 11.513C6.33333 12.7221 5.92857 13.8847 5.59524 15.1635C6.7381 14.6055 7.85714 14.2799 9 15.117C7.5 15.303 6.26191 16.0238 5.04762 16.7446C4.90477 16.8143 4.78572 16.8841 4.64286 16.9306C4.04762 17.1166 3.66667 16.9306 3.5 16.3493C3.42858 16.1401 3.42857 15.9075 3.40476 15.6983C3.30952 13.9312 3.07143 12.2106 2.42858 10.5365C2.38096 10.397 2.45238 10.1645 2.47619 9.74594C3.7619 11.234 3.7619 12.9314 4.2619 14.559C5.61905 11.8153 5.92857 9.0949 5.09524 6.28149C4.30952 3.53783 3.07143 1.1662 0 0.236142C0.761905 -0.0428741 1 -0.112628 1.59524 0.236142Z"
							fill="var(--c-black)"></path>
					</svg>
				</span>
				<a
					href="https://www.linkedin.com/in/vrogg/"
					rel="noreferrer nofollow"
					target="_blank"
					aria-label="LinkedIn"
					on:click="{() => sendClientEvent('open-social', document, { social: 'linkedin' })}"
					><i class="fa-brands fa-linkedin"></i></a>
			</span>
			<a
				href="mailto:mail@valentinrogg.de"
				rel="noreferrer nofollow"
				target="_blank"
				class="mail"
				aria-label="Mail"
				on:click="{() => sendClientEvent('open-social', document, { social: 'mail' })}"
				><i class="fa-solid fa-envelope"></i></a>
		</div>
	</div>
</div>

<style lang="scss">
	.overflow_hand {
		position: relative;
	}

	:global(img.emoji) {
		height: 1em;
		width: 1em;
		margin: 0 0.05em 0 0.1em;
		vertical-align: -0.1em;
	}

	.hand_arrow {
		position: absolute;
		bottom: 100%;
		right: 0;
		transform: translate(-4px, 0);
	}

	.looking_for_cv {
		position: absolute;
		width: max-content;
		font-size: 1.1rem;
		bottom: 100%;
		right: 0;
		transform: translate(24px, -24px) rotateZ(-4deg);
		transform-origin: right;
		text-align: center;
		line-height: 1;
	}
</style>
