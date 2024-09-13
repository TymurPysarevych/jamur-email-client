import "./App.css";
import EmailComponent from "./components/email/EmailComponent.tsx";
import {RecoilRoot} from "recoil";
import LoadingComponent from "./components/loading/LoadingComponent.tsx";
import InitialSetup from "./components/initial-setup/InitialSetup.tsx";

export default function App() {
    return (
        <RecoilRoot>
            <div className="container">
                <LoadingComponent/>
                <EmailComponent/>
                <InitialSetup/>
            </div>
        </RecoilRoot>
    );
}
