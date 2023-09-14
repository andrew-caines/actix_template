import { Alert } from "@mantine/core";
import { BiCommentError } from "react-icons/bi";
import { useRouteError } from "react-router-dom";

export default function Error404(props) {
  let error = useRouteError();
  console.log(error);
  return (
    <>
      <Alert
        icon={<BiCommentError size="1rem" />}
        title="404 ERROR"
        color="red"
        radius="xl"
        style={{ maxWidth: "500px" }}
      >
        You attempted to Navigate to a Page that doesn't exist! Please navigate
        from menu options selectable on the LEFT side of the screen!
        {error}
      </Alert>
      <div className="arrow">
        <span></span>
        <span></span>
        <span></span>
      </div>
    </>
  );
}
