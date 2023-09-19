import {
  AppShell,
} from "@mantine/core";


//Nav Groups
import AdminNavGroup from "./AdminNavGroup";
import SseNavGroup from "./SseNavGroup";
import UserNavGroup from "./UserNavGroup";

export default function AppSideNavBar() {
  return (
    <AppShell.Section width={{ base: 300 }} height={"95%"} p="xs">
      <AdminNavGroup />
      <SseNavGroup />
      <UserNavGroup />
    </AppShell.Section>
  );
}
