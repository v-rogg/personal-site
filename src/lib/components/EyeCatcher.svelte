<script lang="ts">
	import Signature from "$lib/components/Signature.svelte";
	import SignatureNumber from "$lib/components/SignatureNumber.svelte";
	import { darkMode, admin } from "../stores";
	import SignatureAdmin from "$lib/components/SignatureAdmin.svelte";
	import { browser } from "$app/environment";
</script>

<section class="eyecatcher">
	{#if browser}
		<Signature />
		<SignatureNumber />
		{#if $admin}
			<SignatureAdmin />
		{/if}
	{/if}
	<picture>
		<source
			media="(max-width: 300px)"
			srcset="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/3b8293de-9bc4-4c08-623e-37635c553100/50" />
		<source
			media="(max-width: 1025px)"
			srcset="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/3b8293de-9bc4-4c08-623e-37635c553100/75" />
		<img
			src="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/3b8293de-9bc4-4c08-623e-37635c553100/100"
			alt="Handsome Portrait Valentin Rogg"
			width="608"
			height="575"
			style="display: {!$darkMode ? 'none' : 'block'}" />
	</picture>
	<picture>
		<source
			media="(max-width: 300px)"
			srcset="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/4d394d71-c80c-4146-8671-5cbd49119800/50" />
		<source
			media="(max-width: 1025px)"
			srcset="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/4d394d71-c80c-4146-8671-5cbd49119800/75" />
		<img
			src="https://imagedelivery.net/JEc1YLA5ZSivE42ux7pbDw/4d394d71-c80c-4146-8671-5cbd49119800/100"
			alt="Handsome Portrait Valentin Rogg"
			width="608"
			height="575"
			style="display: {$darkMode ? 'none' : 'block'}" />
	</picture>
</section>

<style>
	.eyecatcher {
		background: var(--c-secondary);
		color: var(--c-black) !important;
		flex-grow: 1;
		margin: 2rem;
		border-radius: 4px;
		position: relative;
		min-height: 670px;
		max-height: 760px;
		overflow: hidden;
	}

	@media (max-width: 540px) {
		.eyecatcher {
			margin: 1rem 1rem 1rem;
		}
	}

	/*.eyecatcher:after {*/
	/*	animation: grain 8s steps(10) infinite;*/
	/*	background-image: url("/noise.webp");*/
	/*	content: "";*/
	/*	height: 300%;*/
	/*	left: -50%;*/
	/*	position: absolute;*/
	/*	top: -100%;*/
	/*	width: 300%;*/
	/*	display: block;*/
	/*	mix-blend-mode: hard-light;*/
	/*}*/

	/*:global(.dark) .eyecatcher:after {*/
	/*	opacity: 0.1;*/
	/*	filter: brightness(1);*/
	/*}*/

	/*:global(.light) .eyecatcher:after {*/
	/*	opacity: 0.05;*/
	/*	filter: brightness(3);*/
	/*}*/

	/*@keyframes grain {*/
	/*	0%,*/
	/*	100% {*/
	/*		transform: translate(0, 0);*/
	/*	}*/
	/*	10% {*/
	/*		transform: translate(-5%, -10%);*/
	/*	}*/
	/*	20% {*/
	/*		transform: translate(-15%, 5%);*/
	/*	}*/
	/*	30% {*/
	/*		transform: translate(7%, -25%);*/
	/*	}*/
	/*	40% {*/
	/*		transform: translate(-5%, 25%);*/
	/*	}*/
	/*	50% {*/
	/*		transform: translate(-15%, 10%);*/
	/*	}*/
	/*	60% {*/
	/*		transform: translate(15%, 0%);*/
	/*	}*/
	/*	70% {*/
	/*		transform: translate(5%, 15%);*/
	/*	}*/
	/*	80% {*/
	/*		transform: translate(3%, 30%);*/
	/*	}*/
	/*	90% {*/
	/*		transform: translate(-10%, 10%);*/
	/*	}*/
	/*}*/

	img {
		position: absolute;
		bottom: 0;
		left: 50%;
		transform: translate(-50%);
	}
</style>
