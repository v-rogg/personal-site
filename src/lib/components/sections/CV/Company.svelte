<script>
	import { storyblokEditable, StoryblokComponent } from '@storyblok/svelte';

	export let blok;
</script>

<div use:storyblokEditable={blok}>
	{blok.title}

	<ul>
		{#each blok.jobs as job}
			<StoryblokComponent blok={job} />
		{/each}
	</ul>
</div>