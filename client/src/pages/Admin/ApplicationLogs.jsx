import { useLoaderData } from "react-router-dom";
import { Table } from "@mantine/core";

export default function ApplicationLog(props) {
  const data = useLoaderData();
  
  const rows = data.map((item) => {
    return (
      <tr key={item.log_id}>
        <td>{item.log_id}</td>
        <td>{item.handler}</td>
        <td>{item.message}</td>
        <td>{new Date(item.created_at).toUTCString("dd,Mmm,yy hh:mm:ss ")}</td>
      </tr>
    );
  });
  return (
    <Table verticalSpacing="xs" fontSize="md">
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
