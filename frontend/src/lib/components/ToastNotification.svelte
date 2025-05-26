<script lang="ts">
    import { onDestroy } from 'svelte';
    import { Toast } from "flowbite-svelte";
    import { BellOutline } from "flowbite-svelte-icons";

    let {position = "bottom-5 right-5"} = $props();

    let toastStatus = $state(false);
    let toastMessage = $state("");
    let toastColor: any = $state("red");
    let toastTimer: ReturnType<typeof setTimeout> | null = null;

    $effect(() => {
        if (toastStatus) {
            if (toastTimer) clearTimeout(toastTimer);

            toastTimer = setTimeout(() => {
                toastStatus = false;
                toastTimer = null;
            }, 5000);
        }
    });

    onDestroy(() => {
        if (toastTimer) clearTimeout(toastTimer);
    });

    // Function to show toast - exported for other components to use
    export function showToast(message: string, color: "red" | "green" | "blue" | "yellow" = "red") {
        toastMessage = message;
        toastColor = color;
        toastStatus = true;
    }
</script>

<Toast
        bind:toastStatus
        color={toastColor}
        class="fixed {position} z-50 max-w-xs w-full"
>
    {#snippet icon()}
        <BellOutline class="h-6 w-6" />
        <span class="sr-only">Bell icon</span>
    {/snippet}
    {toastMessage}
</Toast>