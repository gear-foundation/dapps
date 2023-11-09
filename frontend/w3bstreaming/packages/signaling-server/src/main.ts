import { server } from './server';
import config from './config';
import { api, meta } from './contract';

const main = async () => {
  await api.isReadyOrError;
  console.log(`Connected to ${await api.chain()}`);
  await meta;
  console.log(`Metadata initialized`);
  console.log(meta);
  server.listen(config.port, () => {
    console.log(`Server is running on port ${config.port}`);
  });
};

main().catch(error => {
  console.log(error);
  process.exit(1);
});
