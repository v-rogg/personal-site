<script>
	import Nav from '../components/Nav.svelte';
	import Header from "../components/Header.svelte";

	export let segment;

	export let lang;
</script>

<style>
	/*main {*/
	/*	position: relative;*/
	/*	max-width: 56em;*/
	/*	background-color: white;*/
	/*	padding: 2em;*/
	/*	margin: 0 auto;*/
	/*	box-sizing: border-box;*/
	/*}*/
	.fixed {
		position: fixed;
		/*background: white;*/
		width: 100%;
		top: 0;
	}

	.main--mt {
		margin-top: 12rem;
	}
</style>

<header class="fixed">
	<Header/>
	<Nav {segment}/>
</header>

<main class="container main--mt">
	<slot></slot>
</main>