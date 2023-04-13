import { createTRPCRouter } from "~/server/api/trpc";
import { exampleRouter } from "~/server/api/routers/example";

import Pusher from "pusher";

/**
 * This is the primary router for your server.
 *
 * All routers added in /api/routers should be manually added here.
 */
export const appRouter = createTRPCRouter({
  example: exampleRouter,
});

export const serverPusher = new Pusher({
  appId: "a7802592-9b4f-4d04-9cd7-6674a16eb531",
  key: process.env.PUSHER_KEY ?? "",
  secret: "IWtUuhPIUciHiPQvr9DEK4mMZwg2PNDg",
  useTLS: true,
  host: "dev-9406c092-5b8b-4a0c-9c59-67e78414165f.authentura.workers.dev",
  port: "443",
});

// export type definition of API
export type AppRouter = typeof appRouter;
