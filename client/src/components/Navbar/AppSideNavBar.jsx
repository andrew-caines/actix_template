import { useContext } from "react";
import { AppShell } from "@mantine/core";

//Context
import { StateContext } from "../../context/state";
//Nav Groups
import AdminNavGroup from "./AdminNavGroup";
import SseNavGroup from "./SseNavGroup";
import UserNavGroup from "./UserNavGroup";
import PleaseLogin from "./PleaseLogin";
import AboutNavBar from "./AboutNavGroup";

export default function AppSideNavBar() {
  const { state, ..._ } = useContext(StateContext);
  //The Extra Section pushes the About to always be bottom Nav choice.
  return (
    <>
      <AppShell.Section grow width={{ base: 300 }} height={"99%"} p="xs">
        {state.isLoggedIn && <AdminNavGroup />}
        {state.isLoggedIn && <SseNavGroup />}
        {state.isLoggedIn && <UserNavGroup />}
        {!state.isLoggedIn && <PleaseLogin />}
      </AppShell.Section>
      <AppShell.Section>
        <AboutNavBar />
      </AppShell.Section>
    </>
  );
}
