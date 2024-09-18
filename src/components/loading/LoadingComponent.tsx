import './style.scss';
import { DNA } from 'react-loader-spinner';
import { useAutoAnimate } from '@formkit/auto-animate/react';
import { useRecoilValue } from 'recoil';
import { runningRequestsState } from '../../state/atoms.ts';

export default function LoadingComponent() {
  const runningRequests = useRecoilValue<number>(runningRequestsState);

  const loading = runningRequests > 0;

  const [parent] = useAutoAnimate({
    duration: 200
  });

  return (
    <div ref={parent}>
      <DNA visible={loading} height="60" width="80" wrapperClass={'loading-container'} />
    </div>
  );
}
