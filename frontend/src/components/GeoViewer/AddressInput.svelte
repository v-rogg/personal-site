<script lang="ts">
	import { geocode, type GeocodingResult } from "../../lib/geo/nominatim";

	let { onSelect, initialValue = "" }: { onSelect: (result: GeocodingResult) => void; initialValue?: string } = $props();

	let query = $state(initialValue);
	let results = $state<GeocodingResult[]>([]);
	let showDropdown = $state(false);
	let loading = $state(false);
	let debounceTimer: ReturnType<typeof setTimeout>;
	let abortController: AbortController | null = null;

	function handleInput() {
		clearTimeout(debounceTimer);
		if (query.length < 3) {
			results = [];
			showDropdown = false;
			return;
		}
		debounceTimer = setTimeout(doSearch, 300);
	}

	async function doSearch() {
		abortController?.abort();
		abortController = new AbortController();
		loading = true;
		try {
			results = await geocode(query, abortController.signal);
			showDropdown = results.length > 0;
		} catch {
			// Aborted or failed
		} finally {
			loading = false;
		}
	}

	function selectResult(result: GeocodingResult) {
		query = result.displayName;
		showDropdown = false;
		results = [];
		onSelect(result);
	}

	async function handleKeydown(e: KeyboardEvent) {
		if (e.key !== "Enter") return;
		e.preventDefault();
		// If dropdown is open, select the first result
		if (showDropdown && results.length > 0) {
			selectResult(results[0]);
			return;
		}
		// Otherwise, force a search and auto-select the first result
		if (query.length >= 3) {
			loading = true;
			try {
				const searchResults = await geocode(query);
				if (searchResults.length > 0) {
					selectResult(searchResults[0]);
				}
			} catch {
				// Failed
			} finally {
				loading = false;
			}
		}
	}
</script>

<div class="relative w-full">
	<input
		type="text"
		bind:value={query}
		oninput={handleInput}
		onkeydown={handleKeydown}
		onfocus={() => results.length > 0 && (showDropdown = true)}
		onblur={() => setTimeout(() => (showDropdown = false), 200)}
		placeholder="Adresse in Bayern eingeben..."
		style="height: 42px; font-size: 14px; padding: 8px 12px; border-radius: 6px; border: 1px solid rgba(0,0,0,0.1);"
		class="w-full bg-white text-black placeholder:text-skin-500 focus:outline-none transition"
	/>
	{#if loading}
		<div class="absolute right-3 top-1/2 -translate-y-1/2">
			<div class="w-4 h-4 border-2 border-white-700 border-t-black/60 rounded-full animate-spin"></div>
		</div>
	{/if}

	{#if showDropdown}
		<div
			class="absolute top-full left-0 right-0 bg-white z-50 overflow-hidden"
			style="margin-top: 4px; border: 1px solid rgba(0,0,0,0.1); border-radius: 6px;"
		>
			{#each results as result}
				<button
					class="w-full text-left text-sm text-black hover:bg-white-600 transition"
					style="padding: 8px 12px;"
					onmousedown={() => selectResult(result)}
				>
					{result.displayName}
				</button>
			{/each}
		</div>
	{/if}
</div>
