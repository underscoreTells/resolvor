<script lang="ts">
    import type { Snippet } from "svelte";
    import type { HTMLButtonAttributes } from "svelte/elements";

    type Variant = "primary" | "ghost" | "outline";

    interface Props extends HTMLButtonAttributes {
        variant?: Variant;
        children: Snippet;
    }

    let {
        variant = "primary",
        children,
        class: className = "",
        ...rest
    }: Props = $props();

    const variantClasses: Record<Variant, string> = {
        primary: "btn-primary",
        ghost: "btn-ghost",
        outline: "btn-outline",
    };
</script>

<button class="{variantClasses[variant]} {className}" {...rest}>
    {@render children()}
</button>
