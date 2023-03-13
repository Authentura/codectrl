import { writable, type Writable } from 'svelte/store';

export const USER_ID: Writable<string> = writable();
