import './style.scss';
import { useRecoilValue, useSetRecoilState } from 'recoil';
import { emailsPreviewState, selectedEmailState } from '../../../state/atoms.ts';
import { useTauriInvoke } from '../../../utils/UseTauriInvoke.ts';
import { WebEmail } from '../../../interfaces/WebEmail.ts';

export default function EmailPreview() {
  const emails = useRecoilValue(emailsPreviewState);
  const setSelectedEmail = useSetRecoilState(selectedEmailState);
  const [fetchEmailById] = useTauriInvoke<WebEmail>();

  async function loadEmail(id: number) {
    setSelectedEmail({} as WebEmail);
    const email = await fetchEmailById('fetch_message_by_id', { id });
    setSelectedEmail(email);
  }

  return (
    <div className="email-preview-container">
      {emails.map((email) => (
        <div className="email-preview" key={email.id} onClick={() => loadEmail(email.id)}>
          <h2>{email.subject}</h2>
          <p>{email.from.join(', ')}</p>
          <p className="email-preview--body">{email.previewBody}</p>
        </div>
      ))}
    </div>
  );
}
