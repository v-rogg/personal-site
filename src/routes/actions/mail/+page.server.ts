import { fail, type Actions } from "@sveltejs/kit";
import { RESEND_TOKEN } from "$env/static/private";
export const prerender = false;
export const actions = {
	sendMail: async ({ request, fetch }) => {
		const form = await request.formData();
		const email = form.get("email");
		const message = form.get("message");
		try {
			await fetch("https://api.resend.com/emails", {
				method: "POST",
				headers: {
					Authorization: `Bearer ${RESEND_TOKEN}`,
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
	}
} satisfies Actions;
