import { FcGoogle } from "react-icons/fc";
import {auth} from "../../services/firebase";
import {GoogleAuthProvider, signInWithPopup} from "firebase/auth";
import {useUser} from "@/hooks/useUser";
import {useNavigate} from "react-router-dom";


export interface FirebaseUser {
    id: string,
    email: string,
    name: string,
    photoUrl: string,
}

export default function LoginPage(){
    const {setUser, setToken, setLoginState, token, user, isLoggedIn} = useUser();
    const navigate = useNavigate();

    const handleGoogleLogin = async () => {
        try {
            const provider = new GoogleAuthProvider();
            const result = await signInWithPopup(auth, provider);
            const token = await result.user.getIdToken(true);
            if (!token) {
                console.log("Google login failed");
                return
            }
            setToken(token);

            navigate("/dashboard");
        } catch (error) {
            console.log(error);
        }
    }

    return (
        <div className="flex flex-col h-screen justify-center px-6 py-12 lg:px-8">
            <div className={"bg-white sm:w-1/5 min-w-fit mx-auto p-5 rounded-md shadow-gray-200 shadow-md"}>
                <div className="mx-auto text-center w-full">
                    <h2 className="mt-10 text-center text-2xl/9 font-bold tracking-tight text-gray-900">
                        Sign in to your account
                    </h2>
                    <button onClick={handleGoogleLogin}>
                        <FcGoogle className={"mx-auto w-12 h-12 text-gray-800"} />
                    </button>
                </div>

                <div className={"mt-5 mb-5 grid md:grid-cols-2 items-center gap-3 w-full sm:mx-auto"}>

                </div>
            </div>
        </div>
    )
}
