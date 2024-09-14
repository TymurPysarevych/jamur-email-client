import AuthGoogle from "../auth-google/AuthGoogle.tsx";
import SmtpSetup from "../smtp-setup/SmtpSetup.tsx";

export default function InitialSetup() {
    return (
        <div>
            <AuthGoogle/>
            <SmtpSetup/>
        </div>
    );
}
