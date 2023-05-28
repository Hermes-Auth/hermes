import { JWT_SECRET } from "$env/static/private"
import * as jwt from "jsonwebtoken"

export function sign_token(user_id:string){
   return jwt.sign({ user:user_id }, JWT_SECRET, { expiresIn:"20 days" }) 
}
