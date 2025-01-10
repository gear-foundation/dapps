import { server } from "./server";
import config from "./config";
import { api } from "./contract";

const main = async () => {
  await api.isReadyOrError;
  console.log(`Connected to ${await api.chain()}`);
  server.listen(config.port, () => {
    console.log(`Server is running on port ${config.port}`);
  });
};

main().catch((error) => {
  console.log(error);
  process.exit(1);
});
