import './style.scss';
import AuthGoogle from '../auth-google/AuthGoogle.tsx';
import SmtpSetup from '../smtp-setup/SmtpSetup.tsx';

export default function InitialSetup() {
  return (
    <div className="initial-setup-container">
      <SmtpSetup />
      <div className="form-container center">
        <hr />
        or
        <hr />
      </div>
      <AuthGoogle />
    </div>
  );
}
