<template>
  <div class="login-wrap">
    <el-switch
      v-model="isDark"
      class="theme-btn"
      style="margin-left: 24px"
      inline-prompt
      :active-icon="Moon"
      :inactive-icon="Sunny"
      @change="toggleDark"
    />
    <div class="back-wrap">
      <div class="bg-item left one"></div>
      <div class="bg-item right two"></div>
      <div class="bg-item left three"></div>
      <div class="bg-item right four"></div>
    </div>
    <div class="login-container">
      <div class="login-title">
        <img style="width: 40px; height: 40px" src="/sing-box.svg" />
        {{ $t('login.title') }}
      </div>
      <el-form v-model="loginForm" class="form-wrap" autocomplete="off">
        <el-form-item prop="secret">
          <el-input
            class="login-input"
            v-model="loginForm.secret"
            :placeholder="$t('login.secret_placeholder')"
            show-password
            :prefix-icon="Lock"
          ></el-input>
        </el-form-item>
      </el-form>
      <el-button
        class="login-btn"
        type="primary"
        :loading="loginLoading"
        @click="loginHandle"
        >{{ $t('login.login_button') }}</el-button
      >
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { Lock, Sunny, Moon } from '@element-plus/icons-vue';
import { useDark, useToggle } from '@vueuse/core';
import { ElMessage } from 'element-plus';
import { checkSecret } from '@/api/secret';
import { useRouter } from 'vue-router';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const isDark = useDark();
const toggleDark = useToggle(isDark);
const router = useRouter();

const loginForm = ref({
  secret: '',
});
const loginLoading = ref(false);

const loginHandle = () => {
  loginLoading.value = true;
  checkSecret(loginForm.value.secret)
    .then(() => {
      localStorage.setItem('secret', loginForm.value.secret);
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('login.login_success'),
      });
      loginLoading.value = false;
      router.push('/');
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('login.login_failed', { err: err }),
      });
      console.log('login failed: ' + err);
      loginLoading.value = false;
    });
};

const enterHandle = (e) => {
  if (e.keyCode == 13 || e.keyCode == 100) {
    loginHandle();
  }
};

onMounted(() => {
  window.addEventListener('keydown', enterHandle);
});

onUnmounted(() => {
  window.addEventListener('keydown', enterHandle, false);
});
</script>

<style>
input:-internal-autofill-selected {
  background-color: transparent !important;
  background-image: none !important;
  color: rgb(255, 255, 255) !important;
}
input:-webkit-autofill,
input:-webkit-autofill:hover,
input:-webkit-autofill:focus,
input:-webkit-autofill:active {
  transition-delay: 500000s;
  transition: background-color 50000s ease-out;
  -webkit-transition-delay: 50000s;
  -webkit-transition: background-color 50000s ease-out;
}
</style>

<style scoped lang="scss">
.login-wrap {
  position: absolute;
  top: 0;
  bottom: 0;
  right: 0;
  left: 0;
  overflow: hidden;
  background-color: var(--el-color-primary-light-5);
  display: flex;
  align-items: center;
  justify-content: center;
}

.theme-btn {
  position: absolute;
  right: 10vh;
  top: 50px;
  z-index: 9;
}

@media (max-width: 55vh) {
  .theme-btn {
    right: 5vh;
  }
}

@media (max-width: 40vh) {
  .theme-btn {
    width: 3vh;
  }
}

// 背景
.back-wrap {
  position: absolute;
  top: 0;
  bottom: 0;
  right: 0;
  left: 0;
  z-index: 1;
  background: var(--el-color-primary-light-9);
  .bg-item {
    position: absolute;
    &.left {
      bottom: 0;
      left: 0;
      filter: drop-shadow(5px 0 20px rgba(0, 0, 0, 0.1));
    }
    &.right {
      bottom: 0;
      right: 0;
      filter: drop-shadow(-5px 0 20px rgba(0, 0, 0, 0.2));
    }
    &.one {
      border-bottom: 50vh solid var(--el-color-primary-light-3);
      border-right: 60vw solid transparent;
      z-index: 6;
    }
    &.two {
      border-bottom: 70vh solid var(--el-color-primary-light-5);
      border-left: 80vw solid transparent;
      z-index: 5;
    }
    &.three {
      border-bottom: 90vh solid var(--el-color-primary-light-7);
      border-right: 90vw solid transparent;
      z-index: 4;
    }
    &.four {
      border-bottom: 110vh solid var(--el-color-primary-light-8);
      border-left: 110vw solid transparent;
      z-index: 3;
    }
  }
}

.login-container {
  padding: 20px;
  margin-left: 10px;
  margin-right: 10px;
  position: relative;
  z-index: 2;
}

.login-title {
  color: var(--color-primary);
  font-size: 48px;
}

.form-wrap {
  width: 100%;
  margin: 0 auto;
  margin-top: 5vh;
  color: var(--color-text-2);
  :deep(.el-input__wrapper) {
    border-radius: 5px;
    border: 1px solid var(--el-color-primary-light-3);
    outline: none;
    box-shadow: none;
    &.is-focus,
    &:hover {
      box-shadow: 0 0 0 1px var(--el-color-primary) inset !important;
    }
  }
  :deep(.el-input__prefix) {
    color: var(--color-text-2);
  }
  :deep(.el-input__inner) {
    // color: var(--color-text-2);
    // outline: none;
    height: 40px;
  }
}

.other-wrap {
  width: 400px;
  margin: 0 auto;
  margin-top: 10px;
  text-align: right;
}

.login-btn {
  width: 50vh;
  height: 40px;
  // font-size: 16px;
  display: block;
  margin: 20px auto;
}

@media (max-width: 55vh) {
  .login-btn {
    width: 40vh;
  }
}

@media (max-width: 40vh) {
  .login-btn {
    width: 30vh;
  }
}
</style>
