import { config, defaultLocale, locales } from "$lib/_i18n";
import type { Handle } from "@sveltejs/kit";
import { admin, darkMode } from "$lib/stores";
import * as cookie from "cookie";
import { ADMIN_PASSWORD } from "$env/static/private";

export const handle: Handle = async ({ event, resolve }) => {
	const { url, request } = event;
	const { pathname } = url;

	const lang: string = (pathname.match(/[^/]+?(?=\/|$)/) || [])[0];
	const preferredLocale = `${`${request.headers.get("accept-language")}`.match(
		/[a-zA-Z]+?(?=[-_,;])/
	)}`.toLowerCase();

	// If this request is a route request
	if (Object.keys(config.translations).includes(lang) || lang === undefined) {
		// Set dark mode
		const cookies = request.headers.get("cookie") || "";
		darkMode.set(cookie.parse(cookies)["darkModeEnabled"] === "true");

		// Get defined locales
		const supportedLocales = locales.get();

		// Try to get locale from `pathname`.
		let locale = supportedLocales.find(
			(l) => `${l}`.toLowerCase() === `${pathname.match(/[^/]+?(?=\/|$)/)}`.toLowerCase()
		);

		// If route locale is not supported
		if (!locale) {
			locale = preferredLocale;

			// Set default locale if user preferred locale does not match
			if (!supportedLocales.includes(locale)) locale = defaultLocale;

			// 301 redirect
			return new Response(undefined, {
				headers: { location: `/${locale}${pathname}` },
				status: 301
			});
		}

		// Add html `lang` attribute
		return resolve(event, {
			transformPageChunk: ({ html }) => html.replace(/<html.*>/, `<html lang="${locale}">`)
		});
	}

	if (pathname.includes("admin")) {
		const password = url.searchParams.get("pa");

		if (password !== ADMIN_PASSWORD) {
			return Response.redirect(`${event.url.origin}/${preferredLocale}`, 307);
		}
	}

	return resolve(event);
};
