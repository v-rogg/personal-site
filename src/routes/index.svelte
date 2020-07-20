<script>
	import {stores} from '@sapper/app';
	import ACA from "./projects/ACA.svelte";
	import Dedun from "./projects/Dedun.svelte";
	import Jobs from "../components/Jobs.svelte";
	import Video from "../components/Video.svelte";

	const {preloading, page, session} = stores();
</script>

<style lang="scss">
	img {
		width: 100px;
	}

	.container {
		margin: 0 auto 2rem;
	}
</style>

<svelte:head>
	<title>Valentin Rogg</title>
	<link rel='icon' type='image/png' href='favicon.png'>
</svelte:head>

<!--Projects-->
<!--<Jobs/>-->
<!--<Video></Video>-->
<section>
	<div class="container">
		<img src="https://www.freeiconspng.com/uploads/vaporwave-font-choice-japanese-signs-gradient-3d-png-3.png">
	</div>
</section>

<ACA/>
<Dedun/>
