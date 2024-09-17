import './App.scss';
import LoadingComponent from './components/loading/LoadingComponent.tsx';
import InitialSetup from './components/initial-setup/InitialSetup.tsx';
import Menu from './components/menu/Menu.tsx';
import { useSetRecoilState } from 'recoil';
import { loadingState } from './state/atoms.ts';
import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

export default function App() {
  const setLoadingState = useSetRecoilState(loadingState);
  const [loadingCredsExist, setLoadingCredsExistState] = useState(true);
  const [foundCredentials, setFoundCredentials] = useState(false);

  useEffect(() => {
    setLoadingState(true);
    setLoadingCredsExistState(true);
    invoke<boolean>('credentials_exist')
      .then((bool) => setFoundCredentials(bool))
      .finally(() => {
        setLoadingState(false);
        setLoadingCredsExistState(false);
      });
  }, []);

  if (loadingCredsExist) {
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
    <div className="main-layout">
      <div className="menu">
        <Menu />
      </div>
      <div className="container">
        <LoadingComponent />
      </div>
    </div>
  );
}
