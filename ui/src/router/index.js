import { createRouter, createWebHistory } from 'vue-router';
import NProgress from 'nprogress';
import 'nprogress/nprogress.css';
import { checkSecret } from '@/api/secret';

NProgress.configure({
  easing: 'ease',
  speed: 500,
  showSpinner: false,
  trickleSpeed: 200,
  minimum: 0.3,
});

const routes = [
  {
    name: 'login',
    path: '/login',
    component: () => import('@/layout/Login.vue'),
  },
  {
    path: '/',
    redirect: 'home',
    component: () => import('@/layout/Global.vue'),
    children: [
      {
        name: 'home',
        path: '/home',
        component: () => import('@/components/Home.vue'),
      },
      {
        name: 'config',
        path: '/config',
        component: () => import('@/components/Config.vue'),
      },
      {
        name: 'script',
        path: '/script',
        component: () => import('@/components/Script.vue'),
      },
      {
        name: 'log',
        path: '/log',
        component: () => import('@/components/Log.vue'),
      },
      {
        name: 'setting',
        path: '/setting',
        component: () => import('@/components/Setting.vue'),
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach(async (to, from, next) => {
  NProgress.start();
  if (to.path !== '/login') {
    const secret = localStorage.getItem('secret');
    if (!secret) {
      localStorage.removeItem('secret');
      next('/login');
      return;
    }
    let err = await checkSecret(secret).then(
      (v) => null,
      (err) => err
    );
    if (err != null) {
      localStorage.removeItem('secret');
      next('/login');
      return;
    }
  }
  next();
});

router.afterEach(() => {
  NProgress.done();
});

export default router;
