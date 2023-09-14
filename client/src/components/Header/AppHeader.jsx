import { useContext } from "react";
import { Header, Group, Text, Button } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { StateContext } from "../../context/state";
import LoginModal from "../Auth/LoginModal";
import Logo from "../../assets/Logo2.svg";
import "./AppHeader.css";

export default function AppHeader(props) {
  const { state, login_user, logout_user } = useContext(StateContext);
  const [opened, { open, close }] = useDisclosure(false);

  return (
    <Header height={60} p="xs">
      <Group position="apart">
        <Group>
          <img
            src={Logo}
            className="filter-indigo"
            alt="Placeholder Logo"
            height="32px"
            width="32px"
            style={{ color: "red" }}
          />
          <Text fz="xl" fw={600}>
            APP NAME
          </Text>
        </Group>
        <Group position="right">
          {state.isLoggedIn ? (
            <Button onClick={() => logout_user()}>Logout</Button>
          ) : (
            <Button onClick={() => open()}>Login</Button>
          )}
        </Group>
      </Group>
      <LoginModal opened={opened} close={close} login={login_user} />
    </Header>
  );
}
