import http from "k6/http";

const BASE_URI = "http://localhost:8080";

export default function () {
  const jsonData = { title: "test" };
  const headers = { "Content-Type": "application/json" };

  http.post("http://localhost:8080/api/task/create", JSON.stringify(jsonData), {
    headers,
  });
}
