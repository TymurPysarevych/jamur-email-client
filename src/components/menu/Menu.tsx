import './style.scss';
import { useSetRecoilState } from 'recoil';
import { loadingState } from '../../state/atoms.ts';
import Button from '../../ui/button/Button.tsx';

export default function Menu() {
  const setLoadingState = useSetRecoilState(loadingState);

  const onClick = () => {
    setLoadingState((currVal) => !currVal);
  };

  return (
    <div>
      <Button onClick={onClick} text="test" icon="" />
    </div>
  );
}
