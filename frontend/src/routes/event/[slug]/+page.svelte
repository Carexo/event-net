<script lang="ts">
    import type {PageProps} from './$types';
    import { onDestroy } from 'svelte';
    import {Heading, Img, Badge, Button, Toast} from "flowbite-svelte";
    import {CalendarMonthSolid, TagSolid, UserAddSolid, ExclamationCircleSolid} from "flowbite-svelte-icons";

    import {selectedUser} from "$lib/stores/userStore";

    let toastStatus = $state(false);
    let toastTimer: ReturnType<typeof setTimeout> | null = null;

    let {data}: PageProps = $props();

    $effect(() => {
        if (toastStatus) {
            if (toastTimer) clearTimeout(toastTimer);

            toastTimer = setTimeout(() => {
                toastStatus = false;
                toastTimer = null;
            }, 5000);
        }
    })

    onDestroy(() => {
        if (toastTimer) clearTimeout(toastTimer);
    });

    const handleSignUp = () => {
        if (!$selectedUser) {
            toastStatus = true;
            return;
        }

        if (data.event) {
            // Here you would typically handle the sign-up logic, e.g., API call
            alert(`Signed up for event: ${data.event.name}`);
        } else {
            alert("No event data available.");
        }
    };
</script>

<section class="flex gap-5 justify-between w-full">
    {#if data.error}
        <div class="text-red-500 p-8 text-center">
            <p>Error: {data.error}</p>
        </div>
    {:else if data?.event}
        <div class="w-full overflow-hidden">
            <div class="w-full h-80 overflow-hidden">
                <Img
                        src={`/events/${data.event.id % 10}.png`}
                        alt={data.event.name}
                        class="w-full h-full object-cover"
                />
            </div>

            <div class="container mx-auto px-4 py-8">
                <Heading tag="h1" class="text-4xl font-bold mb-8 text-center">{data.event.name}</Heading>

                <div class="max-w-3xl mx-auto space-y-6 mb-10">
                    <div class="flex items-center gap-3">
                        <CalendarMonthSolid class="w-6 h-6 text-blue-600"/>
                        <span class="text-lg text-gray-700 dark:text-gray-300">
                            {new Date(data.event.start_datetime).toLocaleString()}
                        </span>
                    </div>

                    <div class="flex items-center gap-3">
                        <TagSolid class="w-6 h-6 text-blue-600 mt-1"/>
                        <div class="flex flex-wrap gap-2">
                            {#each data.event.keywords as keyword}
                                <Badge color="blue" size="large">{keyword.trim()}</Badge>
                            {/each}
                        </div>
                    </div>
                </div>

                <div class="flex justify-center mt-10 self-end">
                    <Button size="xl" color="blue" class="px-12 py-3 text-lg" onclick={handleSignUp}>
                        <UserAddSolid class="mr-3 h-6 w-6"/>
                        Sign up for this event
                    </Button>
                </div>
            </div>
        </div>
    {/if}

</section>
<Toast
        bind:toastStatus
        color="red">
    {#snippet icon()}
        <ExclamationCircleSolid class="h-5 w-5"/>
        <span class="sr-only">Warning icon</span>
    {/snippet}
    Choose user first
</Toast>