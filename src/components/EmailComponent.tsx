import {Email} from "../interfaces/Email.ts";
import EmailBody from "./EmailBody.tsx";
import './EmailComponent.css';

interface EmailComponentProps {
    email: Email;
}

export default function EmailComponent({email}: EmailComponentProps) {
    return (
        <div className="email">
            <h1>{email.subject}</h1>
            <p>From: {email.from.join(", ")}</p>
            <p>To: {email.to.join(", ")}</p>
            {email.bodies.map((body, index) => <EmailBody key={index} body={body}/>)}
        </div>
    )
}
