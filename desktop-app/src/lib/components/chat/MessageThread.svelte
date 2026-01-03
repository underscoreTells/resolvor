<script lang="ts">
    import * as types from "../../types/index";
    import Message from "./Message.svelte";

    let { messages }: { messages: types.Message[] } = $props();
</script>

<div class="flex-1 flex flex-col min-w-0">
    <div class="flex-1 overflow-y-auto p-4">
        {#each messages as message (message.id)}
            <Message {message} />
        {/each}
    </div>
</div>
