import './style.scss';
import EmailAttachmentComponent from '../attachments/EmailAttachmentComponent.tsx';
import EmailBody from '../body/EmailBody.tsx';
import { useRecoilValue } from 'recoil';
import { localeState, selectedEmailState } from '../../../state/atoms.ts';

export default function EmailDetailsComponent() {
  const { to, from, htmlBodies, textBodies, attachments, deliveredAt, subject, id } =
    useRecoilValue(selectedEmailState);
  const locale = useRecoilValue(localeState);

  const parseDate = (deliveredAt: string) => {
    const date = new Date(deliveredAt);
    const formatter = new Intl.DateTimeFormat(locale, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: 'numeric',
      minute: 'numeric',
      second: 'numeric'
    });
    return formatter.format(date);
  };

  const bodies = () => {
    if (htmlBodies.length > 0) {
      return <EmailBody bodies={htmlBodies} />;
    } else if (textBodies.length > 0) {
      return <EmailBody bodies={textBodies} />;
    }
    return <div>---</div>;
  };

  const recipients = () => {
    if (to.length > 1) {
      return (
        <>
          <div className="bubble">{to[0]}</div>
          <div className="bubble">& {to.length - 1} more</div>
        </>
      );
    }
    return <div className="bubble">{to[0]}</div>;
  };

  return (
    <div>
      <div className="email">
        <div className="email-address-list">From: {from.join(', ')}</div>
        <h1>{subject}</h1>
        <div className="email-address-list">To: {recipients()}</div>
        <div>At: {parseDate(deliveredAt)}</div>
        {bodies()}
        <EmailAttachmentComponent attachments={attachments} />
      </div>
    </div>
  );
}
