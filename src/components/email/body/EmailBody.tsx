import './EmailBody.css';
import ShadowRoot from "../../shadow-root/ShadowRoot.tsx";

interface EmailBodyProps {
    bodies: Array<string>;
}

export default function EmailBody({bodies}: EmailBodyProps) {
    const shadowContent = (body: string) =>
        <div className="body" id="container">
            <div dangerouslySetInnerHTML={{__html: body}}/>
        </div>
    return (
        bodies.map((body: string, index) => <ShadowRoot key={index} child={shadowContent(body)}/>)
    );
}
