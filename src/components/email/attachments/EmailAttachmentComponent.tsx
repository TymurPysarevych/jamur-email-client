import './EmailAttachmentComponent.scss';
import { EmailAttachment } from '../../../interfaces/Email.ts';
import { BaseDirectory, writeFile } from '@tauri-apps/plugin-fs';

interface EmailAttachmentProps {
  attachments: Array<EmailAttachment>;
}

export default function EmailAttachmentComponent({ attachments }: EmailAttachmentProps) {
  async function createDownloadableAttachments(attachment: EmailAttachment) {
    const contents = new Uint8Array(attachment.content);
    await writeFile(attachment.filename, contents, { baseDir: BaseDirectory.Download });
  }

  function filename(filename: string) {
    const maxFilenameLength = 50;
    const filenameWithoutExtension = filename.split('.').slice(0, -1).join('.');
    return filenameWithoutExtension.length > maxFilenameLength
      ? filenameWithoutExtension.substring(0, maxFilenameLength) + '...'
      : filenameWithoutExtension;
  }

  function fileExtension(filename: string) {
    const extension = filename.split('.').pop();
    if (!extension) return '';
    return extension.toUpperCase();
  }

  return (
    <div className="attachments">
      {attachments.map((attachment, index) => {
        return (
          <div key={index} className="attachment" onClick={() => createDownloadableAttachments(attachment)}>
            <div className="icon-container">
              <div className="icon">📄</div>
              <div className="file-extension">{fileExtension(attachment.filename)}</div>
            </div>
            <div className="filename">{filename(attachment.filename)}</div>
          </div>
        );
      })}
    </div>
  );
}
