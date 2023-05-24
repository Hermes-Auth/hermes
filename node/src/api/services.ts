import { Request, Response } from "express";
import bcrypt from "bcrypt"
import sql from "../db";
import { createTransport } from "nodemailer"
const MAX = 999999
const MIN = 100000
const SALT_ROUNDS = 10

export async function send_code(req: Request<{}, {}, { app_id:string, target:string, subject?: string, app_name: string, ttl: string } >, res: Response){
    try {
        const { SMTP_PASSWORD } = process.env
        const code = Math.floor(Math.random() * ( MAX - MIN + 1 ) ) + MIN
        const transporter = createTransport({
            service:"gmail",
            auth:{
                user:"noreply.auth.hermes@gmail.com",
                pass:SMTP_PASSWORD as string
            }
        })
        const mailOptions = {
            from :"noreply.auth.hermes.@gmail.com",
            to:req.body.target,
            subject: req.body.subject ?? `Your ${req.body.app_name} authentication code`,
            text:code.toString()
        }
        transporter.sendMail(mailOptions, async(err, _info)=>{
            if(err){
                console.log(`Error while sending auth code: ${err}`)
                return res.status(500).send()
            }
            const hash = bcrypt.hashSync(code.toString(), SALT_ROUNDS)
            const generated_at = Date.now()
            await sql`
                insert into codes(code, ttl, generated_at, generated_for, app )
                values(${hash}, ${req.body.ttl}, ${generated_at}, ${req.body.target}, ${req.body.app_id})
            `
            return res.status(200).send()
        })
    } catch (err) {
        console.log(`Error while sending auth code: ${err}`)
        return res.status(500).send()
    }
}
