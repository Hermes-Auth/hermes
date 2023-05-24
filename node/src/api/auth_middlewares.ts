import { NextFunction, Request, Response } from "express";
import sql from "../db";

export async function auth_middleware( req: Request, res: Response, next: NextFunction ){
    try {
        const api_key = req.headers["x-hermes-auth"] ?? ""
        if(api_key==="") return res.status(401).send("Could not find api key. Make sure the x-hermes-auth header is set with your api key")
        await sql`select * from users where api_key=${api_key}`.then(row=>{
            if(row.length===0) return res.status(401).send({ message:"Invalid api key" })
            req.body.api_user = row[0].id
            next()
        })
    } catch (err) {
        console.log(`Error in auth middleware ${err}`)
        return res.status(500).json({ message:"Something went wrong. Please try again or contact support" })
    }
}

