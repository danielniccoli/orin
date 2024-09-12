<!--
This route handles critical initialization tasks (e.g., database migrations) that must occur
before the app starts. It must be strictly restricted from user access to prevent accidental
triggering, ensuring stability. Upon success, it navigates the user to the home page and replaces
the current history entry to avoid accidental navigation back, as multiple executions could cause
issues.
-->
<script>
    import { browser } from "$app/environment"
    import { goto } from "$app/navigation"

    /** @type {import('./$types').PageData} */
    export let data

    if (data.errorMessages.length === 0 && browser) {
        goto("/", { replaceState: true })
    }
</script>

<p>Hello at /startup</p>

{#if data.errorMessages}
    <p style="color: red;">{data.errorMessages}</p>
{/if}
