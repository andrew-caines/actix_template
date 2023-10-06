import { NavLink } from "@mantine/core";
import { BiLogIn } from "react-icons/bi";

export default function PleaseLogin() {
  return (
    <NavLink
      label="Please Login to begin"
      leftSection={<BiLogIn size={16} color="red" />}
    />
  );
}
