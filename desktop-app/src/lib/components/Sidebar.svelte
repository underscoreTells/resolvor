<script lang="ts">
    import Button from "./Button.svelte";

    interface Conversation {
        id: string;
        title: string;
    }

    interface Props {
        conversations?: Conversation[];
        onNewChat?: () => void;
        onSettings?: () => void;
    }

    let { conversations = [], onNewChat, onSettings }: Props = $props();
</script>

<aside class="sidebar">
    <Button variant="outline" class="mb-3 text-left" onclick={onNewChat}>
        + New Chat
    </Button>

    <nav class="flex-1 overflow-y-auto">
        {#each conversations as conv}
            <a href="/chat/{conv.id}" class="nav-link">
                {conv.title}
            </a>
        {/each}
    </nav>

    <div class="border-t border-border pt-2">
        <Button variant="ghost" class="w-full text-left" onclick={onSettings}>
            Settings
        </Button>
    </div>
</aside>
