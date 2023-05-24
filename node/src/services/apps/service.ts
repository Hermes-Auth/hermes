import { Request, Response } from "express";
import sql from "../../db";

export async function create_app(req: Request<{}, {}, { user: string, name: string, active?: boolean, ttl: string }>, res: Response){
    try {
        const {user, name, active, ttl} = req.body
        await sql`select * from apps where name=${name} and owner=${user}`.then(async result=>{
            if(result.length!==0) return res.status(409).send()
            await sql`insert into apps(name, owner, active, default_codes_ttl) values(${name}, ${user}, ${active ?? true}, ${ttl})`.then(_=>{
                return res.status(201).send()
            })
        })
    } catch (err) {
        console.error(`Error while creating app: ${err}`)
        return res.status(500).send()
    }
}

export async function get_apps( req: Request<{}, {}, { user: string }>, res: Response){
    try {
        const data = await sql`select * from apps where owner=${req.body.user}`
        return res.status(200).json(data)
    } catch (err) {
        console.error(`Error while fetching user ${req.body.user} apps: ${err}`)
        return res.status(500).send()
    }
}

export async function deactivate( req: Request<{ app: string }, {}, { user: string, current_state: boolean }>, res: Response ){
    try {
        await sql`select * from apps where owner=${req.body.user} and id=${req.params.app}`.then(async result=>{
            if(result.length===0) return res.status(404).send()
            await sql`update apps set active=${!req.body.current_state} where id=${req.params.app}`
            return res.status(200).send()
        })
    } catch (err) {
        console.error(`Error while deactivating app: ${err}`)
        return res.status(500).send()
    }
}
