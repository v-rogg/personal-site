declare namespace turnstile {
	interface TurnstileOptions {
		sitekey: string;
		callback?: (token: string) => void;
		"expired-callback"?: () => void;
		"error-callback"?: () => void;
		theme?: "light" | "dark" | "auto";
		tabindex?: number;
		appearance?: "always" | "execute" | "interaction-only";
		size?: "normal" | "compact";
		action?: string;
		cData?: string;
		retry?: "auto" | "never";
		refresh_expired?: "auto" | "manual" | "never";
	}

	interface Turnstile {
		render: (container: string | HTMLElement, options: TurnstileOptions) => string;
		reset: (widgetId: string) => void;
		remove: (widgetId: string) => void;
		getResponse: (widgetId: string) => string | undefined;
	}
}

interface Window {
	turnstile: turnstile.Turnstile;
}

declare const turnstile: turnstile.Turnstile;
