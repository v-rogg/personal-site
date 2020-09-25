<script context="module">
    const arrowRight = '<span class="mono">~~></span>';
</script>

<script>
	export let status;
	export let error;

	const dev = process.env.NODE_ENV === 'development';
</script>

<style lang="scss">
	h1, p {
		margin: 0 auto;
	}

	h1 {
		font-size: 2.8em;
		font-weight: 700;
		margin: 0 0 0.5em 0;
	}

	p {
		margin: 1em auto;
	}

	@media (min-width: 480px) {
		h1 {
			font-size: 4em;
		}
	}

    a {
        display: inline-block;
        border: 1px solid black;
        margin-top: 3rem;
        padding: 1rem;
        transition: all linear 100ms;

        &:hover {
            background: black;
            color: white;
        }
    }
</style>

<svelte:head>
	<title>{status}</title>
</svelte:head>

<section class="container">
    <h1>{status}</h1>

    <p>{error.message}</p>

    {#if dev && error.stack}
        <pre>{error.stack}</pre>
    {/if}

    <a rel=external href="/">Wieder aufs Trockene {@html arrowRight}</a>
</section>
