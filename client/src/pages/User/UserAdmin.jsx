import { useContext, useEffect, useState } from "react";
import { StateContext } from "../../context/state";

export default function UserAdmin(props) {
  const { state, ..._ } = useContext(StateContext);
  const [all_users, set_All_Users] = useState([]);

  useEffect(() => {
    //Note, currently you can not access Context in a route loader, so this will be manually added here.
    //Future state: have token in context, and then pre-load data with the token set from Context. 

    async function load() {
      const URL = "http://localhost/users/list";
      const requestOptions = {
        method: "GET",
        headers: {
          Authorization: `Bearer ${state.token}`,
        },
      };
      const request = await fetch(URL, requestOptions);
      let results = await request.json();
      console.log(results);
      set_All_Users(results);
    }
    
    load();
  }, [state]);

  return <>User Admin section {all_users.count}</>;
}