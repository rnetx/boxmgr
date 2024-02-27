<template>
  <div>
    <el-container>
      <el-header>
        <el-menu router mode="horizontal">
          <el-dropdown trigger="click" @command="commandHandle">
            <el-menu-item>
              <img style="width: 32px; height: 32px" src="/sing-box.svg" />
            </el-menu-item>
            <template #dropdown>
              <el-dropdown-menu>
                <el-dropdown-item>
                  <el-text style="margin-right: 10px">
                    {{ $t('global.dark_theme') }}
                  </el-text>
                  <el-switch
                    v-model="isDark"
                    class="theme-btn"
                    inline-prompt
                    :active-icon="Moon"
                    :inactive-icon="Sunny"
                    @change="toggleDark"
                  />
                </el-dropdown-item>
                <el-dropdown-item command="logout">
                  <el-button link>
                    <el-icon>
                      <SwitchButton />
                    </el-icon>
                    {{ $t('global.logout') }}
                  </el-button>
                </el-dropdown-item>
              </el-dropdown-menu>
            </template>
          </el-dropdown>
          <el-menu-item
            index="home"
            :class="$route.name == 'home' ? 'is-active' : ''"
          >
            <template #title>
              <el-icon>
                <HomeFilled />
              </el-icon>
              <span> {{ $t('global.home') }} </span>
            </template>
          </el-menu-item>
          <el-menu-item
            index="config"
            :class="$route.name == 'config' ? 'is-active' : ''"
          >
            <template #title>
              <el-icon>
                <Files />
              </el-icon>
              <span> {{ $t('global.config') }} </span>
            </template>
          </el-menu-item>
          <el-menu-item
            index="script"
            :class="$route.name == 'script' ? 'is-active' : ''"
          >
            <template #title>
              <el-icon>
                <Document />
              </el-icon>
              <span> {{ $t('global.script') }} </span>
            </template>
          </el-menu-item>
          <el-menu-item
            index="log"
            :class="$route.name == 'log' ? 'is-active' : ''"
          >
            <template #title>
              <el-icon>
                <Tickets />
              </el-icon>
              <span> {{ $t('global.log') }} </span>
            </template>
          </el-menu-item>
          <el-menu-item
            index="setting"
            :class="$route.name == 'setting' ? 'is-active' : ''"
          >
            <template #title>
              <el-icon>
                <Setting />
              </el-icon>
              <span> {{ $t('global.setting') }} </span>
            </template>
          </el-menu-item>
        </el-menu>
      </el-header>
      <el-container>
        <el-main>
          <router-view />
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<script setup>
import {
  HomeFilled,
  Document,
  Tickets,
  Files,
  Setting,
  SwitchButton,
  Moon,
  Sunny,
} from '@element-plus/icons-vue';
import { useDark, useToggle } from '@vueuse/core';
import { useRouter } from 'vue-router';

const isDark = useDark();
const toggleDark = useToggle(isDark);
const router = useRouter();

const commandHandle = (command) => {
  switch (command) {
    case 'logout':
      localStorage.removeItem('secret');
      router.push('/login');
      break;
  }
};
</script>

<style>
.container {
  padding-top: 0%;
  height: 100%;
}

.el-header {
  position: fixed;
  width: 100%;
  z-index: 1000; /* 适当的 z-index 值，确保在其他元素之上 */
}

.el-main {
  position: absolute;
  top: 60px; /* 适当的距离，确保 el-main 不会被 el-header 遮挡 */
  left: 0;
  right: 0;
  bottom: 0;
  overflow-y: auto;
}

.el-menu {
  justify-content: center;
}

.flex-grow {
  flex-grow: 1;
}
</style>
