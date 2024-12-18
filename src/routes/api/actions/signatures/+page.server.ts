import { env } from "$env/dynamic/private";
import { postSignature } from "$lib/d1";
import { fail, type Actions } from "@sveltejs/kit";

export const actions = {
	create: async ({ request, platform }) => {
		const formData = await request.formData();
		const name = formData.get("name")?.toString();
		const email = formData.get("email")?.toString();
		const signature = formData.get("signature")?.toString();

		if (!name || !signature || !platform) {
			return fail(400, {
				msg: "Missing input data"
			});
		}

		const result = await postSignature(platform, name, signature, email);

		await fetch("https://api.resend.com/emails", {
			method: "POST",
			headers: {
				Authorization: `Bearer ${env.RESEND_TOKEN}`,
				"Content-Type": "application/json"
			},
			body: JSON.stringify({
				from: "work@mail.valentinrogg.de",
				to: "mail@valentinrogg.de",
				subject: `Signature Created - ${name}`,
				html: `URL: <a href="https://valentinrogg.de/?s=${result.id}">${name}</a><br/>ID: ${result.id}<br/>EMail: ${email}`
			})
		});

		return {
			success: result.success,
			id: result.id
		};
	}
} satisfies Actions;
