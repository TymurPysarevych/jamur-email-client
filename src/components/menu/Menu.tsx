import './style.scss';
import { keychainEntriesState } from '../../state/atoms.ts';
import { useRecoilValue } from 'recoil';

export default function Menu() {
  const keychainEntries = useRecoilValue(keychainEntriesState);
  return (
    <div>
      <h1>Menu</h1>
      <ul>
        {keychainEntries.map((entry) => (
          <li key={`${entry.key}-${entry.id}`}>{entry.id}</li>
        ))}
      </ul>
    </div>
  );
}
