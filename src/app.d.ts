/// <reference types="@cloudflare/workers-types" />
// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		interface Platform {
			env: {
				D1: D1Database;
				KV: KVNamespace;
				SIGNATURES_WORKER: Fetcher;
				SIGNATURES_WORKER_KEY: string;
			};
		}
	}
}

export {};
