import { defineConfig } from '@hey-api/openapi-ts';

export default defineConfig({
  input: './openapi.json',
  output: {
    path: './src/generated',
  },
  client: '@hey-api/client-fetch',
  services: {
    asClass: true,
  },
});
