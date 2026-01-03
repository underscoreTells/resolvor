<script lang="ts">
    interface Props {
        onSend: (message: string) => void;
        disabled?: boolean;
        placeholder?: string;
    }

    let {
        onSend,
        disabled = false,
        placeholder = "Type your message here...",
    }: Props = $props();

    let message = $state("");
    let textarea: HTMLTextAreaElement | undefined = $state();

    $effect(() => {
        message; // track message changes to trigger resize
        if (textarea) {
            textarea.style.height = "auto";
            textarea.style.height = `${textarea.scrollHeight}px`;
        }
    });

    function handleSubmit(event?: Event) {
        event?.preventDefault();

        const trimmed = message.trim();
        if (!trimmed || disabled) return;

        onSend(trimmed);
        message = "";
    }

    function handleKeyDown(event: KeyboardEvent) {
        if (event.key === "Enter" && !event.shiftKey) {
            event.preventDefault();
            handleSubmit();
        }
    }
</script>

<form onsubmit={handleSubmit} class="flex gap-2 max-w-3xl mx-auto w-full">
    <label for="message-input" class="sr-only"></label>
    <textarea
        id="message-input"
        bind:this={textarea}
        bind:value={message}
        {placeholder}
        {disabled}
        onkeydown={handleKeyDown}
        rows="1"
        class="input flex-1 resize-none font-[inherit] text-base overflow-y-hidden max-h-60"
    ></textarea>
    <button
        type="submit"
        disabled={disabled || !message.trim()}
        class="btn-primary px-6 disabled:opacity-50 disabled:cursor-not-allowed h-fit"
    >
        Send
    </button>
</form>
