<script lang="ts">
    import "../app.css";
    import Sidebar from "$lib/components/Sidebar.svelte";

    let conversations = [
        { id: "1", title: "Help with Rust code" },
        { id: "2", title: "Explain SvelteKit" },
        { id: "3", title: "Debug my app" },
    ];

    function handleNewChat() {
        // TODO: Implement new chat creation
        console.log("New chat clicked");
    }

    function handleSettings() {
        // TODO: Implement settings modal/page
        console.log("Settings clicked");
    }
</script>

<div class="flex h-screen w-screen overflow-hidden">
    <Sidebar
        {conversations}
        onNewChat={handleNewChat}
        onSettings={handleSettings}
    />

    <main class="main-content">
        <slot />
    </main>
</div>
