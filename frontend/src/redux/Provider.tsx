import {PersistGate} from "redux-persist/integration/react";
import {persistor, store} from "@/redux/store";
import {Provider} from "react-redux";
import {ReactNode} from "react";

export const AppStoreProvider = ({children}: { children: ReactNode }) => {
    return (
        <Provider store={store}>
            <PersistGate loading={null} persistor={persistor}>
                {children}
            </PersistGate>
        </Provider>
    )
}