<template>
  <div class="form">
    <el-form>
      <el-form-item :label="$t('setting.language')">
        <el-select v-model="locale" @change="setLocale">
          <el-option
            v-for="item in locales"
            :key="item.locale"
            :label="item.name"
            :value="item.locale"
          />
        </el-select>
      </el-form-item>
      <el-form-item :label="$t('setting.auto_start')">
        <el-switch
          v-model="autoStart"
          :loading="isAutoStartLoading"
          @change="setAutoStartHandle"
        />
      </el-form-item>
      <el-form-item :label="$t('setting.core_path')">
        <el-space>
          <el-input
            v-model="corePath"
            :placeholder="$t('generic.please_input')"
            :disabled="isCorePathLoading"
          />
          <el-button
            type="primary"
            :icon="Upload"
            circle
            @click="uploadClick"
          />
          <el-button
            type="success"
            :icon="Check"
            circle
            :loading="isCorePathLoading"
            @click="setCorePathHandle"
          />
        </el-space>
      </el-form-item>
      <el-form-item :label="$t('setting.exit')">
        <el-button
          type="danger"
          @click="requestToExitCheckDialogVisible = true"
        >
          {{ $t('setting.request_to_exit') }}
        </el-button>
      </el-form-item>
    </el-form>
  </div>

  <!-- upload core dialog -->
  <el-dialog
    v-model="uploadCoreDialogVisible"
    :title="$t('setting.upload_core_title')"
    width="40vh"
    top="20vh"
    destroy-on-close
    center
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    @closed="closedUploadCoreDialog"
  >
    <el-upload
      v-model:file-list="uploadCoreList"
      drag
      action="#"
      :limit="1"
      :auto-upload="false"
    >
      <el-icon>
        <UploadFilled />
      </el-icon>
      <div>
        {{ $t('setting.upload_info_prefix') }}
        {{ $t('setting.upload_info_or') }}
        <em>{{ $t('setting.upload_info_suffix') }}</em>
      </div>
    </el-upload>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="closeUploadCoreDialog">
          {{ $t('generic.cancel') }}
        </el-button>
        <el-button type="success" @click="uploadCoreHandle">
          {{ $t('setting.upload') }}
        </el-button>
      </div>
    </template>
  </el-dialog>

  <!-- request to exit check dialog -->
  <el-dialog
    v-model="requestToExitCheckDialogVisible"
    :title="$t('setting.request_to_exit_check_title')"
    width="500"
    center
    :close-on-click-modal="false"
    :close-on-press-escape="false"
  >
    <span>{{ $t('setting.request_to_exit_check_body') }}</span>
    <template #footer>
      <div class="dialog-footer">
        <el-button @click="requestToExitCheckDialogVisible = false">
          {{ $t('generic.cancel') }}
        </el-button>
        <el-button type="danger" @click="requestToExitCheckHandle">
          {{ $t('generic.confirm') }}
        </el-button>
      </div>
    </template>
  </el-dialog>

  <!-- exited dialog -->
  <el-dialog
    v-model="exitedDialogVisible"
    :title="$t('setting.exited')"
    width="500"
    center
    :close-on-click-modal="false"
    :close-on-press-escape="false"
    :show-close="false"
  />
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { Check, Upload, UploadFilled } from '@element-plus/icons-vue';
import { ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n';
import { locales } from '@/i18n';
import {
  setCorePath,
  uploadCore,
  getCorePath,
  setAutoStart,
  getAutoStart,
} from '@/api/service';
import { requestToExit } from '@/api/manager';

const { t, locale } = useI18n();

const setLocale = () => {
  localStorage.setItem('locale', locale.value);
};

const corePath = ref('');
const isCorePathLoading = ref(false);
const autoStart = ref(false);
const isAutoStartLoading = ref(false);
const uploadCoreDialogVisible = ref(false);
const uploadCoreList = ref([]);
const requestToExitCheckDialogVisible = ref(false);
const exitedDialogVisible = ref(false);

const refreshGetCorePath = () => {
  isCorePathLoading.value = true;
  getCorePath()
    .then((res) => {
      if (res) {
        corePath.value = res;
      }
      isCorePathLoading.value = false;
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('setting.get_core_path_failed', { err: err }),
      });
      console.log('get core path failed: ', err);
      isCorePathLoading.value = false;
    });
};

const refreshGetAutoStart = () => {
  isAutoStartLoading.value = true;
  getAutoStart()
    .then((res) => {
      if (res) {
        autoStart.value = res;
      }
      isAutoStartLoading.value = false;
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('setting.get_auto_start_failed', { err: err }),
      });
      console.log('get auto start failed: ', err);
      isAutoStartLoading.value = false;
    });
};

const uploadClick = () => {
  uploadCoreDialogVisible.value = true;
};

const closedUploadCoreDialog = () => {
  uploadCoreList.value = [];
};

const closeUploadCoreDialog = () => {
  uploadCoreDialogVisible.value = false;
};

const setCorePathHandle = () => {
  isCorePathLoading.value = true;
  setCorePath(corePath.value)
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('setting.set_core_path_success'),
      });
      refreshGetCorePath();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('setting.set_core_path_failed', { err: err }),
      });
      console.log('set core path failed: ', err);
      isCorePathLoading.value = false;
    });
};

const uploadCoreHandle = () => {
  const uploading = ElLoading.service({
    lock: true,
    text: t('setting.uploading'),
  });
  const file = uploadCoreList.value[0];
  const formData = new FormData();
  formData.append(file.name, file.raw);
  uploadCore(formData)
    .then(setCorePath)
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('setting.set_core_path_success'),
      });
      closeUploadCoreDialog();
      uploading.close();
      refreshGetCorePath();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('setting.set_core_path_failed', { err: err }),
      });
      console.log('set core path failed: ', err);
      closeUploadCoreDialog();
      uploading.close();
    });
};

const setAutoStartHandle = () => {
  isAutoStartLoading.value = true;
  setAutoStart(autoStart.value)
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('setting.set_auto_start_success'),
      });
      refreshGetAutoStart();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('setting.set_auto_start_failed', { err: err }),
      });
      console.log('set auto start failed: ', err);
      isAutoStartLoading.value = false;
    });
};

const requestToExitCheckHandle = () => {
  const exiting = ElLoading.service({
    lock: true,
    text: t('setting.exiting'),
  });
  requestToExitCheckDialogVisible.value = false;
  requestToExit()
    .then(() => {
      exiting.close();
      exitedDialogVisible.value = true;
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('setting.exit_failed', { err: err }),
      });
      console.log('exit failed: ', err);
      exiting.close();
    });
};

onMounted(() => {
  refreshGetAutoStart();
  refreshGetCorePath();
});
</script>

<style>
.form {
  display: flex;
  justify-content: center;
  margin: auto;
  margin-top: 5vh;
}
</style>
