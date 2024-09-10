import express from 'express';
import bodyParser from 'body-parser';
import { GaslessService } from './gasless-service';
import dotenv from 'dotenv';

export { GaslessService };

dotenv.config();

const app = express();
const gaslessService = new GaslessService();

app.use(bodyParser.json());

app.post('/issue', async function (req, res) {
  const data = req.body;
  const voucher = await gaslessService.issue(data.account, data.programId, data.amount, data.durationInSec);
  res.send(voucher);
});


app.post('/prolong', async function (req, res) {
  const data = req.body;
  await gaslessService.prolong(data.voucherId, data.account, data.balance, data.durationInSec);
  res.status(200);
});

app.post('/revoke', async function (req, res) {
  const data = req.body;
  await gaslessService.revoke(data.voucherId, data.account);
  res.status(200);
});

const port = process.env.PORT || 3000;
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
