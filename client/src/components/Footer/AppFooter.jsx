import { Group, Text } from "@mantine/core";
export default function AppFooter(props) {
  return (
    <Group justify="center" align="flex-start">
      <Text fw={500}> Andrew Caines 2023</Text> -{" "}
      <Text c="dimmed">
        <a href="mailto:andrew.p.caines@gmail.com">Email me!</a>
      </Text>
    </Group>
  );
}
