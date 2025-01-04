import { locale } from '@tauri-apps/plugin-os';

export async function Locale() {
  return await locale();
}
