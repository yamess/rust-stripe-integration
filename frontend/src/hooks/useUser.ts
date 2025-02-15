import {useAppDispatch, useAppSelector} from "@/redux/hooks";
import {User} from "@/services/userApi";

export const useUser = () => {
    const dispatch = useAppDispatch();
    const user = useAppSelector((state) => state.user.user);
    const token = useAppSelector((state) => state.user.token);
    const isLoggedIn = useAppSelector((state) => state.user.isLoggedIn);

    const setUser = (user: User) => dispatch({type: 'user/setUser', payload: user});
    const setToken = (token: string) => dispatch({type: 'user/setToken', payload: token});
    const setLoginState = (isLoggedIn: boolean) => dispatch({type: 'user/setIsLoggedIn', payload: isLoggedIn});

    return {setUser, setToken, setLoginState, user, token, isLoggedIn};
}