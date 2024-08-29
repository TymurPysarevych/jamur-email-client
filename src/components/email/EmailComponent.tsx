import './EmailComponent.css';
import EmailAttachmentComponent from "./attachments/EmailAttachmentComponent.tsx";
import EmailBody from "./body/EmailBody.tsx";
import {useState} from "react";
import {Email} from "../../interfaces/Email.ts";
import {invoke} from "@tauri-apps/api/tauri";
import {useSetRecoilState} from "recoil";
import {loadingState} from "../../state/atoms.ts";

export default function EmailComponent() {
    const [emails, setEmails] = useState<Array<Email>>([]);
    const setLoading = useSetRecoilState<boolean>(loadingState);

    const fetchEmails = async () => {
        setLoading(true)
        await invoke<Array<Email>>("fetch_messages", {server: '', login: '', password: ''})
            .then((response) => {
                setEmails(response)
                console.log("Emails: ", emails.length)
            })
            .catch((e) => console.error(e))
            .finally(() => setLoading(false))
    }

    return (
        <div>
            <button onClick={fetchEmails}>Fetch Emails</button>
            {emails.map((email: Email) =>
                <div className="email">
                    <h1>{email.subject}</h1>
                    <div>From: {email.from.join(", ")}</div>
                    <div>To: {email.to.join(", ")}</div>
                    <div>At: {email.deliveredAt}</div>
                    <EmailBody bodies={email.bodies}/>
                    <EmailAttachmentComponent attachments={email.attachments}/>
                </div>
            )}
        </div>
    )
}
