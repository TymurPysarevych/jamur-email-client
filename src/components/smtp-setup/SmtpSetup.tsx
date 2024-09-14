import "./style.css"
import {useState} from "react";

export default function SmtpSetup() {
    const [username, setUserName] = useState<string>('')
    const [imapHost, setImapHost] = useState<string>('')
    const [imapPort, setImapPort] = useState<number>(993)
    const [smtpHost, setSmtpHost] = useState<string>('')
    const [smtpPort, setSmtpPort] = useState<number>(587)

    return (
        <div className="form-container">
            <form>
                <div className="form-group">
                    <label htmlFor="username">Username/Email:</label>
                    <input
                        type="text"
                        id="username"
                        value={username}
                        onChange={(e) => setUserName(e.target.value)}
                    />
                </div>
                <div className="form-group">
                    <label htmlFor="imapHost">IMAP Host:</label>
                    <input
                        type="text"
                        id="imapHost"
                        value={imapHost}
                        onChange={(e) => setImapHost(e.target.value)}
                    />
                </div>
                <div className="form-group">
                    <label htmlFor="imapPort">IMAP Port:</label>
                    <input
                        type="number"
                        id="imapPort"
                        value={imapPort}
                        onChange={(e) => setImapPort(Number(e.target.value))}
                    />
                </div>
                <div className="form-group">
                    <label htmlFor="smtpHost">SMTP Host:</label>
                    <input
                        type="text"
                        id="smtpHost"
                        value={smtpHost}
                        onChange={(e) => setSmtpHost(e.target.value)}
                    />
                </div>
                <div className="form-group">
                    <label htmlFor="smtpPort">SMTP Port:</label>
                    <input
                        type="number"
                        id="smtpPort"
                        value={smtpPort}
                        onChange={(e) => setSmtpPort(Number(e.target.value))}
                    />
                </div>
                <button type="submit">Submit</button>
            </form>
        </div>
    );
}
