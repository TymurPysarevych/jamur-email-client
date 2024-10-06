import './style.scss';
import { imapEmailsState, keychainEntriesState } from '../../state/atoms.ts';
import { useRecoilValue, useSetRecoilState } from 'recoil';
import { Email } from '../../interfaces/Email.ts';
import { useTauriInvoke } from '../../utils/UseTauriInvoke.ts';
import { KEYCHAIN_KEY_GMAIL, KEYCHAIN_KEY_IMAP, KeychainEntry } from '../../interfaces/KeychainEntry.ts';
import { GEmail } from '../../interfaces/GEmail.ts';

export default function Menu() {
  const keychainEntries = useRecoilValue(keychainEntriesState);
  const setImapEmails = useSetRecoilState(imapEmailsState);
  const [fetchImapMessages] = useTauriInvoke<Array<Email>>();
  const [fetchGmailMessages] = useTauriInvoke<Array<GEmail>>();

  const loadEmails = (entry: KeychainEntry) => {
    console.log('Loading emails for:', entry);
    if (entry.key.startsWith(KEYCHAIN_KEY_GMAIL)) {
      fetchGmailMessages('fetch_gmail_messages').then((emails) => {
        console.log(emails);
      });
    } else if (entry.key.startsWith(KEYCHAIN_KEY_IMAP)) {
      fetchImapMessages('fetch_messages', { keychainEntry: entry }).then((emails) => {
        setImapEmails(emails);
      });
    } else {
      console.error('Unknown keychain entry:', entry);
    }
  };

  return (
    <div className="menu-container">
      {keychainEntries.map((entry) => (
        <div className="menu-container--entry" onClick={() => loadEmails(entry)} key={`${entry.key}-${entry.id}`}>
          {entry.id}
        </div>
      ))}
    </div>
  );
}
