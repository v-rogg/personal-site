<script>
	import {stores} from '@sapper/app';
	import ACA from "./projects/ACA.svelte";
	import Dedun from "./projects/Dedun.svelte";
	import Montgolfiere from "./projects/Montgolfiere.svelte";

	const {preloading, page, session} = stores();
</script>

<svelte:head>
	<title>Valentin Rogg</title>
	<link rel='icon' type='image/png' href='favicon.png'>
</svelte:head>

<!--Projects-->
<ACA></ACA>
<Dedun></Dedun>
<Montgolfiere></Montgolfiere>
