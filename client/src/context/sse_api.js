const BASE_URL = `http://localhost`;
const SSE_GENERAL_URL = "/sse/general";
const SSE_GENERAL_SEND = "/sse/test_message";

//TODO build connecting to SSE endpoint.
export const connect_to_general_SSE = () => {};

export const send_to_general_SSE = async (message) => {
  //http://localhost/sse/test_message
  try {
    const requestOptions = {
      method: "POST",
      headers: { "Content-type": "application/json" },
      body: JSON.stringify({ message: message }),
    };
    const response = await fetch(
      "http://localhost/sse/test_message",
      requestOptions
    );
    if (response.status === 200) {
      return { success: true, message: "Message Sent" };
    } else {
      return { success: false, message: response.statusText };
    }
  } catch (e) {
    console.log(`(sse_api.js)[send_to_general_SSE] Got Error: ${e}`);
    return { success: false, message: `${e}` };
  }
};
