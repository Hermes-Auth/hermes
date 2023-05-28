import type { RequestHandler } from "./$types";
import { createTransport } from "nodemailer"
import { GMAIL_API_KEY } from "$env/static/private"

export const POST = (async ({ request }) => {
    try {
        const body = await request.json() as { email: string }
        const email = body.email
        const code = Math.floor(Math.random() * 1000000)

        const transporter = createTransport({
            service: "gmail",
            auth: {
                user: "noreply.auth.hermes@gmail.com",
                pass: GMAIL_API_KEY
            }
        })
        const mailOptions = {
            from: "noreply.auth.hermes@gmail.com",
            to: email,
            subject: "Hermes Test",
            text: `Ton code chien ${code}`
        }
        transporter.sendMail(mailOptions, async (err, _) => {
            if (err) {
                console.log(err)
                return new Response("", { status: 500 })
            }
        })
        return new Response("")
    } catch (err) {
        console.log(err)
        return new Response("", { status: 500 })
    }
}) satisfies RequestHandler
