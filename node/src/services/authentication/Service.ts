import sql from "../../db";
import { Request, Response } from "express";
import { sign_token } from "../../utils";

export async function auth( req: Request<{}, {}, { email: string, auth_code: string }>, res: Response ){
    try {
        const { email, auth_code } = req.body
        await sql` select * from users where email=${email} `.then(async result=>{
            if(result.length===0){
                const new_user = await sql` insert into users(email) values(${email}) returning *`
                const token = sign_token(new_user[0].id)
                return res.status(201).json(token)
            }
            const token = sign_token(result[0].id)
            return res.status(200).json(token)
        })
    } catch (err) {
        console.error(`Error while authenticating user:: ${err}`)
        return res.status(500).send()
    }
}

export async function update_api_key( req: Request<{}, {}, { user: string }>, res: Response ){
    try {
        const { user } = req.body
        await sql`update users set api_key = gen_random_uuid()::text where email=${user}`
        return res.status(200).send()
    } catch (err) {
        console.error(`Error while updating user's api key ${err}`)
        return res.status(500).send()
    }
}
