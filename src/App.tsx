import {useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import "./App.css";
import {Email} from "./interfaces/Email.ts";
import EmailComponent from "./components/EmailComponent.tsx";

export default function App() {
    const [emails, setEmails] = useState<Array<Email>>([]);

    const fetchEmails = async () => {
        await invoke<Array<Email>>("fetch_messages", {server: '', login: '', password: ''})
            .then((response) => {
                setEmails(response)
            })
            .catch((e) => console.error(e))
    }

    return (
        <div className="container">
            <button onClick={fetchEmails}>Fetch Emails</button>
            <h1>All Emails</h1>
            <div>
                {emails.map((email, index) => <EmailComponent key={index} email={email}/>)}
            </div>
        </div>
    );
}
