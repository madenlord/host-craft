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
import Welcome, {
  action as welcomeAction
} from './pages/welcome';
import Home, {
  loader as homeLoader
} from './pages/home';
import Settings from './pages/settings';
import SettingsServer, {
  loader as serverLoader,
  action as serverAction
} from './pages/settingsServer';
import SettingsRepo, {
  loader as repoLoader,
  action as repoAction
} from './pages/settingsRepo';
import ServerExecution from './pages/run';

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
        path='/welcome'
        element={<Welcome />}
        action={welcomeAction}
      />
      <Route
        path='/home'
        element={<Home />}
        loader={homeLoader}
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
      <Route
        path='/run'
        element={<ServerExecution />}
      />
    </>
  )
);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>,
)
