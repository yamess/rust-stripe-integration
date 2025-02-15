import Navbar from "@/components/Navbar";

const HomePage = () => {
    return (
        <div className={"flex flex-col h-screen"}>
            <Navbar />
            <main className={"h-max items-center justify-between flex-grow"}>Main</main>
        </div>

    );
}

export default HomePage;
