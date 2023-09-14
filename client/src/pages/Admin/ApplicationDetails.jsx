import { useLoaderData } from "react-router-dom";
import { Alert, Container } from "@mantine/core";
import { TbMessage2Heart } from "react-icons/tb";
export default function ApplicationDetails(props) {
  const data = useLoaderData();
  return (
    <Container>
      <Alert
        icon={<TbMessage2Heart size="1rem" />}
        title="Server Details"
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
  const DETAILS_URL = "/util/health";
  //localhost/util/logs
  const requestOptions = {
    method: "GET",
    headers: { "Content-type": "application/json" },
  };
  const response = await fetch(`${BASE_URL}${DETAILS_URL}`, requestOptions);
  return response;
}
