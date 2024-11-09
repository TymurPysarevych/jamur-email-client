import './App.scss';
import InitialSetup from './components/initial-setup/InitialSetup.tsx';
import Menu from './components/menu/Menu.tsx';
import { useRecoilState, useRecoilValue, useSetRecoilState } from 'recoil';
import { keychainEntriesState, selectedEmailState, snacksState } from './state/atoms.ts';
import { useEffect, useState } from 'react';
import { KeychainEntry } from './interfaces/KeychainEntry.ts';
import { useTauriInvoke } from './utils/UseTauriInvoke.ts';
import EmailPreview from './components/email/preview/EmailPreview.tsx';
import { listen } from '@tauri-apps/api/event';
import { Snacks } from './interfaces/Snacks.ts';
import SnacksView from './components/snacks/SnacksView.tsx';
import EmailDetailsComponent from './components/email/detail/EmailDetailsComponent.tsx';

export default function App() {
  const [loadingCredsExist, setLoadingCredsExistState] = useState(true);
  const [keychainEntries, setKeychainEntries] = useRecoilState(keychainEntriesState);
  const setSnacks = useSetRecoilState(snacksState);
  const selectedEmail = useRecoilValue(selectedEmailState);
  const [fetchKeychainEntries] = useTauriInvoke<Array<KeychainEntry>>();

  useEffect(() => {
    setLoadingCredsExistState(true);
    fetchKeychainEntries('credentials_exist')
      .then((entries) => setKeychainEntries(entries))
      .finally(() => {
        setLoadingCredsExistState(false);
      });

    listen<Snacks>('show_snack', ({ payload }) => {
      setSnacks(payload);
    }).finally(() => {});
  }, []);

  if (loadingCredsExist) {
    return <></>;
  }

  if (keychainEntries.length === 0) {
    return (
      <>
        <SnacksView />
        <InitialSetup />
      </>
    );
  }

  return (
    <>
      <SnacksView />
      <div className="main-layout">
        <div className="menu">
          <Menu />
        </div>
        <div className="container">
          <div className="container--email-preview">
            <EmailPreview />
          </div>
          <div className="container--email-view">{selectedEmail.id && <EmailDetailsComponent />}</div>
        </div>
      </div>
    </>
  );
}
