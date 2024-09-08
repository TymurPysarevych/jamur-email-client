import {invoke} from "@tauri-apps/api/tauri";

export default function AuthGoogle() {

    const login = async () => {
        await invoke('authenticate_google');
    };

    const getMails = async () => {
        await invoke('fetch_gmail_messages');
    }

    return (
        <div>
            <h2>React Google Login</h2>
            <br/>
            <button onClick={() => login()}>Sign in with Google 🚀</button>
            <button onClick={() => getMails()}>Get all Emails</button>
        </div>
    );
}
