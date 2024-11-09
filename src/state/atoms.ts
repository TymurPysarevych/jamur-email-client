import { atom } from 'recoil';
import { KeychainEntry } from '../interfaces/KeychainEntry.ts';
import { WebEmailPreview } from '../interfaces/Email.ts';
import { Snacks } from '../interfaces/Snacks.ts';

export const runningRequestsState = atom<number>({
  key: 'runningRequestsState',
  default: 0
});

export const keychainEntriesState = atom<Array<KeychainEntry>>({
  key: 'keychainEntriesState',
  default: []
});

export const emailsPreviewState = atom<Array<WebEmailPreview>>({
  key: 'emailsPreviewState',
  default: []
});

export const snacksState = atom<Snacks>({
  key: 'snacksState',
  default: {
    open: false,
    vertical: 'top',
    horizontal: 'center',
    message: '',
    severity: 'success'
  }
});
