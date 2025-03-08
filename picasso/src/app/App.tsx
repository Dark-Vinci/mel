import { JSX } from 'react'
import {RouterProvider} from "react-router-dom";

import './App.scss'
import {router} from "../router";

export function App(): JSX.Element {
  return <RouterProvider router={router}/>
}
