import { create_app, get_apps, deactivate } from "./service"
import { Router } from "express"
import { auth_user } from "../../utils"
const app_router = Router()

app_router.route("").post( auth_user, create_app )
app_router.route("").get( auth_user,  get_apps )
app_router.route("/:app/deactivate").post( auth_user, deactivate )

export default app_router
