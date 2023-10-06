


export default function UserAdmin(props){

    return (<>User Admin section</>)
}

export async function loader() {
    const BASE_URL = "http://localhost";
    const DETAILS_URL = "/users/admin";
    //localhost/util/logs
    const requestOptions = {
      method: "GET",
      headers: { "Content-type": "application/json" },
    };
    const response = await fetch(`${BASE_URL}${DETAILS_URL}`, requestOptions);
    return response;
  }

