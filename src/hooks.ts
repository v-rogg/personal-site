import { defaultLocale, locales } from "$lib/_i18n";
import type { Handle } from "@sveltejs/kit";

const routeRegex = new RegExp(/^\/[^.]*([?#].*)?$/);
import pg from "pg";
import { faunaDBStore, postgresDB } from "./stores";
import faunadb from "faunadb";


export const handle: Handle = async ({ event, resolve }) => {
  const { url, request } = event;
  const { pathname } = url;

  const { Pool } = pg;

  const pool = new Pool({
    connectionString: `${import.meta.env.VITE_COCKROACH_DB_STRING}`,
    application_name: "$ placeholder_postgres"
  })

  const cock = await pool.connect();

  postgresDB.set(cock);

  const { Client } = faunadb;
  const client = new Client({
    secret: `${import.meta.env.VITE_FAUNA_SECRET}`,
    domain: "db.eu.fauna.com",
    scheme: "https",
    port: 443
  });

  faunaDBStore.set(client);

  // If this request is a route request
  if (
    !event.url.pathname.includes("api") &&
    !event.url.pathname.includes(".css") &&
    routeRegex.test(pathname)
  ) {
    // Get defined locales
    const supportedLocales = locales.get();

    // Try to get locale from `pathname`.
    let locale = supportedLocales.find(
      (l) => `${l}`.toLowerCase() === `${pathname.match(/[^/]+?(?=\/|$)/)}`.toLowerCase()
    );

    // If route locale is not supported
    if (!locale) {
      // Get user preferred locale
      locale = `${`${request.headers.get("accept-language")}`.match(
        /[a-zA-Z]+?(?=[-_,;])/
      )}`.toLowerCase();

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

  return resolve(event);
};
