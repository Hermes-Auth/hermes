import { NextFunction, Request, Response } from "express";
import { z } from "zod"
import sql from "../db";

export async function auth_middleware( req: Request, res: Response, next: NextFunction ){
    try {
        const api_key = req.headers["x-hermes-auth"] ?? ""
        if(api_key==="") return res.status(401).send("Could not find api key. Make sure the x-hermes-auth header is set with your api key")
        const schema = z.object({
            target: z.string().email(),
            app_id: z.string(),
            ttl: z.number().int().positive().optional(),
            subject: z.string().optional()
        })
        const validation_result = schema.safeParse(req.body)
        const message = "Check your request body. Some fields might be missing or have incorrect types"
        if(!validation_result.success) return res.status(400).json({ message })
        const { data } = validation_result
        await sql`select * from users where api_key=${api_key}`.then(async result=>{
            if(result.length===0) return res.status(400).send({ message:"Invalid api key" })
            const api_user = result[0]
            await sql`select * from apps where id=${data.app_id} and owner=${api_user.id}`.then(app_row=>{
                if(app_row.length===0) return res.status(400).send({ message:"No app found for the provided app id" })
                const app = app_row[0]
                if(!app.active) return res.status(400).send({ message:"The specified app is not active." })
                if(!data.ttl) req.body.ttl = app.codes_ttl
                req.body.app_name = app.name
                next()
            })
        })
    } catch (err) {
        console.log(`Error in auth middleware ${err}`)
        return res.status(500).json({ message:"Something went wrong. Please try again or contact support" })
    }
}
