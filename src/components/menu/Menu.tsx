import './style.scss';
import { keychainEntriesState } from '../../state/atoms.ts';
import { useRecoilValue } from 'recoil';
import { Email } from '../../interfaces/Email.ts';
import { useTauriInvoke } from '../../utils/UseTauriInvoke.ts';
import { KEYCHAIN_KEY_GMAIL, KEYCHAIN_KEY_IMAP, KeychainEntry } from '../../interfaces/KeychainEntry.ts';
import { GEmail } from '../../interfaces/GEmail.ts';

export default function Menu() {
  const keychainEntries = useRecoilValue(keychainEntriesState);
  const [fetchImapMessages] = useTauriInvoke<Array<Email>>('fetch_messages');
  const [fetchGmailMessages] = useTauriInvoke<Array<GEmail>>('fetch_gmail_messages');

  const loadEmails = (entry: KeychainEntry) => {
    if (entry.key.startsWith(KEYCHAIN_KEY_GMAIL)) {
      fetchImapMessages().then((emails) => {
        console.log(emails);
      });
    } else if (entry.key.startsWith(KEYCHAIN_KEY_IMAP)) {
      fetchGmailMessages().then((emails) => {
        console.log(emails);
      });
    } else {
      console.error('Unknown keychain entry:', entry);
    }
  };

  return (
    <div className="menu-container">
      {keychainEntries.map((entry) => (
        <div onClick={() => loadEmails(entry)} key={`${entry.key}-${entry.id}`}>
          {entry.id}
        </div>
      ))}
    </div>
  );
}
