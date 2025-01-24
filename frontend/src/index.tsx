/* @refresh reload */
import { lazy } from 'solid-js';
import { render } from 'solid-js/web';
import { Router, Route } from '@solidjs/router';
import { AuthProvider } from './store/Auth';
import './index.css';

import App from './App';

// Lazy imports
const Login = lazy(() => import("./pages/auth/Login"))
const NotFound = lazy(() => import("./pages/NotFound"))

const root = document.getElementById('root') as HTMLElement;

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    'Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?',
  );
}

render(() =>
  <AuthProvider>
    <Router>
      <Route path="/" component={App} />
      <Route path="*404" component={NotFound} />
      <Route path="/login" component={Login} />
    </Router>
  </AuthProvider>,
  root);
