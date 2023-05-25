import { NextFunction, Request, Response } from "express"
import * as jwt from "jsonwebtoken"
const JWT_SECRET = process.env.JWT_SECRET as string

export function sign_token(user_id:string){
    return jwt.sign({ user: user_id }, JWT_SECRET, { expiresIn:"7d" })
}

export async function auth_user(req: Request, res: Response, next: NextFunction){
    try {
        const token = req.headers.authorization ?? ""
        if(token==="") return res.status(401).send()
        try {
            const { user } = jwt.verify(token, JWT_SECRET) as { user: string }
            req.body.user = user
            next()
        } catch (_) {
            return res.status(401).send()
        }
    } catch (err) {
        console.log(`Error while checking user auth state: ${err}`)
        return res.status(500).send()
    }

}

export function code_is_alive( code_ttl:number, generated_at:number ){
    try {
        const now = Date.now()
        let diff_in_min = Math.floor( (now - generated_at)/60000 )%60
        return code_ttl>diff_in_min
    } catch (err) {
        console.error(`Error while checking code lifespan: ${err}`)
        throw Error("CODE_ALIVE_ERR")
    }
}
