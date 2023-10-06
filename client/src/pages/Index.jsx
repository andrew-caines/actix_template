import { StateContext } from "../context/state";
import { useContext } from "react";

export default function Index(props) {
  const { state, ..._ } = useContext(StateContext);
  
  if (state.isLoggedIn) {
    return <>Please select a section on the left to begin</>;
  } else {
    return <>Please LOGIN to begin.</>;
  }
}
