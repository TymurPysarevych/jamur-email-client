import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { RecoilRoot } from 'recoil';
import LoadingComponent from './components/loading/LoadingComponent.tsx';
import { ThemeProvider } from '@mui/material';
import theme from './ui/theme.ts';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <RecoilRoot>
      <ThemeProvider theme={theme}>
        <div>
          <LoadingComponent />
          <App />
        </div>
      </ThemeProvider>
    </RecoilRoot>
  </React.StrictMode>
);
