import { useState } from "react";
import {
  useLoaderData,
  useFetcher,
  useLocation,
  useSearchParams,
} from "react-router-dom";
import { Table, Pagination } from "@mantine/core";

export default function ApplicationLog(props) {
  //data {count:N,logs:[]}
  const [activePage, setPage] = useState(1);
  const data = useLoaderData();
  const fetcher = useFetcher();
  const location = useLocation();
  let [searchParams, setSearchParams] = useSearchParams();

  const handlePagination = (page) => {
    //Set the current page, and load the data with those offsets.
    const offset = (page - 1) * 25; //25 items per page. page 3 = offset 75 LIMIT 25
    //Refetch the data for the selected page
    setSearchParams(
      new URLSearchParams({
        limit: 25,
        offset: offset,
      })
    );
    fetcher.load(`${location.pathname}/?${searchParams}`);
    setPage(page);
  };

  const rows = data.logs?.map((item) => {
    return (
      <Table.Tr key={item.log_id}>
        <Table.Td>{item.log_id}</Table.Td>
        <Table.Td>{item.handler}</Table.Td>
        <Table.Td>{item.message}</Table.Td>
        <Table.Td>
          {new Date(item.created_at).toUTCString("dd,Mmm,yy hh:mm:ss ")}
        </Table.Td>
      </Table.Tr>
    );
  });

  return (
    <Table
      striped
      fontSize="xs"
      verticalSpacing="xs"
    >
      <Table.Thead>
        <Table.Tr>
          <Table.Th>ID</Table.Th>
          <Table.Th>Handler</Table.Th>
          <Table.Th>Message</Table.Th>
          <Table.Th>Date/Time</Table.Th>
        </Table.Tr>
      </Table.Thead>
      <Table.Tbody>{rows}</Table.Tbody>
      <Table.Tfoot>
        <Table.Tr>
          <Table.Td>
            <Pagination
              value={activePage}
              onChange={handlePagination}
              siblings={2}
              total={Math.floor(data.count / 25) + 1}
            />
          </Table.Td>
        </Table.Tr>
      </Table.Tfoot>
    </Table>
  );
}

export async function loader({ request }) {
  //localhost/util/logs  25 entries per page
  const LIMIT = 25;
  const url = new URL(request.url);
  const BASE_URL = "http://localhost";
  const LOGS_URL = "/util/logs";

  const searchParams = new URLSearchParams({
    offset: url.searchParams.get("offset"),
    limit: LIMIT,
  });
  const requestOptions = {
    method: "GET",
    headers: { "Content-type": "application/json" },
  };
  const response = await fetch(
    `${BASE_URL}${LOGS_URL}?${searchParams}`,
    requestOptions
  );
  const results = await response.json();
  return results;
}
