import { BiGlassesAlt, BiMailSend } from "react-icons/bi";
import { CiStreamOn } from "react-icons/ci";
import { useNavigate } from "react-router-dom";
import { Accordion, NavLink } from "@mantine/core";

export default function SseNavGroup(props) {
  let navigate = useNavigate();
  return (
    <Accordion>
      <Accordion.Item value="SSE">
        <Accordion.Control icon={<CiStreamOn size={16} color="#D0342C" />}>
          SSE Section
        </Accordion.Control>
        <Accordion.Panel>
          <NavLink
            label="Watch General SSE Stream"
            icon={<BiGlassesAlt size={16} color="#CBA3B2" />}
            onClick={() => navigate("/sse/general")}
          />
          <NavLink
            label="Send Message to General"
            icon={<BiMailSend size={16} color="#599F68" />}
            onClick={() => navigate("/sse/sendToAll")}
          />
        </Accordion.Panel>
      </Accordion.Item>
    </Accordion>
  );
}
