import { useState } from "react";
import { AppShell, Container } from "@mantine/core";
import { Outlet } from "react-router-dom";
import SideNavBar from "./components/Navbar/AppSideNavBar";
import AppHeader from "./components/Header/AppHeader";
import AppFooter from "./components/Footer/AppFooter";
import SideNavToggler from "./components/Navbar/SideNavToggle";

function Root() {
  const [sideMenuOpened, setToggleSideMenu] = useState(true);

  return (
    <AppShell
      padding="md"
      navbar={
        sideMenuOpened ? (
          <SideNavBar opened={sideMenuOpened} toggle={setToggleSideMenu} />
        ) : (
          <SideNavToggler toggle={setToggleSideMenu} />
        )
      }
      header={<AppHeader />}
      footer={<AppFooter />}
      style={{ height: "90vh" }}
      styles={(theme) => ({
        main: {
          backgroundColor:
            theme.colorScheme === "dark"
              ? theme.colors.dark[8]
              : theme.colors.gray[0],
        },
      })}
    >
      <Container style={{minWidth:"100%"}}>
        <Outlet />
      </Container>
    </AppShell>
  );
}

export default Root;

//
