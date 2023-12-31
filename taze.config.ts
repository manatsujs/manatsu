import { defineConfig } from 'taze';

export default defineConfig({
  force: true,
  write: true,
  includeLocked: true,
  recursive: true,
  packageMode: {
    vue: 'ignore',
    '@vitejs/plugin-vue': 'ignore'
  }
});
