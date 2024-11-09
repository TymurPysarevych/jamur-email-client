import './style.scss';
import EmailAttachmentComponent from '../attachments/EmailAttachmentComponent.tsx';
import EmailBody from '../body/EmailBody.tsx';
import { useRecoilValue } from 'recoil';
import { selectedEmailState } from '../../../state/atoms.ts';

export default function EmailDetailsComponent() {
  const { to, from, htmlBodies, textBodies, attachments, deliveredAt, subject, id } =
    useRecoilValue(selectedEmailState);

  function parseDate(deliveredAt: string) {
    const date = new Date(deliveredAt);
    return date.toLocaleString();
  }

  return (
    <div>
      <div className="email">
        <h1>{subject}</h1>
        <div>From: {from.join(', ')}</div>
        <div>To: {to.join(', ')}</div>
        <div>At: {parseDate(deliveredAt)}</div>
        <EmailBody bodies={htmlBodies} />
        <EmailAttachmentComponent attachments={attachments} />
      </div>
    </div>
  );
}
