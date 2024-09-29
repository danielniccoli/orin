<script>
    import { browser } from "$app/environment"
    import { invoke } from "@tauri-apps/api/core"

    /** @type {string} */
    let url1

    async function get() {
        if (browser) {
            let bytes = await invoke("get_document")
            debugger
            let b1 = new Blob([bytes], { type: "application/pdf" })
            url1 = URL.createObjectURL(b1)
        }
    }
</script>

<div>
    <label>
        Get docs:
        <button type="button" on:click={get}>Get docs</button>
    </label>
</div>

{#if url1}
    <embed src={url1} type="application/pdf" />
    <a href={url1}>download</a>
{/if}
