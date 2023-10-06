import { useContext, useState } from "react";
import { Group, Indicator, Text, Button, Burger } from "@mantine/core";
import { useNavigate } from "react-router-dom";
import { StateContext } from "../../context/state";
import { GoInbox } from "react-icons/go";
import LoginModal from "../Auth/LoginModal";
import Logo from "../../assets/Logo2.svg";
import "./AppHeader.css";

export default function AppHeader({ opened, toggle }) {
  const { state, login_user, logout_user } = useContext(StateContext);
  const [loginModalOpen, setLoginModalState] = useState(false);
  const navigate = useNavigate();

  return (
    <Group h="100%" px="md" justify="space-between">
      <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />
      <Group onClick={() => navigate("/")}>
        <img
          src={Logo}
          alt="Placeholder Logo"
          className="filter-ofxbase"
          height="32px"
          width="32px"
        />
        <Text fz="xl" fw={600}>
          IT-DASH
        </Text>
      </Group>
      <Group justify="flex-end" align="end">
        <Indicator
          color="#81BE83"
          inline
          label={`${state.it_ws_message.length}`}
          size={18}
        >
          <GoInbox size={24} color="#8FD8D8" />
        </Indicator>

        {state.isLoggedIn ? (
          <Button onClick={() => logout_user()}>Logout</Button>
        ) : (
          <Button onClick={() => setLoginModalState(true)} className="pulse-button">Login</Button>
        )}
      </Group>
      <LoginModal
        opened={loginModalOpen}
        close={setLoginModalState}
        login={login_user}
      />
    </Group>
  );
}
