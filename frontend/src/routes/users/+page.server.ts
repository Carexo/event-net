import type {PageServerLoad} from "./$types";
import {getApiUrl} from "$lib/utils/api";
import type {User} from "$lib/types/user";
import type {ApiResponse, PaginatedResponse} from "$lib/types/pagination";
import {USER_PAGINATION} from "$lib/config/pagination";

export const load: PageServerLoad = async ({fetch, url}) => {
    try {
        const page = Number(url.searchParams.get('page')) || USER_PAGINATION.DEFAULT_PAGE;
        const limit = USER_PAGINATION.DEFAULT_LIMIT;

        const apiUrl = getApiUrl(`/users?page=${page}&limit=${limit}`);
        const response = await fetch(apiUrl);


        if (!response.ok) {
            const data = await response.json();

            throw new Error(data?.message);
        };

        const result = await response.json() as ApiResponse<PaginatedResponse<User>>;

        return {
            users: result.data.items,
            pagination: {
                total: result.data.total,
                page: result.data.page,
                limit: result.data.limit,
                pages: result.data.pages
            },
            message: result.message
        };
    } catch (error: any) {
        return {
            users: [],
            pagination: {
                total: 0,
                page: 1,
                limit: USER_PAGINATION.DEFAULT_LIMIT,
                pages: 0
            },
            error: error.message
        };
    }
}
