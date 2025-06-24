import { version } from "../../package.json";
import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async () => {
	return {
		version
	};
};
