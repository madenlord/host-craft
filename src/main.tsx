import React from 'react'
import ReactDOM from 'react-dom/client'
import {
  createBrowserRouter,
  createRoutesFromElements,
  Route,
  RouterProvider
} from 'react-router-dom';

import Index, {
  loader as indexLoader
} from './pages/index';
import Settings from './pages/settings';
import SettingsServer, {
  loader as serverLoader
} from './pages/settingsServer';

import './style/globals.css';

const router = createBrowserRouter(
  createRoutesFromElements(
    <>
      <Route
        path='/'
        element={<Index />}
        loader={indexLoader}
      />
      <Route
        path='/settings'
        element={<Settings />}
      />
      <Route
        path='/settings/server'
        element={<SettingsServer />}
        loader={serverLoader}
      />
    </>
  )
);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
)
