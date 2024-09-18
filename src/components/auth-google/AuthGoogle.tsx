import './style.scss';
import { useTauriInvoke } from '../../utils/UseTauriInvoke.ts';

export default function AuthGoogle() {
  const [invokeAuthenticateGoogle] = useTauriInvoke('authenticate_google');
  const login = async () => {
    await invokeAuthenticateGoogle();
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
