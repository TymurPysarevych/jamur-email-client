import { invoke } from '@tauri-apps/api/tauri';
import './style.scss';
import { useSetRecoilState } from 'recoil';
import { loadingState } from '../../state/atoms.ts';

export default function AuthGoogle() {
  const setLoadingState = useSetRecoilState(loadingState);
  const login = () => {
    setLoadingState(true);
    invoke('authenticate_google').finally(() => setLoadingState(false));
  };

  /**
   * following code and styles are part of Googles guidelines from 13th Sep. 2024
   * @see https://developers.google.com/identity/branding-guidelines
   */
  return (
    <div className={'form-container, center'}>
      <button className="gsi-material-button" onClick={() => login()}>
        <div className="gsi-material-button-state"></div>
        <div className="gsi-material-button-content-wrapper">
          <div className="gsi-material-button-icon">
            <img src="src/assets/images/google_logo.svg" />
          </div>
          <span className="gsi-material-button-contents">Sign in with Google</span>
        </div>
      </button>
    </div>
  );
}
