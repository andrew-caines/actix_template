import { useState } from "react";
import { Button, Center, Group, Textarea } from "@mantine/core";
import { GrSend } from "react-icons/gr";
import { send_to_general_SSE } from "../../context/sse_api";

export default function SendToSSEStream(props) {

  const [message, setMessage] = useState("");

  const send_message = async() => {
    //Take whatever message value is, and send it to endpoint!
    let resp = await send_to_general_SSE(message);
    if (resp.success) {
      setMessage(""); //Clear previous message
    } else {
      setMessage("Error sending Message, please try again.");
    }
  };

  return (
   
      <span>
        <Textarea
          value={message}
          onChange={(event) => setMessage(event.currentTarget.value)}
          size="xl"
          radius="xl"
          label="Send Message to General"
          description="Enter a message to see it in General SSE stream"
          placeholder="Message content..."
          w={"100%"}
        />
        <Button
          radius="xl"
          size="xl"
          fullWidth
          rightSection={<GrSend size={24} />}
          onClick={() => send_message()}
        >
          Send
        </Button>
      </span>
    
  );
}
