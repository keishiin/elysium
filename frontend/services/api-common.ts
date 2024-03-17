import axios from "axios";

const apiClient = axios.create({
  baseURL: "http://127.0.0.1:8080/",
  headers: {
    "Content-Type": "application/json",
  },
  withCredentials: true,
});

apiClient.interceptors.response.use(
  (response) => {
    if (response.config.method === "post") {
      const customHeader = response.headers["authorization"];
      const customHeader2 = response.headers["axum-accountid"];

      localStorage.setItem("token", customHeader);
      localStorage.setItem("user", customHeader2);
    }
    return response;
  },
  (error) => {
    return Promise.reject(error);
  },
);

export default apiClient;