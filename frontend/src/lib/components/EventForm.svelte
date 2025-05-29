<script lang="ts">
    import {
        Button,
        Label,
        Input,
        Timepicker
    } from "flowbite-svelte";
    import {DateInput} from 'date-picker-svelte'
    import {CalendarWeekSolid, TagSolid, ClockSolid} from "flowbite-svelte-icons";
    import ToastNotification from "$lib/components/ToastNotification.svelte";
    import {getApiUrl} from "$lib/utils/api";
    import {createDate} from "$lib/utils/date";

    interface EventFormProps {
        eventId?: number;
        initialData?: {
            name: string;
            date?: Date;
            time?: string;
            keywords: string;
        };
        submitLabel: string;
        onSuccess: (eventId: number) => void;
    }
    // Props
    let {eventId, initialData, submitLabel, onSuccess}: EventFormProps = $props();

    // State
    let selectedDate = $state<Date | undefined>(initialData?.date);
    let selectedTime = $state(initialData?.time || "09:00");
    let eventName = $state(initialData?.name || "");
    let eventKeywords = $state(initialData?.keywords || "");
    let isSubmitting = $state(false);

    let toast: ToastNotification;

    function handleTimeChange(data: {
        [key: string]: string;
        time: string;
        endTime: string;
    }) {
        selectedTime = data.time;
    }

    async function handleSubmit() {
        if (!eventName.trim()) {
            toast.showToast("Event name is required.", "red");
            return;
        }
        if (!selectedDate) {
            toast.showToast("Please select a start date.", "red");
            return;
        }
        if (!selectedTime) {
            toast.showToast("Please select a start time.", "red");
            return;
        }
        if (!eventKeywords.trim()) {
            toast.showToast("Please enter at least one keyword.", "red");
            return;
        }

        const startDatetime = createDate(selectedDate, selectedTime);

        // Format the data for submission
        const formattedData = {
            name: eventName.trim(),
            start_datetime: startDatetime.toISOString(),
            keywords: eventKeywords.split(',').map(k => k.trim()).filter(k => k !== "")
        };

        if (startDatetime <= new Date()) {
            toast.showToast("Start date and time cannot be in the past.", "red");
            return;
        }

        console.log(`${eventId ? 'Updating' : 'Creating'} event:`, formattedData);
        isSubmitting = true;

        try {
            const url = eventId
                ? getApiUrl(`/event/${eventId}`)
                : getApiUrl('/event');

            const method = eventId ? 'PUT' : 'POST';

            const response = await fetch(url, {
                method,
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(formattedData)
            });

            if (!response.ok) {
                const errorData = await response.json();
                throw new Error(errorData.message || `Failed to ${eventId ? 'update' : 'create'} event.`);
            }

            const event = await response.json();

            toast.showToast(event.message || `Event ${eventId ? 'updated' : 'created'} successfully!`, "green");

            if (event.data && event.data.id) {
                onSuccess(event.data.id);
            }

        } catch (error: any) {
            console.error(`Error ${eventId ? 'updating' : 'creating'} event:`, error);
            toast.showToast(error.message, "red");
        } finally {
            isSubmitting = false;
        }
    }
</script>

<form onsubmit={handleSubmit}
      class="space-y-6 bg-white dark:bg-gray-800 p-8 rounded-lg shadow-md border border-gray-100">
    <!-- Event Name -->
    <div class="mb-4">
        <Label for="name" class="mb-2">Event Name*</Label>
        <Input id="name" required bind:value={eventName} placeholder="Enter event name"/>
    </div>

    <!-- Datetime Selector -->
    <div class="mb-4 flex gap-8">
        <div>
            <Label class="mb-2 flex items-center">
                <CalendarWeekSolid class="mr-2 h-5 w-5 text-blue-600"/>
                Start Date*
            </Label>
            <DateInput bind:value={selectedDate} format="yyyy-MM-dd" required closeOnSelection
                       placeholder="Enter start date"/>
        </div>
        <div>
            <Label class="mb-2 flex items-center">
                <ClockSolid class="mr-2 h-5 w-5 text-blue-600"/>
                Start Time*
            </Label>
            <Timepicker value={selectedTime} onselect={handleTimeChange}/>
        </div>
    </div>

    <!-- Keywords -->
    <div class="mb-4">
        <Label for="keywords" class="mb-2 flex items-center">
            <TagSolid class="mr-2 h-5 w-5 text-blue-600"/>
            Keywords*
        </Label>
        <Input id="keywords" bind:value={eventKeywords}
               placeholder="Comma-separated keywords (e.g., tech, workshop, networking)"/>
        <p class="text-sm text-gray-500 mt-1">These help people discover your event</p>
    </div>

    <!-- Submit Button -->
    <div class="flex justify-center mt-6">
        <Button type="submit" size="lg" color="blue" class="px-8" disabled={isSubmitting}>
            {isSubmitting ? 'Saving...' : submitLabel}
        </Button>
    </div>
</form>

<ToastNotification bind:this={toast}/>

<style lang="css">
    :global(.date-time-field input) {
        padding: 8px !important;
        border-radius: 8px !important;
        font-size: 14px !important;
    }
</style>