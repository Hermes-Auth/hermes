import { writable } from "svelte/store"

export const current_tab = writable("auth")

export const user_is_logged_in = writable(false)
