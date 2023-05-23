import { auth, update_api_key } from "./Service"
import { auth_user } from "../../utils"
import { Router } from "express"

const auth_router = Router()

auth_router.route("").post( auth )
auth_router.route("/update_key").post( auth_user, update_api_key )

export default auth_router
