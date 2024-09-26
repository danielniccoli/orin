<script>
    import { browser } from "$app/environment"
    import { open } from "@tauri-apps/plugin-dialog"
    import { invoke, convertFileSrc } from "@tauri-apps/api/core"

    async function add() {
        if (browser) {
            let p = await open({
                multiple: false,
                directory: false,
                filters: [{ name: "PDF Documents", extensions: ["pdf"] }],
            })

            await invoke("store_file", { filePath: p })
        }
    }
</script>

<div>
    <label for="avatar">Add one document:</label>
    <button type="button" on:click={add}>Add one document</button>
</div>
