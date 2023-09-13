import { Collapse, Group, Navbar } from "@mantine/core";

//Nav Util
import SideNavToggler from "./SideNavToggle";

//Nav Groups
import AdminNavGroup from "./AdminNavGroup";
import SseNavGroup from "./SseNavGroup";
import UserNavGroup from "./UserNavGroup";

export default function AppSideNavBar({ opened, toggle }) {
  return (
    <Collapse
      in={opened}
      transitionDuration={500}
      transitionTimingFunction="linear"
    >
      <Navbar width={{ base: 300 }} height={"95%"} p="xs">
        <Navbar.Section>
          <Group position="right">
            <SideNavToggler toggle={toggle} />
          </Group>
        </Navbar.Section>
        <Navbar.Section>
          <AdminNavGroup />
        </Navbar.Section>
        <Navbar.Section>
          <SseNavGroup />
        </Navbar.Section>
        <Navbar.Section>
          <UserNavGroup />
        </Navbar.Section>
      </Navbar>
    </Collapse>
  );
}
