// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	/// <reference types="lucia-auth" />
	declare namespace Lucia {
		type Auth = import('$lib/server/lucia').Auth;
		type GithubAuth = import('$lib/server/lucia').GithubAuth;
		type UserAttributes = {
			username: string;
			name: string;
			email: string;
			profile_picture: string;
		};
	}

	/// <reference types="@sveltejs/kit" />
	declare namespace App {
		interface Locals {
			validate: import('@lucia-auth/sveltekit').Validate;
			validateUser: import('@lucia-auth/sveltekit').ValidateUser;
			setSession: import('@lucia-auth/sveltekit').SetSession;
		}
	}
}

export {};
