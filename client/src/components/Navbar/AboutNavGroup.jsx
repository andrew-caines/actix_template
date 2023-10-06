import { useNavigate } from "react-router-dom";
import { Accordion, NavLink } from "@mantine/core";
import { FcAbout } from "react-icons/fc";
import { CgNotes } from "react-icons/cg";
import { TfiHelp } from "react-icons/tfi";

export default function AboutNavBar(props) {
  const navigate = useNavigate();
  return (
    <Accordion>
      <Accordion.Item value="About">
        <Accordion.Control icon={<FcAbout size={16} color="#F49739" />}>
          About
        </Accordion.Control>
        <Accordion.Panel>
          <NavLink
            label="Patch Notes"
            leftSection={<CgNotes size={16} color="#DBBE9C" />}
            onClick={() => navigate("/")}
          />
          <NavLink
            label="Help"
            leftSection={<TfiHelp size={16} color="#0067A5" />}
            onClick={() => navigate("/")}
          />
        </Accordion.Panel>
      </Accordion.Item>
    </Accordion>
  );
}
