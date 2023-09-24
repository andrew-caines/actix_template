//This worker connects to a Websocket endpoint and forwards messages back that will update ContextState for the Application.

const it_channel_socket = new WebSocket("ws://localhost/ws/it");

it_channel_socket.onopen = (event) => {
  console.log(`A Connection to the I.T Channel is complete.`);
};

it_channel_socket.onmessage = (event) => {
  let payload = JSON.parse(event.data);
  switch (payload.type) {
    case "WS_MESSAGE":
      postMessage({
        type: "it_channel_message",
        payload: payload.message.trim(),
      });
      break;
    case "WS_CONNECTION_NEWUSER":
      console.log(`Got NEW USER joined channel message!`);
      break;
    case "WS_DISCONNECTED_USER":
      console.log(`Got USER DISCONNECTED message! (update online count)`);
      break;
    case "WS_CONNECTION_SUCCESS":
      console.log(
        `Got USER CONNECTION successful (get socket-id and update online count)!`
      );
      break;
    default:
      console.log(`Got an untyped Message on Websocket Channel.${event.data}`);
      break;
  }
};

onmessage = (msg) => {
  console.log(`Web Worker Got Message of: ${msg} from main-thread`);
};
