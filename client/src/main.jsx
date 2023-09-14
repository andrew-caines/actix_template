import React from "react";
import ReactDOM from "react-dom/client";
import { MantineProvider } from "@mantine/core";
import { StateProvider } from "./context/state.jsx";
import {
  createRoutesFromElements,
  createBrowserRouter,
  RouterProvider,
  Route,
} from "react-router-dom";
import "./index.css";

//Pages
import Index from "./pages/Index.jsx";
import Root from "./Root.jsx";
import ApplicationLog, {
  loader as app_log_loader,
} from "./pages/Admin/ApplicationLogs.jsx";

const router = createBrowserRouter(
  createRoutesFromElements(
    <Route path="/" element={<Root />}>
      <Route index element={<Index />} />
      <Route
        path="/applicationlogs"
        element={<ApplicationLog />}
        loader={app_log_loader}
        errorElement={<div>OhOh</div>}
      />
    </Route>
  )
);

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <StateProvider>
      <MantineProvider withGlobalStyles withNormalizeCSS>
        <RouterProvider router={router} />
      </MantineProvider>
    </StateProvider>
  </React.StrictMode>
);
