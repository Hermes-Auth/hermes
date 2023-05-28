import type { RequestHandler } from "./$types";
import { createTransport } from "nodemailer"
import { GMAIL_API_KEY } from "$env/static/private"
import sql from "../../../../lib/db/db"

export const POST = (async ({ request }) => {
    try {
        const body = await request.json() as { email: string }
        const email = body.email
        const code = Math.floor(Math.random() * 1000000)

        const transporter = createTransport({
            service: "gmail",
            secure:true,
            auth: {
                user: "noreply.auth.hermes@gmail.com",
                pass: GMAIL_API_KEY
            }
        })
        const mailOptions = {
            from: {
                name:"Hermes",
                address:"noreply.auth.hermes@gmail.com"
            },
            to: email,
            subject: "Hermes Test",
            text: `Ton code chien ${code}`
        }
        await new Promise(async (resolve, reject)=>{
            await sql`
                insert into codes(code, ttl, generated_for, generated_at)
                values (${code}, ${5}, ${email}, ${Date.now()});
            `
            transporter.sendMail(mailOptions).then((_)=>{
                resolve("Success")
            }).catch(err=>{
                console.log(`Error while sending code ${err}`)
                reject(err)
            })
        })
        return new Response("")
    } catch (err) {
        console.log(err)
        return new Response("", { status: 500 })
    }
}) satisfies RequestHandler

