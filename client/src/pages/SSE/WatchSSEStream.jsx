import { useState, useEffect } from "react";
import {
  Card,
  Group,
  ScrollArea,
  Text,
  Tooltip,
  useMantineTheme,
} from "@mantine/core";
import { TbPlugConnected, TbPlugConnectedX } from "react-icons/tb";
import { FaEraser } from "react-icons/fa6";

export default function WatchSSEStream(props) {
  const theme = useMantineTheme();
  const [connected, setConnected] = useState(false);
  const [messages, setMessages] = useState([]);

  useEffect(() => {
    //setup SSE listener
    const BASE_URL = "http://localhost";
    const SSE_STREAM = "/sse/general";
    const events = new EventSource(`${BASE_URL}${SSE_STREAM}`); //TODO make the URL present in State so this can be single-configured.
    setConnected(true);

    //Event handling
    events.onmessage = (message) => {
      if (message.data === "connected") {
        setMessages((prev) => [
          ...prev,
          `${new Date().toLocaleTimeString(
            "en-us"
          )} - You have connected to the General Feed.`,
        ]);
      } else {
        setMessages((prev) => [
          `${new Date().toLocaleTimeString("en-us")} - ${message.data}`,
          ...prev,
        ]); //Newsest Messages First
      }
    };
    return () => {
      //When component is unmounted, disconnect from SSE stream.
      setConnected(false);
      events.close();
    };
  }, []);
  const Message_Rows = () => {
    return messages.map((item, idx) => {
      return <li key={idx}>{item}</li>;
    });
  };
  return (
    <Card shadow="sm" padding="lg" radius="md" withBorder>
      <Card.Section>
        <Group justify="center">
          <Text size="lg">General SSE Feed. ({messages.length})</Text>
          {connected ? (
            <TbPlugConnected size={24} color={theme.colors.ofxbase[0]} />
          ) : (
            <TbPlugConnectedX size={24} color={theme.colors.red[7]} />
          )}
          <Tooltip label="Erase All Messages" position="bottom">
            <FaEraser
              size={24}
              color="Orchid"
              onClick={() => setMessages([])}
            />
          </Tooltip>
        </Group>
      </Card.Section>
      <ScrollArea h={"86vh"} type="auto">
        <ol style={{ listStyleType: "none" }}>
          <Message_Rows />
        </ol>
      </ScrollArea>
    </Card>
  );
}
