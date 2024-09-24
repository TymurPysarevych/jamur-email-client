import { invoke, InvokeArgs } from '@tauri-apps/api/tauri';
import { useSetRecoilState } from 'recoil';
import { runningRequestsState } from '../state/atoms.ts';

export function useTauriInvoke<T>() {
  const set = useSetRecoilState(runningRequestsState);
  const invoker = (cmd: string, args?: InvokeArgs) => {
    set((currVal) => currVal + 1);
    const promise = invoke<T>(cmd, args);
    promise.finally(() => set((currVal) => currVal - 1));
    return promise;
  };

  return [invoker];
}
