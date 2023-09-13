import { AiOutlineControl } from "react-icons/ai";
import { BiTestTube } from "react-icons/bi";
import { RiHealthBookLine } from "react-icons/ri";
import { GoLog } from "react-icons/go";
import { useNavigate } from "react-router-dom";
import { Accordion, Navbar, NavLink } from "@mantine/core";

export default function (props) {
  //const navigate = useNavigate();
  return (
    <Navbar.Section grow mt="md">
      <Accordion>
        <Accordion.Item value="Admin">
          <Accordion.Control icon={<AiOutlineControl size={16} color="#909090"/>}>
            Admin Section
          </Accordion.Control>
          <Accordion.Panel>
            <NavLink
              label="Application Health"
              icon={<RiHealthBookLine size={16} color="red" />}
            />
            <NavLink
              label="Application Test"
              icon={<BiTestTube size={16} color="#0067A5" />}
            />
            <NavLink
              label="Application Logs"
              icon={<GoLog size={16} color="#B18C65" />}
            />
          </Accordion.Panel>
        </Accordion.Item>
      </Accordion>
    </Navbar.Section>
  );
}