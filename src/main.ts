import '@/assets/css/main.css';

import { createApp } from 'vue';
import { createI18n } from 'vue-i18n';
import { createMemoryHistory, createRouter } from 'vue-router';
import ui from '@nuxt/ui/vue-plugin';

import App from '@/App.vue';
import common from '@/i18n/locales/pt-BR/common.json';
import cpu from '@/i18n/locales/pt-BR/cpu.json';
import errors from '@/i18n/locales/pt-BR/errors.json';
import mainboard from '@/i18n/locales/pt-BR/mainboard.json';
import memory from '@/i18n/locales/pt-BR/memory.json';
import shell from '@/i18n/locales/pt-BR/shell.json';

const router = createRouter({
  history: createMemoryHistory(),
  routes: [],
});

const i18n = createI18n({
  legacy: false,
  locale: 'pt-BR',
  fallbackLocale: 'pt-BR',
  messages: {
    'pt-BR': { common, cpu, errors, mainboard, memory, shell },
  },
});

createApp(App).use(router).use(i18n).use(ui).mount('#app');
