import express from "express"
import cors from "cors"
import dotenv from "dotenv"
import postgres from "postgres"
dotenv.config()

const app = express()

app.use(cors())
app.use(express.json())

const PORT = process.env.PORT || 5000

const sql = postgres(process.env.DATABASE_URL!, { ssl:"require" })

async function getPgVersion() {
  const result = await sql`select version()`;
  console.log(result);
}

getPgVersion();

app.listen(PORT, ()=>{
    console.log(`App listening on port ${PORT}`)
})
