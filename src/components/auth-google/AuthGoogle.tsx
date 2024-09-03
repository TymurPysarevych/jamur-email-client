import {invoke} from "@tauri-apps/api/tauri";

export default function AuthGoogle() {

    const login = async () => {
        await invoke('authenticate');
    };

    return (
        <div>
            <h2>React Google Login</h2>
            <br/>
            <button onClick={() => login()}>Sign in with Google 🚀</button>
        </div>
    );
}
