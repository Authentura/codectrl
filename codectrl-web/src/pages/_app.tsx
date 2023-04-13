import { type AppType } from "next/app";
import {
  ClerkProvider,
  SignIn,
  SignOutButton,
  SignedIn,
  SignedOut,
} from "@clerk/nextjs";

import { api } from "~/utils/api";

import "~/styles/globals.css";

const MyApp: AppType = ({ Component, pageProps }) => {
  return (
    <ClerkProvider {...pageProps}>
      <SignedOut>
        <div className={"flex h-screen items-center justify-center"}>
          <SignIn />
        </div>
      </SignedOut>

      <SignedIn>
        <SignOutButton />
        <Component {...pageProps} />
      </SignedIn>
    </ClerkProvider>
  );
};

export default api.withTRPC(MyApp);
