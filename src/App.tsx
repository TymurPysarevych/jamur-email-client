import './App.scss';
import InitialSetup from './components/initial-setup/InitialSetup.tsx';
import Menu from './components/menu/Menu.tsx';
import { useRecoilState } from 'recoil';
import { keychainEntriesState } from './state/atoms.ts';
import { useEffect, useState } from 'react';
import { KeychainEntry } from './interfaces/KeychainEntry.ts';
import { useTauriInvoke } from './utils/UseTauriInvoke.ts';
import EmailPreview from './components/email/preview/EmailPreview.tsx';

export default function App() {
  const [loadingCredsExist, setLoadingCredsExistState] = useState(true);
  const [keychainEntries, setKeychainEntries] = useRecoilState(keychainEntriesState);
  const [fetchKeychainEntries] = useTauriInvoke<Array<KeychainEntry>>();

  useEffect(() => {
    setLoadingCredsExistState(true);
    fetchKeychainEntries('credentials_exist')
      .then((entries) => setKeychainEntries(entries))
      .finally(() => {
        setLoadingCredsExistState(false);
      });
  }, []);

  if (loadingCredsExist) {
    return <></>;
  }

  if (keychainEntries.length === 0) {
    return (
      <>
        <InitialSetup />
      </>
    );
  }
  return (
    <div className="main-layout">
      <div className="menu">
        <Menu />
      </div>
      <div className="container">
        <div className="container--email-preview">
          <EmailPreview />
        </div>
        <div className="container--email-view">
          <h1>Email View</h1>
        </div>
      </div>
    </div>
  );
}
