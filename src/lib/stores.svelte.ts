import type { Metadata } from "./types";

export const appState: { metadata: Metadata | undefined } = $state({
	metadata: undefined
});
