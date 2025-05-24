import {writable} from 'svelte/store';

export const selectedUser = writable<string | null>(null);

export function selectUser(userName: string) {
    selectedUser.set(userName);
}