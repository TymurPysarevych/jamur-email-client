import './style.scss';
import { useRecoilValue } from 'recoil';
import { imapEmailsState } from '../../../state/atoms.ts';

export default function EmailPreview() {
  const emails = useRecoilValue(imapEmailsState);

  const toLongString = (body: string = '', length: number) => {
    if (body.length > length) {
      return body.substring(0, length) + '...';
    }
    return body;
  };

  return (
    <div className="email-preview-container">
      {emails.map((email) => (
        <div className="email-preview" key={email.id}>
          <h2>{toLongString(email.subject, 50)}</h2>
          <p>{email.from.join(', ')}</p>
          <p className="email-preview--body">{toLongString(email.textBodies[0], 150)}</p>
        </div>
      ))}
    </div>
  );
}
