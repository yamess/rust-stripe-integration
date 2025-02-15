import {createSlice, PayloadAction} from "@reduxjs/toolkit";
import {User} from "@/services/userApi";

export interface UserState {
    user: User | null;
    token: string | null;
    isLoggedIn: boolean;
}

const initialState: UserState = {
    user: null,
    token: null,
    isLoggedIn: false
}

export const userSlice = createSlice({
    name: "user",
    initialState,
    reducers: {
        setUser: (state, action: PayloadAction<User>) => {
            state.user = action.payload;
        },
        setToken: (state, action: PayloadAction<string>) => {
            state.token = action.payload;
        },
        setAuthStatus: (state, action: PayloadAction<boolean>) => {
            state.isLoggedIn = action.payload;
        },
        clearUserState: (state: UserState) => {
            state = initialState;
            console.log(`User: ${state}`); // @TODO: Remove this line
        }
    }
});

export const { setUser, setToken, setAuthStatus, clearUserState} = userSlice.actions;
export default userSlice.reducer;