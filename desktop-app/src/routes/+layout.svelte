<script lang="ts">
    import "../app.css";

    // Could hold conversation list, user info, etc.
    let conversations = [
        { id: "1", title: "Help with Rust code" },
        { id: "2", title: "Explain SvelteKit" },
        { id: "3", title: "Debug my app" },
    ];
</script>

<div class="flex h-screen w-screen overflow-hidden">
    <!-- Sidebar -->
    <aside class="w-[260px] shrink-0 flex flex-col p-2 bg-[#202123] text-white">
        <button
            class="p-3 mb-3 text-left text-white bg-transparent border border-gray-600 rounded-md cursor-pointer transition-colors duration-200 hover:bg-[#2a2b32]"
        >
            + New Chat
        </button>

        <nav class="flex-1 overflow-y-auto">
            {#each conversations as conv}
                <a
                    href="/chat/{conv.id}"
                    class="block p-3 mb-1 text-[#ececec] text-sm no-underline rounded-md truncate transition-colors duration-200 hover:bg-[#2a2b32]"
                >
                    {conv.title}
                </a>
            {/each}
        </nav>

        <div class="border-t border-gray-600 pt-2">
            <button
                class="w-full p-3 text-left text-white bg-transparent border-none rounded-md cursor-pointer transition-colors duration-200 hover:bg-[#2a2b32]"
            >
                Settings
            </button>
        </div>
    </aside>

    <!-- Main content area -->
    <main class="flex-1 flex flex-col relative bg-[#343541] text-white">
        <slot />
        <!-- Your +page.svelte content goes here -->
    </main>
</div>
