import { AiOutlineControl } from "react-icons/ai";
import { BiTestTube } from "react-icons/bi";
import { RiHealthBookLine } from "react-icons/ri";
import { GoLog } from "react-icons/go";
import { Accordion,NavLink } from "@mantine/core";
import { useNavigate } from "react-router-dom";

export default function (props) {
  const navigate = useNavigate();
  return (
    <Accordion>
      <Accordion.Item value="Admin">
        <Accordion.Control
          icon={<AiOutlineControl size={16} color="#909090" />}
        >
          Admin Section
        </Accordion.Control>
        <Accordion.Panel>
          <NavLink
            label="Application Details"
            icon={<RiHealthBookLine size={16} color="red" />}
            onClick={() => navigate("/application_details")}
          />
          <NavLink
            label="Application Test"
            icon={<BiTestTube size={16} color="#0067A5" />}
            onClick={() => navigate("/application_test")}
          />
          <NavLink
            label="Application Logs"
            icon={<GoLog size={16} color="#B18C65" />}
            onClick={() => navigate("/application_logs?limit=25&offset=0")}
          />
        </Accordion.Panel>
      </Accordion.Item>
    </Accordion>
  );
}
