import type {PageServerLoad} from "./$types";
import {getApiUrl} from "$lib/utils/api";
import type {EventCard} from "$lib/types/event";

export const load: PageServerLoad = async ({fetch}) => {
    try {
        const response = await fetch(getApiUrl("/events"));

        if (!response.ok) {
            const data = await response.json();

            throw new Error(data?.message);
        }
        const events = await response.json();

        return {events: events.data as EventCard[]};
    } catch (error: any) {
        return {
            events: [] as EventCard[],
            error: error.message
        };
    }
}