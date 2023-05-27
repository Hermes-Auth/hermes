import { Request, Response } from "express";
import { z } from "zod"
import sql from "../db";
import { createTransport } from "nodemailer"
import { markAsUntransferable } from "worker_threads";
import { code_is_alive } from "../utils";
const MAX = 999999
const MIN = 100000

export async function request_code(req: Request<{}, {}, { api_user: string } & Record<string,string>>, res: Response) {
    try {
        const schema = z.object({
            target: z.string().email(),
            app_id: z.string().uuid(),
            ttl: z.number().int().positive().optional(),
            subject: z.string().optional()
        })
        console.log(req.body)
        const validation_result = schema.safeParse(req.body)
        const message = "Check your request body. Some fields might be missing or have incorrect types"
        if (!validation_result.success) return res.status(400).json({ message })
        const { data } = validation_result
        await sql`select * from apps where id=${data.app_id} and owner=${req.body.api_user}`.then(async row => {
            if (row.length === 0) return res.status(400).send({ message: "No app found for the provided app id" })
            const app = row[0]
            if (!app.active) return res.status(400).send({ message: "The specified app is not active" })
            const { SMTP_PASSWORD } = process.env
            const code = String(Math.floor(Math.random() * (MAX - MIN + 1)) + MIN) 
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
                const generated_at = Date.now()
                await sql`
                    insert into codes(code, ttl, generated_at, generated_for, app )
                    values(${code}, ${data.ttl ?? app.codes_ttl}, ${generated_at}, ${data.target}, ${data.app_id})
                `
                return res.status(200).send()
            })

        })
    } catch (err) {
        console.log(`Error while sending auth code: ${err}`)
        return res.status(500).send()
    }
}

export async function verify(req: Request<{}, {}, { api_user:string }>, res: Response) {
    try {
        const schema = z.object({
            code: z.string(),
            user: z.string().email(),
            app_id: z.string().uuid()
        })
        const validation_result = schema.safeParse(req.body)
        const message = "Check your request body. Some fields might be missing or have incorrect types"
        if(!validation_result.success) return res.status(400).send({ message })
        const { data } = validation_result
        await sql`select * from apps where id=${data.app_id} and owner=${req.body.api_user}`.then(async row=>{
            if (row.length === 0) return res.status(400).send({ message: "No app found for the provided app id" })
            const app = row[0]
            if (!app.active) return res.status(400).send({ message: "The specified app is not active" })
            await sql`select * from codes where code=${data.code} and app=${data.app_id} and generated_for=${data.user} and used=${false}`.then(async row=>{
                if(row.length===0) return res.status(400).send({ message:"Invalid code" })
                const code = row[0]
                let code_alive = code_is_alive(Number.parseInt(code.ttl), Number.parseInt(code.generated_at))
                if(!code_alive) return res.status(400).send({ message:"Invalid code 2" })
                await sql`update codes set used=${true} where id=${code.id} `
                return res.status(200).send()
            })
        })
    } catch (err) {
        console.error(`Error while verifying auth code ${err}`)
        return res.status(500).send()
    }
}
