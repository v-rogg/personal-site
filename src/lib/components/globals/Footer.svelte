<script lang="ts">
	let date = new Date();
</script>

<footer
	class="container relative mx-auto mb-16 mt-16 flex flex-col-reverse items-center justify-between gap-8 px-10 sm:flex-row sm:items-end"
>
	<div class="flex w-fit flex-col gap-1 text-center text-sm sm:text-left">
		<a href="/">Impressum & Datenschutz</a>
		<div>
			{date.toLocaleDateString("de")}
			<i class="fa-regular fa-copyright ml-2"></i>
			Valentin Rogg
		</div>
	</div>

	<div class="flex gap-4 justify-self-end text-md-lg">
		<a href="https://www.komoot.de/user/2406562570904" rel="noreferrer nofollow" target="_blank" aria-label="Komoot">
			<i class="fa-kit fa-komoot"></i>
		</a>
		<a href="https://www.instagram.com/valentin_rogg/" rel="noreferrer nofollow" target="_blank" aria-label="Instagram"
			><i class="fa-brands fa-instagram"></i></a
		>
		<a href="https://www.youtube.com/@roggnroll" rel="noreferrer nofollow" target="_blank" aria-label="YouTube"
			><i class="fa-brands fa-youtube"></i></a
		>
		<a href="https://www.linkedin.com/in/vrogg/" rel="noreferrer nofollow" target="_blank" aria-label="LinkedIn"
			><i class="fa-brands fa-linkedin"></i></a
		>
		<a href="mailto:mail@valentinrogg.de" rel="noreferrer nofollow" target="_blank" class="mail" aria-label="Mail"
			><i class="fa-solid fa-envelope"></i></a
		>
	</div>
</footer>

<style lang="postcss">
	/* @media (max-width: 1265px) {
		footer {
			display: flex;
			flex-direction: column-reverse;
			align-items: center;
			gap: 3rem;
			position: relative;
		}
	} */
</style>
