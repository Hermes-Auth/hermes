import { Request, Response } from "express";
import { z } from "zod"
import bcrypt from "bcrypt"
import sql from "../db";
import { createTransport } from "nodemailer"
const MAX = 999999
const MIN = 100000
const SALT_ROUNDS = 10

export async function request_code(req: Request<{}, {}, { api_user: string }>, res: Response) {
    try {
        const schema = z.object({
            target: z.string().email(),
            app_id: z.string().uuid(),
            ttl: z.number().int().positive().optional(),
            subject: z.string().optional()
        })
        const validation_result = schema.safeParse(req.body)
        const message = "Check your request body. Some fields might be missing or have incorrect types"
        if (!validation_result.success) return res.status(400).json({ message })
        const { data } = validation_result
        await sql`select * from apps where id=${data.app_id} and owner=${req.body.api_user}`.then(async row => {
            if (row.length === 0) return res.status(400).send({ message: "No app found for the provided app id" })
            const app = row[0]
            if (!app.active) return res.status(400).send({ message: "The specified app is not active" })
            const { SMTP_PASSWORD } = process.env
            const code = Math.floor(Math.random() * (MAX - MIN + 1)) + MIN
            const transporter = createTransport({
                service: "gmail",
                auth: {
                    user: "noreply.auth.hermes@gmail.com",
                    pass: SMTP_PASSWORD as string
                }
            })
            const mailOptions = {
                from: "noreply.auth.hermes.@gmail.com",
                to: data.target,
                subject: data.subject ?? `Your ${app.name} authentication code`,
                text: code.toString()
            }
            transporter.sendMail(mailOptions, async (err, _info) => {
                if (err) {
                    console.log(`Error while sending auth code: ${err}`)
                    return res.status(500).send()
                }
                const hash = bcrypt.hashSync(code.toString(), SALT_ROUNDS)
                const generated_at = Date.now()
                await sql`
                insert into codes(code, ttl, generated_at, generated_for, app )
                values(${hash}, ${data.ttl ?? app.codes_ttl}, ${generated_at}, ${data.target}, ${data.app_id})
                `
                return res.status(200).send()
            })

        })
    } catch (err) {
        console.log(`Error while sending auth code: ${err}`)
        return res.status(500).send()
    }
}

export async function verify(req: Request, res: Response) {
    try {
        const schema = z.object({

        })
    } catch (err) {
        console.error(`Error while verifying auth code ${err}`)
        return res.status(500).send()
    }
}
