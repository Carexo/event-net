<script lang="ts">
    import type {PageProps} from './$types';
    import {onDestroy} from 'svelte';
    import {Heading, Img, Badge, Button, Toast} from "flowbite-svelte";
    import {CalendarMonthSolid, TagSolid, UserAddSolid, BellOutline} from "flowbite-svelte-icons";

    import {selectedUser} from "$lib/stores/userStore";
    import {getApiUrl} from "$lib/utils/api";
    import KeywordsList from "$lib/components/KeywordsList.svelte";
    import ToastNotification from "$lib/components/ToastNotification.svelte";

    let toast: ToastNotification;

    let {data}: PageProps = $props();

    const handleSignUp = async () => {
        if (!$selectedUser) {
            toast.showToast("Please select a user first", "red");
            return;
        }

        if (data.event) {
            try {

                const response = await fetch(getApiUrl(`/events/${data.event.id}/attendees/${$selectedUser}`), {
                    method: 'PUT',
                });

                if (!response.ok) {
                    const errorData = await response.json();
                    throw new Error(errorData.message || 'Failed to sign up for event');
                }

                const responseData = await response.json();

                // Success message
                toast.showToast(responseData.message, "green");
            } catch (error) {
                // Error message
                const toastMessage = error instanceof Error ? error.message : 'An error occurred';
                toast.showToast(toastMessage, "red");
            }
        } else {
            toast.showToast("No event data available", "red");
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
                        <KeywordsList keywords={data.event.keywords} showIcon={false}/>
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

<ToastNotification bind:this={toast}/>