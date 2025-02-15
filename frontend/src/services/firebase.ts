import {initializeApp} from "firebase/app";
// import { getAnalytics } from "firebase/analytics";
import {getAuth} from "firebase/auth";

const firebaseConfig = {
    apiKey: "AIzaSyA8D1yRslzhdNlt4bPXoPGu4E_JYvx2omM",
    authDomain: "inventsys-8930b.firebaseapp.com",
    projectId: "inventsys-8930b",
    storageBucket: "inventsys-8930b.firebasestorage.app",
    messagingSenderId: "118335284422",
    appId: "1:118335284422:web:496df46707d702a5c363c7",
    measurementId: "G-Q6CG8KVLZF"
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);
// const analytics = getAnalytics(app);
const auth = getAuth(app);
export {auth, app};