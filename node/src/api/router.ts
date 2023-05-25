import { request_code, verify } from "./services";
import { Router } from "express";
import { auth_middleware } from "./auth_middlewares";

const router = Router()

router.route("/request").post( auth_middleware, request_code )
router.route("/verify").post( auth_middleware, verify )

export default router
