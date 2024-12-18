import { checkSignature, getSignatures } from "$lib/d1";
import type { PageServerLoad } from "./$types";

export const load: PageServerLoad = async ({ platform, url }) => {
	if (platform) {
		const signatures = (await getSignatures(platform)).sort(() => Math.random() - 0.5);

		console.log(signatures.length);

		const requestedSignature = url.searchParams.get("s");
		if (requestedSignature) {
			const check = await checkSignature(platform, requestedSignature);

			if (check) {
				const index = signatures.findIndex((signature) => signature.id === check.id);
				let updatedSignatures = [...signatures];
				if (index === -1) {
					updatedSignatures.unshift(check);
				} else {
					updatedSignatures = [
						updatedSignatures[index],
						...updatedSignatures.slice(0, index),
						...updatedSignatures.slice(index + 1)
					];
				}

				console.log(updatedSignatures.length);

				return { signatures: updatedSignatures, autoplay: false };
			}
		}

		return { signatures, autoplay: true };
	} else {
		return {};
	}
};
