<script lang="ts">
	import { currentSignatureStore, refIndexStore, signatureRefsStore } from "$lib/components/sections/EyeCatcher/signature.stores";
	import { enhance } from "$app/forms";
	import { page } from "$app/stores";
	import type { Signature, SignaturesResponse } from "$lib/fauna-gql/schema";
	import { gql } from "graphql-request";
	import { fail } from "@sveltejs/kit";
	import { client } from "$lib/fauna-gql/public.client";

	const password = $page.url.searchParams.get("pa");

	enum Filter {
		New,
		All
	}

	let filter = Filter.New;

	async function reloadSignatures() {
		let reloadedSignatures: SignaturesResponse;

		if (filter === Filter.New) {
			reloadedSignatures = await client
				.request(
					gql`
						query {
							allNewSignatures {
								data {
									_id
								}
							}
						}
					`
				)
				.then((res) => res.allNewSignatures)
				.catch((err) => {
					console.log(err);
				});
		} else if (filter === Filter.All) {
			reloadedSignatures = await client
				.request(
					gql`
						query {
							allSignatures {
								data {
									_id
								}
							}
						}
					`
				)
				.then((res) => res.allSignatures)
				.catch((err) => {
					console.log(err);
				});
		}

		if (!reloadedSignatures.data) return fail(500, { msg: "query failed" });

		if (reloadedSignatures.data.length > 0) {
			const firstResult = await client
				.request(
					gql`
						query ($id: ID!) {
							findSignatureByID(id: $id) {
								_id
								_ts
								name
								status
								ts_created
								signature {
									penColor
									minWidth
									maxWidth
									dotSize
									points {
										x
										y
										time
										pressure
									}
								}
							}
						}
					`,
					{ id: reloadedSignatures.data[0]._id }
				)
				.then((res) => res.findSignatureByID)
				.catch((err) => {
					console.log(err);
				});

			$signatureRefsStore = reloadedSignatures.data;
			$currentSignatureStore = firstResult;
			$refIndexStore = 0;
		} else {
			$signatureRefsStore = reloadedSignatures.data;
			$currentSignatureStore = <Signature>undefined;
			$refIndexStore = 0;
		}
	}
</script>

<div class="container container-tight">
	<div class="overlay">
		{#if $currentSignatureStore}
			<form
				class="admin-panel"
				method="post"
				use:enhance="{({ data }) => {
					data.set('id', $currentSignatureStore?._id);

					return async ({ result }) => {
						if (result.status === 200) $currentSignatureStore.status = result.data.status;
					};
				}}">
				<button
					id="approve"
					formaction="?/approve&pa={password}"
					class:notstatus="{$currentSignatureStore?.status !== 'approved' &&
						$currentSignatureStore?.status !== 'new'}">
					<i class="fa-regular fa-check"></i>
				</button>
				<button
					id="decline"
					formaction="?/decline&pa={password}"
					class:notstatus="{$currentSignatureStore?.status !== 'declined' &&
						$currentSignatureStore?.status !== 'new'}">
					<i class="fa-regular fa-xmark"></i>
				</button>
			</form>
		{/if}
		<div class="filter">
			<div class="amount">
				{$signatureRefsStore.length === 0 ? 0 : $refIndexStore + 1}
				<i class="fa-regular fa-slash-forward"></i>
				{$signatureRefsStore.length}
			</div>

			<button
				on:click="{() => {
					filter = Filter.New;
					reloadSignatures();
				}}"
				class="filter_switch"
				class:active="{filter === Filter.New}"
				disabled="{filter === Filter.New}"><span>New</span></button>
			<button
				on:click="{() => {
					filter = Filter.All;
					reloadSignatures();
				}}"
				class="filter_switch"
				class:active="{filter === Filter.All}"
				disabled="{filter === Filter.All}"><span>All</span></button>
		</div>
	</div>
</div>

<style lang="scss">
	.container {
		position: absolute;
		height: 100%;
		left: 50%;
		transform: translateX(-50%);
		z-index: 799;
	}

	.overlay {
		position: relative;
		height: 100%;
		width: 100%;
	}

	.admin-panel {
		position: absolute;
		bottom: 2rem;
		z-index: 800;
		left: 50%;
		transform: translateX(-50%);
		display: grid;
		grid-template-columns: repeat(2, 3rem);
		align-items: center;
		gap: 2rem;
	}

	.notstatus:before {
		opacity: 0.6;
		transform: scale(0.8);
	}

	#approve,
	#decline {
		border: none;
		font-size: 1rem;
		z-index: 250;
		width: 3rem;
		height: 3rem;
		border-radius: 100%;
		position: relative;
		color: var(--r-white);
		background: none;
		justify-self: center;
	}

	#approve:before,
	#decline:before {
		content: "";
		position: absolute;
		transition: 100ms ease-in-out;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		border-radius: 100%;
		z-index: -1;
	}

	#approve:hover,
	#decline:hover {
		opacity: 1;
		cursor: pointer;
	}

	#approve:before {
		background: var(--c-green);
	}

	#approve:hover:before {
		background: var(--c-green-hover);
	}

	#approve:active:before {
		background: var(--c-green-active);
	}

	#decline:before {
		background: var(--c-red);
	}

	#decline:hover:before {
		background: var(--c-red-hover);
	}

	#decline:active:before {
		background: var(--c-red-active);
	}

	.amount {
		align-self: center;
		margin-right: 1rem;
	}

	.filter {
		position: absolute;
		z-index: 250;
		bottom: 2rem;
		right: 0;
		display: flex;
		backdrop-filter: blur(8px);
		border-radius: 4px;
	}

	.filter_switch {
		width: 4rem;
		height: 2rem;
		border: none;
		border-radius: 4px;
		background: none;
		font-size: var(--fs-10);
		display: flex;
		justify-content: center;
		align-items: center;
		position: relative;

		&:before {
			position: absolute;
			content: "";
			border-radius: 4px;
			top: 0;
			left: 0;
			width: 100%;
			height: 100%;
			z-index: -1;
		}
	}

	@keyframes in-right {
		0% {
			left: 100%;
		}
		100% {
			left: 0;
		}
	}

	@keyframes in-left {
		0% {
			left: -100%;
		}
		100% {
			left: 0;
		}
	}

	.filter_switch > span {
		//transform: translateY(-1px);
	}

	.active {
		font-weight: 700;
		&:before {
			background: var(--c-bg);
			animation: in-right 250ms ease-in-out;
		}
	}

	.active:last-of-type:before {
		animation: in-left 250ms ease-in-out;
	}

	.filter_switch:hover:not(.active) {
		//background: var(--c-bg-hover);
		cursor: pointer;
	}

	.filter_switch:active:not(.active) {
		//background: var(--c-bg-active);
		cursor: default;
	}
</style>
