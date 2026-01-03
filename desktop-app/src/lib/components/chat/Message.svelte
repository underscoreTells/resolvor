<script lang="ts">
    import * as types from "../../types/index";
    import TextContent from "../content/TextContent.svelte";
    import ToolUseContent from "../content/ToolUseContent.svelte";

    let { message }: { message: types.Message } = $props();

    const isUser = $derived(message.role === "user");
    const isSystem = $derived(message.role === "system");
    const isError = $derived(message.role === "error");

    const bubbleClasses = $derived.by(() => {
        if (isSystem) return "px-4 py-3 bg-surface-hover text-content-muted";
        if (isError) return "px-4 py-3 bg-red-900/50 text-red-300";
        return ""; // assistant - no background, no padding
    });
</script>
