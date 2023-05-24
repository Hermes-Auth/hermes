import express from "express"
import cors from "cors"
import dotenv from "dotenv"
dotenv.config()
import sql from "./db"
import auth_router from "./services/authentication/router"
import app_router from "./services/apps/router"
import router from "./api/router"
const app = express()

app.use(cors())
app.use(express.json())

app.use("/auth", auth_router)
app.use("/app", app_router)
app.use("/v1", router)

const PORT = process.env.PORT || 5000

app.listen(PORT, async()=>{
    try {
        const db_version = await sql`select version()`
        const version = db_version[0].version!
        console.log(`Connected to database ${version}`)
    } catch (err) {
        console.error("Failed to connect to database")
        console.error(err)
        process.exit(1)
    }
    console.log(`App listening on port ${PORT}`)
})
