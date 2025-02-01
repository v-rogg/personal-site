<script lang="ts">
	import { appState } from "$lib/stores.svelte";
	import { onMount } from "svelte";

	let dice = $state([-1, -1]);
	let tasks: string[] = $state(["", "", "", "", "", ""]);
	let activeTasksCount = $derived(tasks.filter((t) => t !== "").length);

	function roll() {
		const array = new Uint16Array(2);
		crypto.getRandomValues(array);
		dice = [(array[0] % activeTasksCount) + 1, (array[1] % 6) + 1];
	}

	function storeTasks() {
		localStorage.setItem("vr-www.taskroll.tasks", JSON.stringify(tasks));
	}

	onMount(() => {
		appState.metadata = {
			slug: "TaskRoll",
			title: "",
			preview: "",
			published: true,
			date: ""
		};

		tasks = JSON.parse(localStorage.getItem("vr-www.taskroll.tasks") || "") || ["", "", "", "", "", ""];
	});
</script>

<main class="container mx-auto mt-48 px-10">
	<div class="prose max-w-full">
		<div class="flex flex-col items-center justify-between gap-8">
			<h1 class="my-0 py-0 leading-none">TaskRoll</h1>
			<div>
				<button
					onclick={roll}
					disabled={activeTasksCount === 0}
					class="rounded-md px-2 py-2 opacity-100 transition hover:bg-white-600 active:bg-white-700 disabled:opacity-0"
				>
					<i class="mr-2">Roll die Würfel</i>
					<i class="fa-solid fa-dice fa-2xl"></i>
				</button>
			</div>
		</div>
		<div class="mx-auto flex justify-center gap-4 max-sm:flex-col max-sm:items-center sm:gap-10">
			<div class="w-max">
				<h2>Aufgabe</h2>
				<div class="list tasklist flex flex-col gap-2">
					<div class:active={dice[0] === 1}>
						{#if dice[0] === 1}
							<i class="fa-solid fa-dice-one fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-one fa-xl mr-4"></i>
						{/if}
						<input
							type="text"
							name="task1"
							class="taskinput"
							placeholder="Aufgabe 1"
							bind:value={tasks[0]}
							oninput={storeTasks}
						/>
					</div>
					<div class:active={dice[0] === 2}>
						{#if dice[0] === 2}
							<i class="fa-solid fa-dice-two fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-two fa-xl mr-4"></i>
						{/if}
						<input
							type="text"
							name="task2"
							class="taskinput"
							placeholder="Aufgabe 2"
							bind:value={tasks[1]}
							oninput={storeTasks}
							disabled={activeTasksCount < 1}
						/>
					</div>
					<div class:active={dice[0] === 3}>
						{#if dice[0] === 3}
							<i class="fa-solid fa-dice-three fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-three fa-xl mr-4"></i>
						{/if}
						<input
							type="text"
							name="task3"
							class="taskinput"
							placeholder="Aufgabe 3"
							bind:value={tasks[2]}
							oninput={storeTasks}
							disabled={activeTasksCount < 2}
						/>
					</div>
					<div class:active={dice[0] === 4}>
						{#if dice[0] === 4}
							<i class="fa-solid fa-dice-four fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-four fa-xl mr-4"></i>
						{/if}
						<input
							type="text"
							name="task4"
							class="taskinput"
							placeholder="Aufgabe 4"
							bind:value={tasks[3]}
							oninput={storeTasks}
							disabled={activeTasksCount < 3}
						/>
					</div>
					<div class:active={dice[0] === 5}>
						{#if dice[0] === 5}
							<i class="fa-solid fa-dice-five fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-five fa-xl mr-4"></i>
						{/if}
						<input
							type="text"
							name="task5"
							class="taskinput"
							placeholder="Aufgabe 5"
							bind:value={tasks[4]}
							oninput={storeTasks}
							disabled={activeTasksCount < 4}
						/>
					</div>
					<div class:active={dice[0] === 6}>
						{#if dice[0] === 6}
							<i class="fa-solid fa-dice-six fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-six fa-xl mr-4"></i>
						{/if}
						<input
							type="text"
							name="task6"
							class="taskinput"
							placeholder="Aufgabe 6"
							bind:value={tasks[5]}
							oninput={storeTasks}
							disabled={activeTasksCount < 5}
						/>
					</div>
					<div>
						<button
							onclick={() => {
								tasks = ["", "", "", "", "", ""];
								storeTasks();
							}}
							class="opacity-100 transition disabled:opacity-0"
							disabled={activeTasksCount <= 0}
						>
							<i class="fa-regular fa-xmark fa-xl mr-8"></i>
							Löschen
						</button>
					</div>
				</div>
			</div>
			<div class="w-max">
				<h2>Dauer</h2>
				<div class="list flex flex-col gap-2">
					<div class:active={dice[1] === 1}>
						{#if dice[1] === 1}
							<i class="fa-solid fa-dice-one fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-one fa-xl mr-4"></i>
						{/if}
						10 Minuten
					</div>
					<div class:active={dice[1] === 2}>
						{#if dice[1] === 2}
							<i class="fa-solid fa-dice-two fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-two fa-xl mr-4"></i>
						{/if}
						20 Minuten
					</div>
					<div class:active={dice[1] === 3}>
						{#if dice[1] === 3}
							<i class="fa-solid fa-dice-three fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-three fa-xl mr-4"></i>
						{/if}
						30 Minuten
					</div>
					<div class:active={dice[1] === 4}>
						{#if dice[1] === 4}
							<i class="fa-solid fa-dice-four fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-four fa-xl mr-4"></i>
						{/if}
						40 Minuten
					</div>
					<div class:active={dice[1] === 5}>
						{#if dice[1] === 5}
							<i class="fa-solid fa-dice-five fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-five fa-xl mr-4"></i>
						{/if}
						50 Minuten
					</div>
					<div class:active={dice[1] === 6}>
						{#if dice[1] === 6}
							<i class="fa-solid fa-dice-six fa-xl mr-4"></i>
						{:else}
							<i class="fa-regular fa-dice-six fa-xl mr-4"></i>
						{/if}
						10 Minuten Pause
					</div>
				</div>
			</div>
		</div>
	</div>
</main>

<style lang="postcss">
	.taskinput {
		@apply h-12 rounded-md bg-white-600 px-4 py-2;

		&::placeholder {
			@apply text-white-700;
		}

		&:disabled {
			@apply bg-transparent;
		}
	}

	.tasklist > div {
		@apply !pr-0;
	}

	.list > div {
		@apply flex h-12 items-center rounded-md px-4 transition;
	}

	.active {
		@apply outline outline-1 outline-white-700;
	}
</style>
