import {createApi, fetchBaseQuery} from "@reduxjs/toolkit/query/react";

export type Profile = {
    id: number;
    firstName: string;
    lastName: string;
    phone: string;
    photo_url: string;
}
export type User  = {
    id: number;
    email: string;
    firebaseId: string;
    stripeCustomerId: string;
    status: string;
    role: string;
    profile: Profile;
}

export const userApi = createApi({
    reducerPath: 'userApi',
    baseQuery: fetchBaseQuery({baseUrl: process.env.NEXT_PUBLIC_API_URL}),
    tagTypes: ['User'],
    endpoints: (builder) => ({
        registerUser: builder.mutation({
            query: (data: User) => ({
                url: '/users',
                method: 'POST',
                body: data,
                headers: {"Authorization": `Bearer ${localStorage.getItem('token')}`}
            }),
            invalidatesTags: ['User'],
        }),
        login: builder.query<User, string>({
            query: (token) => ({
                url: '/auth/login',
                method: 'POST',
                headers: {"Authorization": `Bearer ${token}`}
            }),
            providesTags: ['User'],
        }),
        getUserData: builder.query<User, string>({
            query: (token) => ({
                url: '/users/me',
                method: 'GET',
                headers: {"Authorization": `Bearer ${token}`}
            }),
            providesTags: ['User'],
        }),
    }),
})