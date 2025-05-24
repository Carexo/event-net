import type {PageServerLoad} from './$types';
import {getApiUrl} from "$lib/utils/api";
import type {EventCard} from "$lib/types/event";

export const load: PageServerLoad = async ({params, fetch}) => {
    try {
        const response = await fetch(getApiUrl(`/event/${params.slug}`));

        if (!response.ok) {
            const data = await response.json();

            throw new Error(data?.message);
        }
        const event = await response.json();

        return {event: event.data as EventCard};
    } catch (error: any) {
        return {
            event: null,
            error: error.message
        };
    }
}