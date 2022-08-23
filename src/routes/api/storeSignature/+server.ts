import type { Action } from "@sveltejs/kit";
import { FormData } from "formdata-node";
import { FormDataEncoder } from "form-data-encoder";
import { Readable } from "stream";

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export const POST: Action = async ({ request }) => {
  const uri = await request.json();

  const blob = await fetch(uri.image).then((res) => res.blob());

  const form = new FormData();
  form.set("file", blob, "signature.png");

  const encoder = new FormDataEncoder(form);

  const headers = Object.assign(
    { Authorization: `Bearer ${import.meta.env.VITE_CLOUDFLARE_IMAGES_TOKEN}` },
    encoder.headers
  );

  await fetch(
    `https://api.cloudflare.com/client/v4/accounts/${
      import.meta.env.VITE_CLOUDFLARE_IMAGES_ACCOUNT
    }/images/v1`,
    {
      method: "POST",
      headers,
      // eslint-disable-next-line @typescript-eslint/ban-ts-comment
      // @ts-ignore
      body: Readable.from(encoder)
    }
  )
    .then((res) => res.text())
    .then((text) => console.log(text));

  return new Response();
};
