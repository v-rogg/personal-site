// Shared state for carousel navigation
// This allows Analytics to tell SignatureCarousel which signature to scroll to

let scrollToCallback: ((id: string) => void) | null = null;

export function registerScrollHandler(callback: (id: string) => void) {
	scrollToCallback = callback;
}

export function unregisterScrollHandler() {
	scrollToCallback = null;
}

export function scrollToSignature(id: string) {
	if (scrollToCallback) {
		scrollToCallback(id);
	}
}
