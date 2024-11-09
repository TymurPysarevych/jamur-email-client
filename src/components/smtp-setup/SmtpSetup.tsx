import './style.scss';
import { useState } from 'react';
import Button from '../../ui/button/Button.tsx';
import { useTauriInvoke } from '../../utils/UseTauriInvoke.ts';
import { Email } from '../../interfaces/Email.ts';
import { useSetRecoilState } from 'recoil';
import { keychainEntriesState } from '../../state/atoms.ts';
import { KeychainEntry } from '../../interfaces/KeychainEntry.ts';

export default function SmtpSetup() {
  const setKeychainEntries = useSetRecoilState(keychainEntriesState);
  const [fetchKeychainEntries] = useTauriInvoke<Array<KeychainEntry>>();
  const [email, setEmail] = useState<string>('');
  const [keychainId, setKeychainId] = useState<string>('');
  const [password, setPassword] = useState<string>('');
  const [imapHost, setImapHost] = useState<string>('');
  const [imapPort, setImapPort] = useState<number>();
  const [smtpHost, setSmtpHost] = useState<string>('');
  const [smtpPort, setSmtpPort] = useState<number>();
  const [invokeSaveImapConfig] = useTauriInvoke<Array<Email>>();

  const saveDisabled =
    !email ||
    !keychainId ||
    !password ||
    !imapHost ||
    !imapPort ||
    !smtpHost ||
    !smtpPort ||
    email.length < 1 ||
    keychainId.length < 1 ||
    password.length < 1 ||
    imapHost.length < 1 ||
    smtpHost.length < 1;

  const save = () => {
    invokeSaveImapConfig('save_imap_config', {
      webCreds: {
        config: {
          username: email,
          imapHost,
          imapPort,
          smtpHost,
          smtpPort,
          keychainId
        },
        password
      }
    }).finally(() => {
      fetchKeychainEntries('credentials_exist').then((entries) => setKeychainEntries(entries));
    });
  };

  return (
    <div className="form-container">
      <form>
        <div className="form-group">
          <label htmlFor="name">Name:</label>
          <input
            placeholder="Name #1"
            type="text"
            id="name"
            value={keychainId}
            onChange={(e) => setKeychainId(e.target.value)}
          />
        </div>
        <div className="inline-group">
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
          <div className="form-group">
            <label htmlFor="password">Password:</label>
            <input type="password" id="password" value={password} onChange={(e) => setPassword(e.target.value)} />
          </div>
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
        <Button text={'Save'} onClick={save} icon={'hugeicons:floppy-disk'} disabled={saveDisabled} />
      </form>
    </div>
  );
}
