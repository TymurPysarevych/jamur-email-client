import './style.css';
import { useState } from 'react';

export default function SmtpSetup() {
  const [email, setEmail] = useState<string>('');
  const [imapHost, setImapHost] = useState<string>('');
  const [imapPort, setImapPort] = useState<number>();
  const [smtpHost, setSmtpHost] = useState<string>('');
  const [smtpPort, setSmtpPort] = useState<number>();

  return (
    <div className="form-container">
      <form>
        <div className="form-group">
          <label htmlFor="email">Email:</label>
          <input
            placeholder="example@mail.com"
            type="text"
            id="email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />
        </div>
        <div className="inline-group">
          <div className="form-group">
            <label htmlFor="imapHost">IMAP Host:</label>
            <input
              placeholder="imap.mail.com"
              type="text"
              id="imapHost"
              value={imapHost}
              onChange={(e) => setImapHost(e.target.value)}
            />
          </div>
          <div className="form-group">
            <label htmlFor="imapPort">IMAP Port:</label>
            <input
              placeholder="993"
              type="number"
              id="imapPort"
              value={imapPort}
              onChange={(e) => setImapPort(Number(e.target.value))}
            />
          </div>
        </div>
        <div className="inline-group">
          <div className="form-group">
            <label htmlFor="smtpHost">SMTP Host:</label>
            <input
              placeholder="smtp.mail.com"
              type="text"
              id="smtpHost"
              value={smtpHost}
              onChange={(e) => setSmtpHost(e.target.value)}
            />
          </div>
          <div className="form-group">
            <label htmlFor="smtpPort">SMTP Port:</label>
            <input
              placeholder="587"
              type="number"
              id="smtpPort"
              value={smtpPort}
              onChange={(e) => setSmtpPort(Number(e.target.value))}
            />
          </div>
        </div>
        <button type="submit">Save</button>
      </form>
    </div>
  );
}
