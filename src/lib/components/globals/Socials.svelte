<script lang="ts">
	let { cls = "text-2xl max-lg:mx-auto mt-16" } = $props();
</script>

<div class="{cls} w-max text-black">
	<div class="flex gap-4">
		<a href="https://github.com/v-rogg/personal-site" rel="noreferrer nofollow" target="_blank" aria-label="GitHub"
			><i class="fa-brands fa-github"></i></a
		>
		<a href="https://www.instagram.com/valentin_rogg/" rel="noreferrer nofollow" target="_blank" aria-label="Instagram"
			><i class="fa-brands fa-instagram"></i></a
		>
		<a href="https://www.youtube.com/@roggnroll" rel="noreferrer nofollow" target="_blank" aria-label="YouTube"
			><i class="fa-brands fa-youtube"></i></a
		>
		<span class="relative">
			<span
				class="absolute bottom-full right-0 w-max origin-right text-center font-hand text-sm leading-[0]"
				style="transform: translate(24px, -24px) rotateZ(-4deg);">Mein Lebenslauf</span
			>
			<span class="absolute bottom-full right-0">
				<svg width="9" height="17" viewBox="0 0 9 17" fill="none" class="arrow" xmlns="http://www.w3.org/2000/svg">
					<path
						d="M1.59524 0.236142C2.42858 0.747671 3.30952 1.28245 3.92857 2.00324C6.28572 4.7469 7 8.00209 6.5 11.513C6.33333 12.7221 5.92857 13.8847 5.59524 15.1635C6.7381 14.6055 7.85714 14.2799 9 15.117C7.5 15.303 6.26191 16.0238 5.04762 16.7446C4.90477 16.8143 4.78572 16.8841 4.64286 16.9306C4.04762 17.1166 3.66667 16.9306 3.5 16.3493C3.42858 16.1401 3.42857 15.9075 3.40476 15.6983C3.30952 13.9312 3.07143 12.2106 2.42858 10.5365C2.38096 10.397 2.45238 10.1645 2.47619 9.74594C3.7619 11.234 3.7619 12.9314 4.2619 14.559C5.61905 11.8153 5.92857 9.0949 5.09524 6.28149C4.30952 3.53783 3.07143 1.1662 0 0.236142C0.761905 -0.0428741 1 -0.112628 1.59524 0.236142Z"
						fill="black"
					></path>
				</svg>
			</span>
			<a href="https://www.linkedin.com/in/vrogg/" rel="noreferrer nofollow" target="_blank" aria-label="LinkedIn"
				><i class="fa-brands fa-linkedin"></i></a
			>
		</span>
		<a href="mailto:mail@valentinrogg.de" rel="noreferrer nofollow" target="_blank" class="mail" aria-label="Mail"
			><i class="fa-solid fa-envelope"></i></a
		>
	</div>
</div>
