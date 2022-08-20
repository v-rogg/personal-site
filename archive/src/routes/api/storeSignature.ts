import {FormData} from 'formdata-node';

export async function post({request}) {
  const uri = await request.json();

  const blob = await fetch(uri.image)
    .then(res => res.blob())

  const form = new FormData();
  form.set('file', blob, 'signature.png');

  fetch(`https://api.cloudflare.com/client/v4/accounts/${import.meta.env.VITE_CLOUDFLARE_IMAGES_ACCOUNT}/images/v1`, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${import.meta.env.VITE_CLOUDFLARE_IMAGES_TOKEN}`
    },
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    body: form
  })
    .then(res => res.text())
    .then(text => console.log(text))

  return {}
}

