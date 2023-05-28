export function code_is_alive( code_ttl:number, generated_at:number ){
    try {
        const now = Date.now()
        let diff_in_min = Math.floor( (now - generated_at)/60000 )%60
        return code_ttl>diff_in_min
    } catch (err) {
        console.error(`Error while checking code lifespan: ${err}`)
        throw Error("CODE_ALIVE_ERR")
    }
}

