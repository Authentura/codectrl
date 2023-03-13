import { githubAuth } from '$lib/server/lucia';
import type { Action, PageServerLoad } from './$types';
import { auth } from '$lib/server/lucia';
import { fail } from '@sveltejs/kit';
import type { Actions } from './$types';

export const load = (async ({ locals }) => {
	const [url, _] = await githubAuth.getAuthorizationUrl();
	const { session, user } = await locals.validateUser();

	return { url: url.toString(), session, user };
}) satisfies PageServerLoad;

export const actions: Actions = {
	default: async ({ locals }) => {
		const session = await locals.validate();
		if (!session) return fail(401);
		await auth.invalidateSession(session.sessionId); // invalidate session
		locals.setSession(null); // remove cookie
	}
};
