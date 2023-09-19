import { AppShell } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { Outlet } from "react-router-dom";

//Navigation Components
import AppSideNavBar from "./components/Navbar/AppSideNavBar";
import AppHeader from "./components/Header/AppHeader";
import AppFooter from "./components/Footer/AppFooter";

function Root() {
  const [opened, { toggle }] = useDisclosure();

  return (
    <AppShell
      header={{ height: 50 }}
      footer={{ height: 35 }}
      navbar={{ width: 300, breakpoint: "sm", collapsed: { mobile: !opened } }}
      aside={{
        width: 300,
        breakpoint: "md",
        collapsed: { desktop: false, mobile: true },
      }}
      padding="md"
    >
      <AppShell.Header>
        <AppHeader opened={opened} toggle={toggle} />
      </AppShell.Header>
      <AppShell.Navbar p="xs">
        <AppSideNavBar />
      </AppShell.Navbar>
      <AppShell.Main>
        <Outlet />
      </AppShell.Main>
      <AppShell.Footer p="xs">
        <AppFooter />
      </AppShell.Footer>
    </AppShell>
  );
}

export default Root;
