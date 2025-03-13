import { createBrowserRouter } from 'react-router-dom';

import { Login } from '@pages';
import { ToggleTheme } from '@containers';

export const router = createBrowserRouter([
  {
    path: '/melon',
    element: <div>
      <div>welcome to rslack app</div>
      <ToggleTheme />
    </div>,
  },
  {
    path: 'auth',
    element: <Login />,
  },
  {
    path: '*',
    element: <div> 404 page </div>,
  },
]);
