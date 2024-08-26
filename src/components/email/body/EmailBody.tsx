import './EmailBody.css';
import ShadowRoot from "../../shadow-root/ShadowRoot.tsx";

interface EmailBodyProps {
    body: string;
}

export default function EmailBody({body}: EmailBodyProps) {
    const shadowContent =
        <div className="body" id="container">
            <div dangerouslySetInnerHTML={{__html: body}}/>
        </div>;

    return <ShadowRoot child={shadowContent}/>;
}
