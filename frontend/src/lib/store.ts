import { browser } from '$app/environment';
import { writable } from 'svelte/store';

let persistedUser = browser && localStorage.getItem('current_user');
let persistedRole = browser && localStorage.getItem('user_role');
let persistedRefreshToken = browser && localStorage.getItem('refresh_token');
let persistedAccess = browser && sessionStorage.getItem('access_token');

export const currentUser = writable<string>(persistedUser ? persistedUser : '');
export const currentRole = writable<string[]>(persistedRole ? persistedRole.split(',') : []);
export const isLogginIn = writable(false);
export const accessToken = writable(persistedAccess ? persistedAccess : '');
export const refreshToken = writable(persistedRefreshToken ? persistedRefreshToken : '');
export const tokenType = writable('');

if (browser) {
	currentUser.subscribe((u) => (localStorage.current_user = u));
	refreshToken.subscribe((o) => (localStorage.refresh_token = o));
	currentRole.subscribe((o) => (localStorage.user_role = o?.join(',')));
	accessToken.subscribe((o) => (sessionStorage.access_token = o));
}

// currentUser.subscribe((u) => (localStorage.current_user = u));
// refreshToken.subscribe((o) => (localStorage.refresh_token = o));
// currentRole.subscribe((o) => (localStorage.user_role = o.join(",")));
// accessToken.subscribe((o) => (sessionStorage.access_token = o));
