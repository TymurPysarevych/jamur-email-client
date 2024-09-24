// import './style.scss';
// import EmailAttachmentComponent from './attachments/EmailAttachmentComponent.tsx';
// import EmailBody from './body/EmailBody.tsx';
// import { useState } from 'react';
// import { Email } from '../../interfaces/Email.ts';
// import { useTauriInvoke } from '../../utils/UseTauriInvoke.ts';
//
// export default function EmailComponent() {
//   const [emails, setEmails] = useState<Array<Email>>([]);
//   const [invokeFetchEmails] = useTauriInvoke<Array<Email>>('fetch_messages', { server: '', login: '', password: '' });
//
//   const fetchEmails = async () => {
//     await invokeFetchEmails()
//       .then((response) => {
//         setEmails(response);
//         console.log('Emails: ', emails.length);
//       })
//       .catch((e) => console.error(e));
//   };
//
//   return (
//     <div>
//       <button onClick={fetchEmails}>Fetch Emails</button>
//       {emails.map((email: Email) => (
//         <div className="email">
//           <h1>{email.subject}</h1>
//           <div>From: {email.from.join(', ')}</div>
//           <div>To: {email.to.join(', ')}</div>
//           <div>At: {email.deliveredAt}</div>
//           <EmailBody bodies={email.bodies} />
//           <EmailAttachmentComponent attachments={email.attachments} />
//         </div>
//       ))}
//     </div>
//   );
// }
