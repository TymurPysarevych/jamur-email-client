import './App.css';
import LoadingComponent from './components/loading/LoadingComponent.tsx';
import InitialSetup from './components/initial-setup/InitialSetup.tsx';
import Menu from './components/menu/Menu.tsx';
import { useRecoilState } from 'recoil';
import { loadingState } from './state/atoms.ts';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export default function App() {
  const [loading, setLoadingState] = useRecoilState(loadingState);
  const [foundCredentials, setFoundCredentials] = useState(false);

  useEffect(() => {
    setLoadingState(true);
    invoke<boolean>('credentials_exist')
      .then((bool) => setFoundCredentials(bool))
      .finally(() => setLoadingState(false));
  }, []);

  if (loading) {
    return (
      <>
        <LoadingComponent />
      </>
    );
  }

  if (!foundCredentials) {
    return (
      <>
        <LoadingComponent />
        <InitialSetup />
      </>
    );
  }
  return (
    <div className="container">
      <Menu />
      <LoadingComponent />
    </div>
  );
}
