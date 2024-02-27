import { createApp } from 'vue';
import App from './App.vue';
import router from './router';
import { i18n } from './i18n';
import { useDark } from '@vueuse/core';
import 'element-plus/theme-chalk/dark/css-vars.css';
import 'element-plus/theme-chalk/el-message.css';
import 'element-plus/theme-chalk/el-loading.css';

useDark();

createApp(App).use(i18n).use(router).mount('#app');
