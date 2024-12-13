import { fail, type Actions } from "@sveltejs/kit";
import { env } from "$env/dynamic/private";
export const prerender = false;

export const actions = {
	sendMail: async ({ request, fetch }) => {
		const form = await request.formData();
		const email = form.get("email");
		const message = form.get("message");
		const turnstile = form.get("cf-turnstile-response");

		const url = "https://challenges.cloudflare.com/turnstile/v0/siteverify";
		const turnstile_validation = await fetch(url, {
			body: JSON.stringify({
				secret: env.CF_TURNSTILE_SECRET_KEY,
				response: turnstile
			}),
			method: "POST",
			headers: {
				"Content-Type": "application/json"
			}
		}).then(async (res) => (await res.json()) as { success: boolean });

		if (turnstile_validation.success) {
			try {
				await fetch("https://api.resend.com/emails", {
					method: "POST",
					headers: {
						Authorization: `Bearer ${env.RESEND_TOKEN}`,
						"Content-Type": "application/json"
					},
					body: JSON.stringify({
						from: "work@mail.valentinrogg.de",
						to: "mail@valentinrogg.de",
						subject: `Work Request - ${email}`,
						html: `Requester: ${email}<br/> ${message}`
					})
				});
				return { success: true };
			} catch (error) {
				return fail(500, { msg: String(error) });
			}
		} else {
			fail(401);
		}
	}
} satisfies Actions;
