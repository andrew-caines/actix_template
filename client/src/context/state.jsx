import React, { useReducer, useEffect, useMemo } from "react";
import state_reducer from "./state_reducers.js";
import { LoginUser, LogoutUser } from "./auth_api.js";

//Setup Intital State Configuration.
const initalState = {
  isLoggedIn: false,
  applicationName: "APP NAME",
  token: "",
  username: "",
};

//Create Context
export const StateContext = React.createContext();

export const StateProvider = (props) => {
  const [state, dispatch] = useReducer(state_reducer, initalState);

  useEffect(() => {
    //One time run on Boot of Context
    console.log(`🌎 - Global State initialized.`);
  }, []);

  const login_user = async (username, password) => {
    let result = await LoginUser(username, password);
    if (result.success) {
      //Set the token in Global state, then send success back to login modal.
      dispatch({ type: "set_is_loggedIn", payload: true });
      dispatch({ type: "set_jwt_token", payload: result.token });
      return { success: true, message: "Login Complete." };
    } else {
      return { success: false, message: "Login information invalid" };
    }
  };

  const logout_user = () => {
    dispatch({ type: "set_is_loggedIn", payload: false });
  };

  const value = useMemo(() => {
    //Return state and functions that should be able to passed via context
    return { state, login_user, logout_user };
  }, [state]);

  return (
    <StateContext.Provider value={value}>
      {props.children}
    </StateContext.Provider>
  );
};
