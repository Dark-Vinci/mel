import {createBrowserRouter} from "react-router-dom";

import {Login} from "@pages";

export const router = createBrowserRouter([
    {
        path: '/',
        element: <div>welcome to rslack app</div>,
    },
    {
        path: "auth",
        element: <Login />,
    },
    {
        path: '*',
        element: <div> 404 page </div>
    }
]);
