import sql from "../../db";
import { PostgresError } from "postgres";
import { Request, Response } from "express";

export async function auth( req: Request<{}, {}, { email: string, auth_code: string }>, res: Response ){
    try {
        const { email, auth_code } = req.body
        await sql` select * from users where email=${email} `.then(async result=>{
            if(result.length===0){
                //Create user
                await sql` insert into users(email) values(${email}) returning *`
                //Sign auth token
                return res.status(201).json("token")
            }
            //Sign auth token
            return res.status(200).json("token")
        })
    } catch (err) {
        console.error(`Error while authenticating user:: ${err}`)
        return res.status(500).send()
    }
}
