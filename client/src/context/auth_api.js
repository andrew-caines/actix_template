const BASE_URL = `http://localhost`; //TODO make this setable by end user, maybe in .env
const LOGIN_URL = `/v1/auth/login`;
const LOGOUT_URL = `/v1/auth/logout`;

export const LoginUser = async (username, password) => {
  //This function will reach out and try to login, given a username/password string.
  try {
    const requestOptions = {
      method: "POST",
      headers: { Authorization: `Basic ${btoa(username + ":" + password)}` },
    };

    const response = await fetch(`${BASE_URL}${LOGIN_URL}`, requestOptions);
    const token = await response.json();
    let result = { success: true, message: "Login Successfull", token };
    return result;
  } catch (err) {
    console.log(`[Login] Got Error. ${JSON.stringify(err)}`);
    return { success: false, message: "Server Error Logging in." };
  }
};

export const LogoutUser = async () => {
  //This function will poke the URL from current user, server should invaldiate Token and all other instances related to this user.
  const requestOptions = {
    method: "POST",
  };
  try {
    const response = await fetch(`${BASE_URL}${LOGOUT_URL}`, requestOptions);
    const res = await response.json();
    return res;
  } catch (err) {
    console.log(`[Login] Got Error. ${JSON.stringify(err)}`);
    return { success: false, message: JSON.stringify(err) };
  }
};
