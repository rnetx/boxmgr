import { createI18n } from 'vue-i18n';
// Languages
import enUS from './en-US.json';
import znCN from './zh-CN.json';
//

const messages = {
  'en-US': enUS.messages,
  'zh-CN': znCN.messages,
};

const locales = [
  {
    locale: 'en-US',
    name: 'English (United States)',
  },
  {
    locale: 'zh-CN',
    name: '简体中文 (Simplified Chinese)',
  },
];

const i18n = createI18n({
  legacy: false,
  locale: localStorage.getItem('locale') || 'en-US',
  fallbackLocale: 'en-US',
  messages,
});

export { i18n, locales };
