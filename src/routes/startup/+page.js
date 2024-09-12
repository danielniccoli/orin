import { invoke } from "@tauri-apps/api/core"
import { browser } from "$app/environment"

/** @type {import('./$types').PageLoad} */
export async function load() {
    /** @type {string[]} */
    let errorMessages = []

    if (browser) {
        try {
            await invoke("run_migrations")
        } catch (err) {
            errorMessages.push(String(err))
        }
    }

    return { errorMessages: errorMessages }
}
