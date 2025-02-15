'use client';

import HomePage from "@/app/HomePage";
import {AppStoreProvider} from "@/redux/Provider";

export default function Home() {
  return (
      <AppStoreProvider>
        <HomePage />
      </AppStoreProvider>
    );
}
