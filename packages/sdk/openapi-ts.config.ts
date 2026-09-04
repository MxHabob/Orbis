import { defineConfig } from '@hey-api/openapi-ts';

export default defineConfig({
  input: '../../apps/api-rust/openapi.json',
  output: {
    path: './src',
  },
  client: '@hey-api/client-fetch',
});
