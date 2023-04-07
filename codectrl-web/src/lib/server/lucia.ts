import { dev } from '$app/environment';
import { PrismaClient } from '@prisma/client';
import lucia from 'lucia-auth';
import prisma from '@lucia-auth/adapter-prisma';
import { github, discord } from '@lucia-auth/oauth/providers';
import { GITHUB_SECRET, GITHUB_ID, DISCORD_ID, DISCORD_SECRET } from '$env/static/private';
import { base } from '$app/paths';

const client = new PrismaClient();

export const auth = lucia({
	adapter: prisma(client),
	env: dev ? 'DEV' : 'PROD',
	autoDatabaseCleanup: true,
	transformUserData: (userData) => {
		return userData;
	}
});

export const githubAuth = github(auth, { clientId: GITHUB_ID, clientSecret: GITHUB_SECRET });
export const discordAuth = discord(auth, {
	clientId: DISCORD_ID,
	clientSecret: DISCORD_SECRET,
	redirectUri: base
});

export type Auth = typeof auth;
export type GithubAuth = typeof githubAuth;
