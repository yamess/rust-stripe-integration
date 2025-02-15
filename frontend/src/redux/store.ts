import {combineReducers, configureStore} from "@reduxjs/toolkit";
import {FLUSH, PAUSE, PERSIST, persistReducer, persistStore, PURGE, REGISTER, REHYDRATE} from 'redux-persist';
import {Persistor} from "redux-persist/es/types";
import createWebStorage from "redux-persist/es/storage/createWebStorage";
import {setupListeners} from "@reduxjs/toolkit/query/react";
import {userSlice} from "@/redux/slices/userSlice";
import {userApi} from "@/services/userApi";

;

const createNoopStorage = () => {
    return {
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        getItem(_key: string) {
            return Promise.resolve(null);
        },
        setItem(_key: string, value: string) {
            return Promise.resolve(value);
        },
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        removeItem(_key: string) {
            return Promise.resolve();
        },
    };
};

const storage =
    typeof window === "undefined" ? createNoopStorage() : createWebStorage('local');

const combinedReducers = combineReducers({
    user: userSlice.reducer,
    [userApi.reducerPath]: userSlice.reducer,
});

const persistConfig = {
    key: "root",
    storage,
    whitelist: ['user']
}

const persistedReducer = persistReducer(persistConfig, combinedReducers);

export const store = configureStore({
    reducer: persistedReducer,
    devTools: process.env.NODE_ENV !== 'production',
    middleware: (getDefaultMiddleware) => getDefaultMiddleware({
        serializableCheck: {
            ignoredActions: [PAUSE, PERSIST, PURGE, FLUSH, REHYDRATE, REGISTER],
        }
    }),
})

setupListeners(store.dispatch);

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
export const persistor: Persistor = persistStore(store);
