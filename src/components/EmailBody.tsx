import './EmailBody.css';
import {ShadowRoot} from "./shadow-root/ShadowRoot.tsx";

interface EmailBodyProps {
    body: string;
}

export default function EmailBody({body}: EmailBodyProps) {

    // const sanitize = (html: string) => {
    //     return DOMPurify.sanitize(html, {SANITIZE_DOM: true});
    //     // return (html);
    // }

    const shadowContent =
        <div className="body" id="container">
            <div dangerouslySetInnerHTML={{__html: body}}/>
        </div>;

    return <ShadowRoot child={shadowContent}/>
}
