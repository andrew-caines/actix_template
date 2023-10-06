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

//Mantine Base CSS
import "@mantine/core/styles.css";

//Custom Colors
import custom_colors from "./custom_colors.json";

//Pages
import Index from "./pages/Index.jsx";
import Root from "./Root.jsx";

//Common Pages
import GeneralError from "./pages/Error/GeneralError.jsx";
import Error404 from "./pages/Error/Error404.jsx";

//Admin Section Pages
import ApplicationLog, {
  loader as app_log_loader,
} from "./pages/Admin/ApplicationLogs.jsx";
import ApplicationDetails, {
  loader as app_details_loader,
} from "./pages/Admin/ApplicationDetails.jsx";
import ApplicationTest, {
  loader as app_test_loader,
} from "./pages/Admin/ApplicationTest.jsx";

//SSE Section Pages
import SendToSSEStream from "./pages/SSE/SendToSSEStream.jsx";
import WatchSSEStream from "./pages/SSE/WatchSSEStream.jsx";

//User Control Pages
import UserAdmin from "./pages/User/UserAdmin.jsx";

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
      <Route
        path="/sse/general"
        element={<WatchSSEStream />}
        errorElement={<GeneralError />}
      />
      <Route
        path="/sse/sendToAll"
        element={<SendToSSEStream />}
        errorElement={<GeneralError />}
      />
      <Route
        path="/user/admin"
        element={<UserAdmin />}
        errorElement={<GeneralError />}
      />
      <Route path="*" element={<Error404 />} />
    </Route>
  )
);

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <StateProvider>
      <MantineProvider
        withGlobalStyles
        withNormalizeCSS
        theme={{ colors: custom_colors }}
        defaultColorScheme="dark"
      >
        <RouterProvider router={router} />
      </MantineProvider>
    </StateProvider>
  </React.StrictMode>
);
