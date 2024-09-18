import { atom } from 'recoil';
import { KeychainEntry } from '../interfaces/KeychainEntry.ts';

export const runningRequestsState = atom<number>({
  key: 'runningRequestsState',
  default: 0
});

export const keychainEntriesState = atom<Array<KeychainEntry>>({
  key: 'keychainEntriesState',
  default: []
});
