import { Alert } from "@mantine/core";
import { BiCommentError } from "react-icons/bi";
import { useRouteError } from "react-router-dom";

export default function GeneralError(props) {
  let error = useRouteError();
  console.log(error);
  return (
    <Alert icon={<BiCommentError size="1rem" />} title="Rutt Roh" color="red">
      Something went wrong with loading/presenting this Page. Email Andrew what
      you were doing when you saw this following Error:
      <br />
      
      <br />
      Thank you
    </Alert>
  );
}
