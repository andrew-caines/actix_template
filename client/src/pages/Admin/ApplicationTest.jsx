import { useLoaderData } from "react-router-dom";
import { Alert, Container } from "@mantine/core";
import { GiEchoRipples } from "react-icons/gi";

export default function ApplicationTest(props) {
  const data = useLoaderData();
  return (
    <Container>
      <Alert
        icon={<GiEchoRipples size={24}/>}
        title="Server Echo Test"
        color="blue"
        radius="xl"
      >
        {data}
      </Alert>
    </Container>
  );
}

export async function loader() {
  const BASE_URL = "http://localhost";
  const ECHO_URL = "/util/echo";
  //localhost/util/logs
  const requestOptions = {
    method: "GET",
    headers: { "Content-type": "application/text" },
  };
  const response = await fetch(`${BASE_URL}${ECHO_URL}`, requestOptions);
  return response;
}
