const { spawnSync } = require('node:child_process');
const { resolve } = require('node:path');

const templateRoot = resolve(__dirname, '..');
const eslintBin = resolve(
  templateRoot,
  'node_modules',
  '.bin',
  process.platform === 'win32' ? 'eslint.cmd' : 'eslint',
);

const result = spawnSync(eslintBin, ['./src/**/*.{ts,tsx}'], {
  cwd: templateRoot,
  stdio: 'inherit',
  env: {
    ...process.env,
    ESLINT_USE_FLAT_CONFIG: 'false',
  },
});

if (result.error) throw result.error;

process.exit(result.status ?? 1);
