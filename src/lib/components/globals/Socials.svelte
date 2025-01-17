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
				class="absolute bottom-full right-0 -m-[2px] w-max origin-right rounded-lg border-2 border-white-500 bg-white-700 px-2 py-1 text-center text-sm font-[450]"
				style="transform: translate(24px, -24px) rotateZ(-4deg);">Mein Lebenslauf</span
			>
			<span class="absolute bottom-full right-0">
				<svg
					width="20"
					viewBox="0 0 67 32"
					version="1.1"
					xmlns="http://www.w3.org/2000/svg"
					style="fill-rule:evenodd;clip-rule:evenodd;stroke-linejoin:round;stroke-miterlimit:2;"
					class="-translate-y-[2.5px] translate-x-[10px] -rotate-[80deg] -scale-x-100"
					><path
						d="M47.077,26.077C13.31,30.875 0.2,2.975 0.2,2.975C-0.291,1.935 0.153,0.692 1.193,0.2C2.232,-0.291 3.475,0.153 3.967,1.193C3.967,1.193 15.872,26.16 46.303,21.976L45.626,18.386C45.47,17.559 45.73,16.709 46.321,16.11C46.913,15.512 47.761,15.242 48.589,15.389L64.622,18.228C65.671,18.413 66.495,19.229 66.693,20.275C66.89,21.321 66.42,22.381 65.511,22.936L51.615,31.423C50.897,31.862 50.009,31.92 49.24,31.578C48.471,31.237 47.919,30.539 47.763,29.712L47.077,26.077Z"
					></path></svg
				>
			</span>
			<a
				href="https://r2.valentinrogg.de/files/Lebenslauf_Valentin_Rogg_250117.pdf"
				target="_blank"
				aria-label="Lebenslauf"><i class="fa-regular fa-memo"></i></a
			>
		</span>
		<a href="https://www.linkedin.com/in/vrogg/" rel="noreferrer nofollow" target="_blank" aria-label="LinkedIn"
			><i class="fa-brands fa-linkedin"></i></a
		>
		<a href="mailto:mail@valentinrogg.de" rel="noreferrer nofollow" target="_blank" class="mail" aria-label="Mail"
			><i class="fa-solid fa-envelope"></i></a
		>
	</div>
</div>
