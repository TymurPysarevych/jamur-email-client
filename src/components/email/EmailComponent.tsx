import './EmailComponent.css';
import EmailAttachmentComponent from "./attachments/EmailAttachmentComponent.tsx";
import {Email} from "../../interfaces/Email.ts";
import EmailBody from "./body/EmailBody.tsx";

interface EmailComponentProps {
    email: Email;
}

export default function EmailComponent({email}: EmailComponentProps) {
    return (
        <div className="email">
            <h1>{email.subject}</h1>
            <div>From: {email.from.join(", ")}</div>
            <div>To: {email.to.join(", ")}</div>
            {email.bodies.map((body, index) => <EmailBody key={index} body={body}/>)}
            {<EmailAttachmentComponent attachments={email.attachments}/>}
        </div>
    )
}
