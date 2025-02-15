'use client';

import React, {useLayoutEffect} from "react";
import {useUser} from "@/hooks/useUser";
import {AppStoreProvider} from "@/redux/Provider";
import {redirect} from "next/navigation";

const Wrapper = ({children}: { children: React.ReactNode }) => {
    const {isLoggedIn} = useUser();
    useLayoutEffect(() => {
        if (!isLoggedIn) {
            redirect('/login');
        }
    }, [isLoggedIn]);

    return <>{children}</>;
}

export default function Layout({children}: { children: React.ReactNode }){
    return (
        <AppStoreProvider>
            <div>
                <h1 className={"text-2xl text-amber-800"}>LayoutHeader</h1>
            </div>
            <Wrapper>{children}</Wrapper>
            <div>
                <h1 className={"text-2xl text-amber-800"}>LayoutFooter</h1>
            </div>
        </AppStoreProvider>

    );
}