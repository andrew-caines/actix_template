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
import ApplicationDetails, {
  loader as app_details_loader,
} from "./pages/Admin/ApplicationDetails.jsx";
import ApplicationTest, {
  loader as app_test_loader,
} from "./pages/Admin/ApplicationTest.jsx";
import GeneralError from "./pages/Error/GeneralError.jsx";
import Error404 from "./pages/Error/Error404.jsx";

const router = createBrowserRouter(
  createRoutesFromElements(
    <Route path="/" element={<Root />}>
      <Route index element={<Index />} />
      <Route
        path="/application_details"
        element={<ApplicationDetails />}
        loader={app_details_loader}
        errorElement={<GeneralError />}
      />
       <Route
        path="/application_test"
        element={<ApplicationTest />}
        loader={app_test_loader}
        errorElement={<GeneralError />}
      />
      <Route
        path="/application_logs"
        element={<ApplicationLog />}
        loader={app_log_loader}
        errorElement={<GeneralError />}
      />
     
      <Route path="*" element={<Error404 />} />
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
