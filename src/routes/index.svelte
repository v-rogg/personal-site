<script>
	import {stores} from '@sapper/app';
	import ACA from "../components/projects/ACA.svelte";
	import Dedun from "../components/projects/Dedun.svelte";
	import Montgolfiere from "../components/projects/Montgolfiere.svelte";

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
