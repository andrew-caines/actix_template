import { AiOutlineUser } from "react-icons/ai";
import { RiUser2Line } from "react-icons/ri";
import { GrUserAdmin } from "react-icons/gr";
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
            leftSection={<GrUserAdmin size={16} color="red" />}
            onClick={() => navigate("/user/admin")}
          />
         
          
        </Accordion.Panel>
      </Accordion.Item>
    </Accordion>
  );
}

/*

<NavLink
label="List Users"
icon={<GoLog size={16} color="#B18C65" />}
/>
<NavLink
label="Delete User"
icon={<RiUser2Line size={16} color="#005AFF" />}
/>

*/