import { AlertColor, SnackbarOrigin } from '@mui/material';

export interface Snacks extends SnackbarOrigin {
  open: boolean;
  message: string;
  severity: AlertColor;
}
