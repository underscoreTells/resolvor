<script lang="ts">
    import InputBox from "$lib/components/InputBox.svelte";

    // Placeholder for messages - you'd likely want to manage this with a store later
    let messages: Array<{
        id: string;
        role: "user" | "assistant";
        content: string;
    }> = $state([]);

    function handleSendMessage(content: string) {
        // Add user message
        messages.push({
            id: crypto.randomUUID(),
            role: "user",
            content,
        });

        // TODO: Send to backend/API and handle assistant response
        console.log("Message sent:", content);
    }
</script>

<div class="flex flex-col h-full">
    <!-- Messages Area -->
    <div class="flex-1 overflow-y-auto p-4">
        {#if messages.length === 0}
            <div
                class="flex items-center justify-center h-full text-content-muted"
            >
                <h1 class="text-2xl font-medium">How can I help you today?</h1>
            </div>
        {:else}
            <div class="max-w-3xl mx-auto w-full">
                {#each messages as message (message.id)}
                    <div
                        class="mb-4 p-4 rounded-lg {message.role === 'user'
                            ? 'bg-surface-input'
                            : 'bg-surface-dark'}"
                    >
                        <p class="whitespace-pre-wrap">{message.content}</p>
                    </div>
                {/each}
            </div>
        {/if}
    </div>

    <!-- Input Area (pinned to bottom) -->
    <div class="p-4 border-t border-border bg-surface">
        <InputBox onSend={handleSendMessage} />
    </div>
</div>
