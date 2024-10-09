import './style.scss';
import { imapEmailsState, keychainEntriesState } from '../../state/atoms.ts';
import { useRecoilValue, useSetRecoilState } from 'recoil';
import { Email, Folder, WebFolders } from '../../interfaces/Email.ts';
import { useTauriInvoke } from '../../utils/UseTauriInvoke.ts';
import { KEYCHAIN_KEY_IMAP } from '../../interfaces/KeychainEntry.ts';
import { GEmail } from '../../interfaces/GEmail.ts';
import { useEffect, useState } from 'react';
import { SimpleTreeView, TreeItem } from '@mui/x-tree-view';

export default function Menu() {
  const keychainEntries = useRecoilValue(keychainEntriesState);
  const setImapEmails = useSetRecoilState(imapEmailsState);
  const [fetchImapMessages] = useTauriInvoke<Array<Email>>();
  const [fetchGmailMessages] = useTauriInvoke<Array<GEmail>>();
  const [fetchImapFolders] = useTauriInvoke<WebFolders>();
  const [subfolderMap, setSubfolderMap] = useState<Map<string, WebFolders>>(new Map<string, WebFolders>());
  const [selectedFolder, setSelectedFolder] = useState<string>('');

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

  // useEffect(() => {
  //   if (selectedFolder) {
  //     keychainEntries
  //       .filter((e) => e.key.startsWith(KEYCHAIN_KEY_IMAP))
  //       .forEach((entry) =>
  //         fetchImapMessages('fetch_imap_messages', { keychainEntry: entry, folder: selectedFolder }).then((emails) => {
  //           setImapEmails(emails);
  //         })
  //       );
  //   }
  // }, [selectedFolder]);

  const buildFoldersForEachEntry = () => {
    return Array.from(subfolderMap.keys()).map((parent) => {
      const webFolders = subfolderMap.get(parent);
      if (!webFolders) {
        return <></>;
      }

      const buildFolder = (folder: Folder) => {
        return (
          <TreeItem
            onClick={() => setSelectedFolder(folder.folderName)}
            itemId={folder.folderName}
            label={folder.folderName}
          >
            {folder.children.map((child) => buildFolder(child))}
          </TreeItem>
        );
      };
      return (
        <SimpleTreeView>
          <TreeItem itemId={parent} label={parent}>
            {webFolders.folders.map((folder) => buildFolder(folder))}
          </TreeItem>
        </SimpleTreeView>
      );
    });
  };

  return <div className="menu-container">{buildFoldersForEachEntry()}</div>;
}
