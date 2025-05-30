import type {PageServerLoad} from "./$types";
import {getApiUrl} from "$lib/utils/api";
import type {EventCard} from "$lib/types/event";

export const load: PageServerLoad = async ({fetch, url}) => {
    try {
				const keywordsParams = url.searchParams.getAll("keyword");
				const searchParams = new URLSearchParams();
				keywordsParams.forEach((keyword) => searchParams.append("keyword", keyword));
        let response = await fetch(getApiUrl("/events/filter?" + searchParams.toString()));

        if (!response.ok) {
            const data = await response.json();

            throw new Error(data?.message);
        }
        const events = await response.json();

				response = await fetch(getApiUrl("/events/keywords"));
				if (!response.ok) {
					const data = await response.json();
					throw new Error(data?.message);
				}

				const keywords = ((await response.json()).data as string[])

        return {events: events.data as EventCard[], keywords: keywords, keywordsParams: keywordsParams};
    } catch (error: any) {
        return {
            events: [] as EventCard[],
            error: error.message
        };
    }
}