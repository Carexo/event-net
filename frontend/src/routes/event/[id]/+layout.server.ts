import type { LayoutServerLoad } from './$types';
import { getApiUrl } from '$lib/utils/api';
import type {EventCard} from "$lib/types/event";

export const load: LayoutServerLoad = async ({params, fetch}) => {
    try {
        const response = await fetch(getApiUrl(`/event/${params.id}`));

        if (!response.ok) {
            const data = await response.json();

            throw new Error(data?.message);
        }
        const event = await response.json();

        const event_date = new Date(event.data.start_datetime);

        return {
            event: {
                ...event.data,
                date: event_date,
                time: event_date.toTimeString().slice(0, 5)
            } as EventCard,
        };
    } catch (error: any) {
        return {
            event: null,
            error: error.message
        };
    }
}
