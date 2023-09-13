import React from "react";
import ReactDOM from "react-dom/client";
import Root from "./Root.jsx";
import "./index.css";
import { MantineProvider } from "@mantine/core";
import { StateProvider } from "./context/state.jsx";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
/*
const router = createBrowserRouter([
  {
    path: "/",
    element: <div>Hello world!</div>,
  },
]);
<RouterProvider router={router} />
*/
ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <StateProvider>
      <MantineProvider withGlobalStyles withNormalizeCSS>
        <Root />
      </MantineProvider>
    </StateProvider>
  </React.StrictMode>
);
