<script lang="ts">
    import SvelteMarkdown from "svelte-markdown";
    import type { TextContent } from "../../types/generated/messages";

    let { textContent }: { textContent: TextContent } = $props();
</script>

<div class="prose prose-invert prose-sm max-w-none">
    <SvelteMarkdown source={textContent.text} />
</div>
