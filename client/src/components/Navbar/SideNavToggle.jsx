import { Group } from "@mantine/core";
import { BsArrowLeftRight } from "react-icons/bs";

export default function SideNavToggler({ toggle }) {
  return (
    <Group
      style={{ width: "32px" }}
      position="center"
      onClick={() => toggle((prev) => !prev)}
    >
      <BsArrowLeftRight size={16} />
    </Group>
  );
}
