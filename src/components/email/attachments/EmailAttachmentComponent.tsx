import './EmailAttachmentComponent.css';
import {EmailAttachment} from "../../../interfaces/Email.ts";
import {BaseDirectory, writeBinaryFile} from "@tauri-apps/api/fs";

interface EmailAttachmentProps {
    attachments: Array<EmailAttachment>;
}

export default function EmailAttachmentComponent({attachments}: EmailAttachmentProps) {
    async function createDownloadableAttachments(attachment: EmailAttachment) {
        const contents = new Uint8Array(attachment.content);
        await writeBinaryFile({path: attachment.filename, contents}, {dir: BaseDirectory.Download});
    }

    function filename(filename: string) {
        const maxFilenameLength = 15;
        const fileExtension = filename.split('.').pop();
        return filename.length > maxFilenameLength ? filename.substring(0, maxFilenameLength) + '... .' + fileExtension : filename
    }

    return (
        <div className="attachments">
            {attachments.map((attachment, index) => {
                return (
                    <div key={index} className="attachment" onClick={() => createDownloadableAttachments(attachment)}>
                        <div className="file-icon">📄</div>
                        <div className="filename">{filename(attachment.filename)}</div>
                    </div>
                );
            })}
        </div>
    );

}
