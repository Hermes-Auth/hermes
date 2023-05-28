import type { RequestHandler } from "@sveltejs/kit";
import sql from "$lib/sql"
import { code_is_alive } from "$lib/utils";

export const POST = (async ({ request }) => {
    try {
        const { code, user } = await request.json() as { code: string, user: string }
        const code_row = await sql`
        select * from codes where code=${code} and generated_for=${user} and used=${false}
        `
        if (code_row.rows.length === 0) {
            return new Response("", { status: 400 })
        }
        const potential_code = code_row.rows[0]
        if (code_is_alive(Number.parseInt(potential_code.ttl), Number.parseInt(potential_code.generated_at))) {
            await sql`
            update codes set used=${true} where id=${potential_code.id}
            `
            return new Response("")
        }
        return new Response("", { status: 400 })
    } catch (err) {
        return new Response("", { status: 500 })
    }


}) satisfies RequestHandler
