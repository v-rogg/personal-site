<script>
	import {stores} from '@sapper/app';
	import ACA from "../components/projects/ACA.svelte";
	import Dedun from "../components/projects/Dedun.svelte";
	import Montgolfiere from "../components/projects/Montgolfiere.svelte";

	const {preloading, page, session} = stores();
</script>

<svelte:head>
	<title>Valentin Rogg</title>
</svelte:head>

<div>
	<ACA></ACA>
	<Dedun></Dedun>
	<Montgolfiere></Montgolfiere>
</div>
