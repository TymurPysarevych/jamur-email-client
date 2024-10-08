import './style.scss';
import { imapEmailsState, keychainEntriesState } from '../../state/atoms.ts';
import { useRecoilValue, useSetRecoilState } from 'recoil';
import { Email, Folder, WebFolders } from '../../interfaces/Email.ts';
import { useTauriInvoke } from '../../utils/UseTauriInvoke.ts';
import { KEYCHAIN_KEY_IMAP } from '../../interfaces/KeychainEntry.ts';
import { GEmail } from '../../interfaces/GEmail.ts';
import { useEffect, useState } from 'react';

export default function Menu() {
  const keychainEntries = useRecoilValue(keychainEntriesState);
  const setImapEmails = useSetRecoilState(imapEmailsState);
  const [fetchImapMessages] = useTauriInvoke<Array<Email>>();
  const [fetchGmailMessages] = useTauriInvoke<Array<GEmail>>();
  const [fetchImapFolders] = useTauriInvoke<WebFolders>();
  const [subfolderMap, setSubfolderMap] = useState<Map<string, WebFolders>>(new Map<string, WebFolders>());

  useEffect(() => {
    keychainEntries
      .filter((e) => e.key.startsWith(KEYCHAIN_KEY_IMAP))
      .forEach((entry) =>
        fetchImapFolders('fetch_imap_folders', { keychainEntry: entry }).then((folder) => {
          setSubfolderMap((oldMap) => {
            const map = new Map(oldMap);
            map.set(entry.id, folder);
            return map;
          });
        })
      );
  }, [keychainEntries]);

  const buildFoldersForEachEntry = () => {
    return Array.from(subfolderMap.keys()).map((parent) => {
      const folders = subfolderMap.get(parent);
      if (!folders) {
        return <></>;
      }

      const buildFolder = (folder: Folder, index: number) => {
        return (
          <>
            <div key={`${folder}${index}`} className="menu-container--folder">
              <div className="menu-container--folder__name">{folder.folderName}</div>
              {folder.children.map((child, index) => buildFolder(child, index))}
            </div>
          </>
        );
      };

      return (
        <>
          <div key={parent} className="menu-container--entry">
            <h1 className="hover">{parent}</h1>
            {folders.folders.map((folder, index) => buildFolder(folder, index))}
          </div>
        </>
      );
    });
  };

  return <div className="menu-container">{buildFoldersForEachEntry()}</div>;
}
