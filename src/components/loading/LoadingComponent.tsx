import './style.scss';
import { useRecoilValue } from 'recoil';
import { loadingState } from '../../state/atoms.ts';
import { DNA } from 'react-loader-spinner';
import { useAutoAnimate } from '@formkit/auto-animate/react';

export default function LoadingComponent() {
  const loading = useRecoilValue<boolean>(loadingState);
  const [parent] = useAutoAnimate({
    duration: 200
  });
  return (
    <div ref={parent}>
      <DNA visible={loading} height="60" width="80" wrapperClass={'loading-container'} />
    </div>
  );
}
