import {useState} from "react";
import {invoke} from "@tauri-apps/api/tauri";
import "./App.css";
import {Email} from "./interfaces/Email.ts";
import DOMPurify from 'dompurify';

function App() {
    const [emails, setEmails] = useState<Array<Email>>([]);

    const fetchEmails = async () => {
        await invoke<Array<Email>>("fetch_messages", {server: '', login: '', password: ''})
        // await invoke<Array<Email>>("fetch_by_query", {server: '', login: '', password: '', since: '20-Jul-2024'})
            .then((response) => setEmails(response))
            .catch((e) => console.error(e))
    }

    const sanitize = (html: Array<string>) => {
        return DOMPurify.sanitize(html.join(), {USE_PROFILES: {html: true}});
    }

    return (
        <div className="container">
            <button onClick={fetchEmails}>Fetch Emails</button>
            <h1>All Emails</h1>
            <div>
                {emails.map((emailDom, index) => (
                    <div key={index} className="email">
                        <h1>{emailDom.subject}</h1>
                        <p>From: {emailDom.from.join(", ")}</p>
                        <p>To: {emailDom.to.join(", ")}</p>
                        <div dangerouslySetInnerHTML={{__html: sanitize(emailDom.body)}}></div>
                    </div>
                ))}
            </div>
        </div>
    );
}

export default App;
