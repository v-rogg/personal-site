<script lang="ts">
	import { darkMode } from "$lib/stores";
	import { t } from "$lib/_i18n";

	function toggle_dark_mode() {
		$darkMode = !$darkMode;
		document.cookie = `darkModeEnabled=${$darkMode};expires=Thu, 18 Dec 2300 12:00:00 UTC; path=/; SameSite=Strict`;
		// localStorage.setItem("darkModeEnabled", $dark_mode)
	}
</script>

<button
	class="dark_switch"
	on:click="{() => toggle_dark_mode()}"
	aria-label="{$t('common.dark_mode')}">
	<span class:hidden="{!$darkMode}">
		<i class="fa-solid fa-moon"></i>
	</span>
	<span class:hidden="{$darkMode}">
		<i class="fa-solid fa-sun"></i>
	</span>
</button>

<style>
	.dark_switch {
		position: relative;
		width: 1.25rem;
		height: 1.25rem;
		border: none;
		background: none;
		margin: 0;
		padding: 0;
		font-size: 16px;
		color: var(--c-black);
		border-radius: 50%;
	}

	.dark_switch:hover {
		cursor: pointer;
	}

	.dark_switch:focus-visible {
		border: none;
		outline: 2px solid var(--c-focus);
	}

	span {
		position: absolute;
		top: 50%;
		left: 50%;
		display: block;
		transform-origin: 50% 500%;
		transform: translate(-50%, -50%);
		animation: in 300ms ease-in-out;
	}

	.hidden {
		animation: out 300ms ease-in-out;
		opacity: 0;
	}

	@keyframes in {
		0% {
			transform: translate(-50%, -50%) rotateZ(15deg);
			opacity: 0;
		}

		20% {
			opacity: 0;
		}

		90% {
			opacity: 1;
		}

		100% {
			transform: translate(-50%, -50%) rotateZ(0deg);
			opacity: 1;
		}
	}

	@keyframes out {
		0% {
			transform: translate(-50%, -50%) rotateZ(0deg);
			opacity: 1;
		}

		10% {
			opacity: 1;
		}

		80% {
			opacity: 0;
		}

		100% {
			transform: translate(-50%, -50%) rotateZ(-15deg);
			opacity: 0;
		}
	}
</style>
