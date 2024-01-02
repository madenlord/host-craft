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
  loader as serverLoader,
  action as serverAction
} from './pages/settingsServer';
import SettingsRepo, {
  loader as repoLoader,
  action as repoAction
} from './pages/settingsRepo';

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
        action={serverAction}
      />
      <Route
        path='/settings/repo'
        element={<SettingsRepo />}
        loader={repoLoader}
        action={repoAction}
      />
    </>
  )
);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
)
