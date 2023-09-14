import { useLoaderData } from "react-router-dom";
import { Container, Table } from "@mantine/core";

export default function ApplicationLog(props) {
  const data = useLoaderData();
  //log_id, handler, message, created_at
  const rows = data.map((item) => {
    return (
      <tr key={item.log_id}>
        <td>{item.log_id}</td>
        <td>{item.handler}</td>
        <td>{item.message}</td>
        <td>{new Date(item.created_at).toLocaleDateString("en-us")}</td>
      </tr>
    );
  });
  return (
    <Container style={{ width: "100%" }}>
      <Table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Handler</th>
            <th>Message</th>
            <th>Date/Time</th>
          </tr>
        </thead>
        <tbody>{rows}</tbody>
      </Table>
    </Container>
  );
}

export async function loader() {
  const BASE_URL = "http://localhost";
  const LOGS_URL = "/util/logs";
  //localhost/util/logs
  const requestOptions = {
    method: "GET",
    headers: { "Content-type": "application/json" },
  };
  const response = await fetch(`${BASE_URL}${LOGS_URL}`, requestOptions);
  const results = await response.json();
  return results;
}
