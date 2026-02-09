import http from "k6/http";
import { type Options } from "k6/options";

export const options: Options = {
  duration: "30s",
  vus: 100,
};

const BASE_URL = __ENV.API_BASE_URL;

export default function () {
  http.get(`${BASE_URL}/schema`);
}
