import { invoke, InvokeArgs } from '@tauri-apps/api/tauri';
import { useSetRecoilState } from 'recoil';
import { runningRequestsState } from '../state/atoms.ts';

export function useTauriInvoke<T>(cmd: string, args?: InvokeArgs) {
  const set = useSetRecoilState(runningRequestsState);
  const invoker = () => {
    set((currVal) => currVal + 1);
    const promise = invoke<T>(cmd, args);
    promise.finally(() => set((currVal) => currVal - 1));
    return promise;
  };

  return [invoker];
}
