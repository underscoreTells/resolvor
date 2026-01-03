<script lang="ts">
    import * as types from "../../types/index";

    let { toolUse }: { toolUse: types.ToolUseContent } = $props();

    const statusConfig = $derived(
        types.toolCallStatusStyles[toolUse.tool_call.status],
    );

    const formattedArgs = $derived(
        JSON.stringify(toolUse.tool_call.arguments, null, 2),
    );

    let isExpanded = $state(false);
</script>
