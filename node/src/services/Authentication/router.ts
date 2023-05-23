import { auth } from "./Service"
import { Router } from "express"

const auth_router = Router()

auth_router.route("").post( auth )

export default auth_router
