<script lang="ts">
    import type {PageProps} from './$types';
    import {Heading, Img, Button, Modal} from "flowbite-svelte";
    import {EditSolid, CalendarMonthSolid, TagSolid, UserAddSolid, BellOutline, UserRemoveSolid, TrashBinSolid} from "flowbite-svelte-icons";

    import {selectedUser} from "$lib/stores/userStore";
    import {getApiUrl} from "$lib/utils/api";
    import KeywordsList from "$lib/components/KeywordsList.svelte";
    import ToastNotification from "$lib/components/ToastNotification.svelte";
    import {goto} from "$app/navigation";

    let toast: ToastNotification;

    let deleteModalOpen = $state(false);
    let isDeleting = $state(false);

    let {data}: PageProps = $props();
    let isRegistered = $state(false);

    const handleDeleteEvent = async () => {
        if (!data.event) {
            toast.showToast("No event data available", "red");
            return;
        }

        isDeleting = true;

        try {
            const response = await fetch(getApiUrl(`/event/${data.event.id}`), {
                method: 'DELETE',
            });

            if (!response.ok) {
                const errorData = await response.json();
                throw new Error(errorData.message || 'Failed to delete event');
            }

            const responseData = await response.json();
            toast.showToast(responseData.message || "Event deleted successfully", "green");

            // Redirect to events page after successful deletion
            setTimeout(() => {
                goto('/events');
            }, 1500);

        } catch (error) {
            const toastMessage = error instanceof Error ? error.message : 'An error occurred';
            toast.showToast(toastMessage, "red");
        } finally {
            isDeleting = false;
            deleteModalOpen = false;
        }
    };

    const checkRegistrationStatus = async () => {
        if (!$selectedUser || !data.event) return;

        try {
            const response = await fetch(getApiUrl(`/events/${data.event.id}/attendees/${$selectedUser}`), {
                method: 'GET',
            });
            if (!response.ok) {
                const errorData = await response.json();
                throw new Error(errorData.message || 'Failed to check registration status');
            }

            const responseData = await response.json();
            isRegistered = responseData.data;

        } catch (error) {
            console.error("Failed to check registration status:", error);
        }
    };

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
                isRegistered = true;
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

    const handleUnregister = async () => {
        if (!$selectedUser) {
            toast.showToast("Please select a user first", "red");
            return;
        }

        if (data.event) {
            try {
                const response = await fetch(getApiUrl(`/events/${data.event.id}/attendees/${$selectedUser}`), {
                    method: 'DELETE',
                });

                if (!response.ok) {
                    const errorData = await response.json();
                    throw new Error(errorData.message || 'Failed to unregister from event');
                }

                const responseData = await response.json();
                isRegistered = false;
                toast.showToast(responseData.message, "green");
            } catch (error) {
                const toastMessage = error instanceof Error ? error.message : 'An error occurred';
                toast.showToast(toastMessage, "red");
            }
        } else {
            toast.showToast("No event data available", "red");
        }
    };

    // Check registration status on mount and when selected user changes
    $effect(() => {
        if ($selectedUser && data.event) {
            checkRegistrationStatus();
        }
    });
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
                            {data.event.date.toLocaleString()}
                        </span>
                    </div>

                    <div class="flex items-center gap-3">
                        <TagSolid class="w-6 h-6 text-blue-600 mt-1"/>
                        <KeywordsList keywords={data.event.keywords} showIcon={false}/>
                    </div>
                </div>

                <div class="flex justify-center mt-10 self-end space-x-4">
                    {#if isRegistered}
                        <Button size="xl" color="red" class="px-12 py-3 text-lg" onclick={handleUnregister}>
                            <UserRemoveSolid class="mr-3 h-6 w-6"/>
                            Unregister from this event
                        </Button>
                    {:else}
                        <Button size="xl" color="blue" class="px-12 py-3 text-lg" onclick={handleSignUp}>
                            <UserAddSolid class="mr-3 h-6 w-6"/>
                            Sign up for this event
                        </Button>
                    {/if}
                    <Button href={`/event/${data.event.id}/edit`} size="xl" color="gray" class="px-12 py-3 text-lg">
                        <EditSolid class="mr-3 h-6 w-6"/>
                        Edit Event
                    </Button>
                    <Button size="xl" color="red" class="px-12 py-3 text-lg" onclick={() => deleteModalOpen = true}>
                        <TrashBinSolid class="mr-3 h-6 w-6"/>
                        Delete Event
                    </Button>
                </div>
            </div>
        </div>
    {/if}

    <Modal bind:open={deleteModalOpen} size="md" autoclose>
        <div class="text-center">
            <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
                Are you sure you want to delete this event?
            </h3>
            <div class="flex justify-center gap-4">
                <Button color="red" class="px-8" disabled={isDeleting} onclick={handleDeleteEvent}>
                    {isDeleting ? 'Deleting...' : 'Yes, delete it'}
                </Button>
                <Button color="alternative" class="px-8" onclick={() => deleteModalOpen = false}>
                    No, cancel
                </Button>
            </div>
        </div>
    </Modal>
</section>




<ToastNotification bind:this={toast}/>