import './style.scss';
import { useRecoilValue } from 'recoil';
import { emailsPreviewState } from '../../../state/atoms.ts';

export default function EmailPreview() {
  const emails = useRecoilValue(emailsPreviewState);

  return (
    <div className="email-preview-container">
      {emails.map((email) => (
        <div className="email-preview" key={email.id}>
          <h2>{email.subject}</h2>
          <p>{email.from.join(', ')}</p>
          <p className="email-preview--body">{email.previewBody}</p>
        </div>
      ))}
    </div>
  );
}
