<script>
    import { browser } from "$app/environment"
    import { invoke } from "@tauri-apps/api/core"

    async function get() {
        if (browser) {
            await invoke("get_documents")
        }
    }
</script>

<div>
    <label>
        Get docs:
        <button type="button" on:click={get}>Get docs</button>
    </label>
</div>
