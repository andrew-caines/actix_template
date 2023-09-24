export default function state_reducer(state, action) {
  switch (action.type) {
    case "it_channel_message":
      let msgs = [...state.it_ws_message];
      msgs.push(action.payload.message);
      return { ...state, it_ws_message: msgs };
    case "set_app_name":
      return { ...state, applicationName: action.payload };
    case "set_is_loggedIn":
      return { ...state, isLoggedIn: action.payload };
    case "set_jwt_token":
      return { ...state, token: action.payload };
    default:
      return state;
  }
}
