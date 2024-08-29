import "./App.css";
import EmailComponent from "./components/email/EmailComponent.tsx";
import {RecoilRoot} from "recoil";
import LoadingComponent from "./components/loading/LoadingComponent.tsx";

export default function App() {
    return (
        <RecoilRoot>
            <div className="container">
                <LoadingComponent/>
                <EmailComponent/>
            </div>
        </RecoilRoot>
    );
}
