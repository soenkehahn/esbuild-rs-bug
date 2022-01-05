import axios from "axios";

async function main() {
  const response = await axios("https://google.com");
  console.log(response.status);
}
main();
