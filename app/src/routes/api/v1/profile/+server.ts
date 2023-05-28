import type { RequestHandler } from "./$types";
import sql from "$lib/sql"
import { verify_token } from "$lib/server";

export const GET = (async ({ request })=>{
    try {
        const token = request.headers.get("Authorization") ?? ""
        if (token === "") return new Response("", { status: 401 })
        const { user } = verify_token(token)
        if (user === "") return new Response("", { status: 401 })
        const user_info = await sql`select * from users where id=${user} `
        return new Response(JSON.stringify(user_info.rows[0]))
    } catch (err){
        console.log(`Error while fetching user stats ${err}`)
        return new Response("", { status:500 })
    }
}) satisfies RequestHandler

export const PUT = (async ({ request })=>{
    try {
        const token = request.headers.get("Authorization") ?? ""
        if (token === "") return new Response("", { status: 401 })
        const { user } = verify_token(token)
        if (user === "") return new Response("", { status: 401 })
        const update_result = await sql`update users set api_key=gen_random_uuid()::text where id=${user} returning api_key`
        return new Response(JSON.stringify({ key:update_result.rows[0].api_key }))
    } catch (err) {
        console.log(`Error while resetting user api key: ${err}`)
        return new Response("", { status:500 })
    }
}) satisfies RequestHandler
