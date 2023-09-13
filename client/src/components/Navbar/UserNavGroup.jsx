import { AiOutlineUser } from "react-icons/ai";
import { RiUser2Line } from "react-icons/ri";
import { GoLog } from "react-icons/go";
import { useNavigate } from "react-router-dom";
import { Accordion, Navbar, NavLink } from "@mantine/core";

export default function UserNavGroup(props) {
  return (
    <Navbar.Section grow mt="md">
      <Accordion>
        <Accordion.Item value="User">
          <Accordion.Control icon={<AiOutlineUser size={16} color="#00666D"/>}>
            User Control
          </Accordion.Control>
          <Accordion.Panel>
            <NavLink
              label="Create New User"
              icon={<RiUser2Line size={16} color="red" />}
            />
            <NavLink
              label="Delete User"
              icon={<RiUser2Line size={16} color="#005AFF" />}
            />
            <NavLink
              label="List Users"
              icon={<GoLog size={16} color="#B18C65" />}
            />
          </Accordion.Panel>
        </Accordion.Item>
      </Accordion>
    </Navbar.Section>
  );
}
