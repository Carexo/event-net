import type {PageServerLoad} from "./$types";
import {getApiUrl} from "$lib/utils/api";
import type {User} from "$lib/types/user";

export const load: PageServerLoad = async ({fetch}) => {
    try {
        const response = await fetch(getApiUrl("/users"));

        if (!response.ok) {
            const data = await response.json();

            throw new Error(data?.message);
        }
        const users = await response.json();

        return {users: users.data as User[]};
    } catch (error: any) {
        return {
            users: [],
            error: error.message
        };
    }
}
