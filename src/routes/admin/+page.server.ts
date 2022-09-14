import type { PageServerLoad } from "./$types";
import { redirect } from "@sveltejs/kit";

export const load: PageServerLoad = async ({ url }) => {
  const password = url.searchParams.get("pa");

  if (password !== import.meta.env.VITE_ADMIN_PASSWORD) {
    throw redirect(307, `/`)
  }
}
