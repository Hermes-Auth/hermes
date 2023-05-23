import postgres from "postgres";

const db_url = process.env.DATABASE_URL!

const sql = postgres(db_url, { ssl:"require" })

export default sql
