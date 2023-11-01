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

    const customHeader = response.headers['authorization'];
    if (customHeader) {
      localStorage.setItem("token", customHeader)
    }
    return response;
  },
  (error) => {
    return Promise.reject(error);
  }
);

export default apiClient;