import { request_code, verify } from "./services";
import { Router } from "express";
import { auth_middleware } from "./auth_middlewares";

const router = Router()

router.route("/request").post( auth_middleware, request_code )

export default router
