import { BiGlassesAlt,BiMailSend } from "react-icons/bi";
import { CiStreamOn } from "react-icons/ci";
import { useNavigate } from "react-router-dom";
import { Accordion, Navbar, NavLink } from "@mantine/core";

export default function SseNavGroup(props) {
  return (
    <Navbar.Section grow mt="md">
      <Accordion>
        <Accordion.Item value="SSE">
          <Accordion.Control icon={<CiStreamOn size={16} color="#D0342C"/>}>
            SSE Section
          </Accordion.Control>
          <Accordion.Panel>
            <NavLink
              label="Watch General SSE Stream"
              icon={<BiGlassesAlt size={16} color="#CBA3B2" />}
            />
            <NavLink
              label="Send Message to General"
              icon={<BiMailSend size={16} color="#599F68" />}
            />
          </Accordion.Panel>
        </Accordion.Item>
      </Accordion>
    </Navbar.Section>
  );
}
