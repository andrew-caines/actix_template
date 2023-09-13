import { Footer, Group, Text } from "@mantine/core";
export default function AppFooter(props) {
  return (
    <Footer height={60} p="md">
      <Group position="center">
        <Text fw={500}> Andrew Caines 2023</Text> -{" "}
        <Text c="dimmed"><a href="mailto:andrew.p.caines@gmail.com">Email me!</a></Text>
      </Group>
    </Footer>
  );
}
