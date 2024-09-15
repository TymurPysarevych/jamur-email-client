import './App.css';
import { RecoilRoot } from 'recoil';
import LoadingComponent from './components/loading/LoadingComponent.tsx';
import InitialSetup from './components/initial-setup/InitialSetup.tsx';

export default function App() {
  return (
    <RecoilRoot>
      <div className="container">
        <LoadingComponent />
        <InitialSetup />
      </div>
    </RecoilRoot>
  );
}
