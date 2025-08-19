import { Libra2FaucetClient } from "./Libra2FaucetClient";

test("node url empty", async () => {
  const client = new Libra2FaucetClient({BASE: "http://127.0.0.1:8081"});
  const response = await client.general.root();
  expect(response).toBe("tap:ok");
});
