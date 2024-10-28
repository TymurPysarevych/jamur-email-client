import './style.scss';
import { imapEmailsState, keychainEntriesState } from '../../state/atoms.ts';
import { useRecoilValue, useSetRecoilState } from 'recoil';
import { Folder, WebEmailPreview, WebFolders } from '../../interfaces/Email.ts';
import { useTauriInvoke } from '../../utils/UseTauriInvoke.ts';
import { KEYCHAIN_KEY_IMAP } from '../../interfaces/KeychainEntry.ts';
import { useEffect, useState } from 'react';
import { SimpleTreeView, TreeItem } from '@mui/x-tree-view';
import { listen } from '@tauri-apps/api/event';

export default function Menu() {
  const keychainEntries = useRecoilValue(keychainEntriesState);
  const setImapEmails = useSetRecoilState(imapEmailsState);
  const [fetchImapMessages] = useTauriInvoke();
  // const [fetchGmailMessages] = useTauriInvoke<Array<GEmail>>();
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

  useEffect(() => {
    if (selectedFolder) {
      keychainEntries
        .filter((e) => e.key.startsWith(KEYCHAIN_KEY_IMAP))
        .forEach(async (entry) => {
          setImapEmails([]);
          await listen<Array<WebEmailPreview>>('new_emails', (event) => {
            setImapEmails(event.payload);
          });
          fetchImapMessages('fetch_messages', {
            keychainEntry: entry,
            folder: selectedFolder
          }).then(() => {});
        });
    }
  }, [selectedFolder]);

  const buildFoldersForEachEntry = () => {
    return Array.from(subfolderMap.keys()).map((parent) => {
      const webFolders = subfolderMap.get(parent);
      if (!webFolders) {
        return <></>;
      }

      const buildFolder = (folder: Folder) => {
        return (
          <TreeItem
            onClick={() => setSelectedFolder(folder.fullPath)}
            itemId={folder.fullPath}
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
