import {useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
    const [greetMsg, setGreetMsg] = useState("");
    const [server, setServer] = useState("");
    const [login, setLogin] = useState("");
    const [password, setPassword] = useState("");

    async function greet() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        const response = await invoke("fetch_messages", {server, login, password})
        console.log(response)
        setGreetMsg(`${response}`);
    }

    return (
        <div className="container">
            <form
                className="row"
                onSubmit={(e) => {
                    e.preventDefault();
                    greet();
                }}
            >
                <input
                    onChange={(e) => setServer(e.currentTarget.value)}
                    placeholder="Enter a server..."
                />
                <input
                    onChange={(e) => setLogin(e.currentTarget.value)}
                    placeholder="Enter a name..."
                />
                <input
                    onChange={(e) => setPassword(e.currentTarget.value)}
                    placeholder="Enter a password..."
                />
                <button type="submit">Start</button>
            </form>

            <p>{greetMsg}</p>
        </div>
    );
}

export default App;
