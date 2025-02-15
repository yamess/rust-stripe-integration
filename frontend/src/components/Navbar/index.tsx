import Link from "next/link";


const Navbar = () => {
    return (
        <div className={"flex justify-between items-center md:mb-7 w-full px-48 py-5 bg-white"}>
            <div className={"flex justify-between items-center"}>
                <button className={"w-10 h-10 text-3xl text-amber-800"}>Logo</button>
            </div>
            <div className={"flex justify-between items-center gap-8"}>
                <Link href={"/"} className={"text-2xl  hover:underline"}>Home</Link>
                <Link href={"/services"} className={"text-2xl  hover:underline"}>Services</Link>
                <Link href={"/pricing"} className={"text-2xl  hover:underline"}>Pricing</Link>
                <Link href={"/about"} className={"text-2xl  hover:underline"}>About</Link>
                <Link href={"/contact"} className={"text-2xl  hover:underline"}>Contact</Link>
            </div>
            <div className={"flex justify-between items-center gap-3"}>
                <Link href={"/login"} className={"text-2xl  hover:underline"}>Login</Link>
                <Link href={"/register"} className={"text-2xl  hover:underline"}>Register</Link>
            </div>
        </div>
    );
}
export default Navbar;