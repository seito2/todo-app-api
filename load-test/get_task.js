import http from "k6/http";

const BASE_URI = "http://localhost:8080";

export default function () {
  http.get(`${BASE_URI}/api/task/`);
}
