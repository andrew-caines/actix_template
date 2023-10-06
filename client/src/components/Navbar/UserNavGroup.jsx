import { AiOutlineUser, AiOutlineUserAdd } from "react-icons/ai";
import { useNavigate } from "react-router-dom";
import { Accordion, NavLink } from "@mantine/core";

export default function UserNavGroup(props) {
  let navigate = useNavigate();
  return (
    <Accordion>
      <Accordion.Item value="User">
        <Accordion.Control icon={<AiOutlineUser size={16} color="#00666D" />}>
          User Control
        </Accordion.Control>
        <Accordion.Panel>
          <NavLink
            label="User Admin"
            leftSection={<AiOutlineUserAdd size={16} color="#183EFA" />}
            onClick={() => navigate("/user/admin")}
          />
        </Accordion.Panel>
      </Accordion.Item>
    </Accordion>
  );
}
