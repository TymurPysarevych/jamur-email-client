import './style.scss';
import { useRecoilState } from 'recoil';
import { snacksState } from '../../state/atoms.ts';
import { Alert, Slide, SlideProps, Snackbar } from '@mui/material';

export default function SnacksView() {
  const [snacks, setSnacks] = useRecoilState(snacksState);

  function SlideTransition(props: SlideProps) {
    return <Slide {...props} direction="up" />;
  }

  const { open, vertical, horizontal, message, severity } = snacks;

  function onClose() {
    setSnacks({ ...snacks, open: false });
  }

  return (
    <Snackbar
      autoHideDuration={5000}
      anchorOrigin={{ vertical, horizontal }}
      TransitionComponent={SlideTransition}
      open={open}
      onClose={onClose}
      key={vertical + horizontal}
    >
      <Alert severity={severity} variant="filled" sx={{ width: '100%' }}>
        {message}
      </Alert>
    </Snackbar>
  );
}
