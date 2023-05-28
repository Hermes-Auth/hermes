import type { RequestHandler } from "./$types";
import sql from "$lib/sql"
import { verify_token } from "$lib/server";

export const GET = (async ({ request }) => {
    try {
        const token = request.headers.get("Authorization") ?? ""
        if (token === "") return new Response("", { status: 401 })
        const { user } = verify_token(token)
        if (user === "") return new Response("", { status: 401 })
        const apps = await sql`select * from users where id=${user} `
        return new Response(JSON.stringify(apps))
    } catch (err) {
        console.log(`Error while fetching apps ${err}`)
        return new Response("", { status: 500 })
    }
}) satisfies RequestHandler
