import { auth, githubAuth } from '$lib/server/lucia';
import { USER_ID } from '$lib/server/stores/user';
import { error, redirect } from '@sveltejs/kit';
import { LuciaError } from 'lucia-auth';
import type { RequestHandler } from './$types';

export const GET = (async ({ url, cookies, params: { provider } }) => {
	const code = url.searchParams.get('code');

	if (!code) throw error(400, 'Bad request');

	let providerAuth;

	switch (provider) {
		case 'github':
			providerAuth = githubAuth;
			break;
		default:
			throw error(404, 'Provider not found');
	}

	try {
		const response = await providerAuth.validateCallback(code);

		let id: string = '';
		const unsubscribe = USER_ID.subscribe((v) => (id = v));

		if (!id || id === '') {
			let { id: userId } = await auth.createUser({
				primaryKey: {
					providerId: provider,
					providerUserId: response.providerUserId,
					password: null
				},
				attributes: {
					username: response.providerUser.login,
					name: response.providerUser.name,
					email: response.providerUser.email,
					profile_picture: response.providerUser.avatar_url
				}
			});
			USER_ID.set(userId);
		}

		const session = await auth.createSession(id);
		const sessionCookies = auth.createSessionCookies(session);

		for (let cookie of sessionCookies) {
			cookies.set(cookie.name, cookie.value, { path: '/' });
		}

		unsubscribe();
	} catch (e) {
		if (e instanceof LuciaError) throw error(500, e.message);
	}

	throw redirect(307, '/');
}) satisfies RequestHandler;
