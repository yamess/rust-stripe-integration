import React from "react";

export default function Layout({children}: { children: React.ReactNode }){
    return (
        <>
            <div>
                <h1 className={"text-2xl text-amber-800"}>LayoutHeader</h1>
            </div>
            <div>{children}</div>
            <div>
                <h1 className={"text-2xl text-amber-800"}>LayoutFooter</h1>
            </div>
        </>

    );
}