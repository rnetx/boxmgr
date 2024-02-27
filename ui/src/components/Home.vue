<template>
  <div>
    <!-- Desktop -->
    <div v-if="!isMobile">
      <el-card>
        <el-row :gutter="5">
          <el-col :span="6">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">{{ $t('home.status') }}</h3>
              <p class="card-text">
                <el-text :type="status_is_running_style">
                  {{ status_is_running }}
                </el-text>
              </p>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">{{ $t('home.running_config') }}</h3>
              <p class="card-text">
                <el-text>
                  {{ status_running_config }}
                </el-text>
              </p>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">{{ $t('home.memory') }}</h3>
              <p class="card-text">
                <el-text>
                  {{ status_memory }}
                </el-text>
              </p>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">{{ $t('home.connection_count') }}</h3>
              <p class="card-text">
                <el-text>
                  {{ status_connection_count }}
                </el-text>
              </p>
            </el-card>
          </el-col>
        </el-row>
        <el-row :gutter="5">
          <el-col :span="6">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">
                {{ $t('home.upload_traffic') }}
              </h3>
              <p class="card-text">
                <el-text>
                  {{ status_upload_traffic }}
                </el-text>
              </p>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">
                {{ $t('home.download_traffic') }}
              </h3>
              <p class="card-text">
                <el-text>
                  {{ status_download_traffic }}
                </el-text>
              </p>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">
                {{ $t('home.upload_speed') }}
              </h3>
              <p class="card-text">
                <el-text>
                  {{ status_upload_speed }}
                </el-text>
              </p>
            </el-card>
          </el-col>
          <el-col :span="6">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">
                {{ $t('home.download_speed') }}
              </h3>
              <p class="card-text">
                <el-text>
                  {{ status_download_speed }}
                </el-text>
              </p>
            </el-card>
          </el-col>
        </el-row>
      </el-card>
    </div>

    <!-- Mobile -->
    <div v-if="isMobile">
      <el-card>
        <el-row :gutter="5">
          <el-col :span="12">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">{{ $t('home.status') }}</h3>
              <p class="card-text">
                <el-text :type="status_is_running_style">
                  {{ status_is_running }}
                </el-text>
              </p>
            </el-card>
          </el-col>
          <el-col :span="12">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">{{ $t('home.running_config') }}</h3>
              <p class="card-text">
                <el-text>
                  {{ status_running_config }}
                </el-text>
              </p>
            </el-card>
          </el-col>
        </el-row>
        <el-row :gutter="5">
          <el-col :span="12">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">{{ $t('home.memory') }}</h3>
              <p class="card-text">
                <el-text>
                  {{ status_memory }}
                </el-text>
              </p>
            </el-card>
          </el-col>
          <el-col :span="12">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">{{ $t('home.connection_count') }}</h3>
              <p class="card-text">
                <el-text>
                  {{ status_connection_count }}
                </el-text>
              </p>
            </el-card>
          </el-col>
        </el-row>
        <el-row :gutter="5">
          <el-col :span="12">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">
                {{ $t('home.upload_traffic') }}
              </h3>
              <p class="card-text">
                <el-text>
                  {{ status_upload_traffic }}
                </el-text>
              </p>
            </el-card>
          </el-col>
          <el-col :span="12">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">
                {{ $t('home.download_traffic') }}
              </h3>
              <p class="card-text">
                <el-text>
                  {{ status_download_traffic }}
                </el-text>
              </p>
            </el-card>
          </el-col>
        </el-row>
        <el-row :gutter="5">
          <el-col :span="12">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">
                {{ $t('home.upload_speed') }}
              </h3>
              <p class="card-text">
                <el-text>
                  {{ status_upload_speed }}
                </el-text>
              </p>
            </el-card>
          </el-col>
          <el-col :span="12">
            <el-card class="status-card" shadow="hover">
              <h3 class="card-text">
                {{ $t('home.download_speed') }}
              </h3>
              <p class="card-text">
                <el-text>
                  {{ status_download_speed }}
                </el-text>
              </p>
            </el-card>
          </el-col>
        </el-row>
      </el-card>
    </div>

    <el-space class="button">
      <el-button
        type="primary"
        :disabled="operationLoading || status_is_running === false"
        @click="startClick"
      >
        {{ $t('home.start') }}
      </el-button>
      <el-button
        type="danger"
        :disabled="operationLoading || status_is_running === true"
        @click="stopClick"
      >
        {{ $t('home.stop') }}
      </el-button>
      <el-button
        type="warning"
        :disabled="operationLoading || status_is_running === true"
        @click="restartClick"
      >
        {{ $t('home.restart') }}
      </el-button>
    </el-space>
    <p></p>
    <div class="form">
      <el-form label-position="right" label-width="100px">
        <el-form-item :label="$t('home.config')">
          <el-select
            v-model="configSelected"
            :placeholder="$t('generic.please_select')"
            :disabled="configSelectLoading"
            @change="setConfig"
            style="width: 25vh"
          >
            <el-option
              v-for="item in configList"
              :key="item.id"
              :label="item.tag"
              :value="item.id"
            />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('home.core_version')">
          <el-text>
            {{ status_core_version }}
          </el-text>
        </el-form-item>
      </el-form>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { ElMessage } from 'element-plus';
import { useI18n } from 'vue-i18n';
import { listConfig } from '@/api/config';
import { setActiveConfig } from '@/api/config';
import { startService, stopService, restartService } from '@/api/service';
import { getWebsocketPrefix } from '@/api/axios';

const { t } = useI18n();

const isMobile = ref(false);

const status_is_running = ref(t('home.collecting'));
const status_is_running_style = ref('');
const status_running_config = ref(t('home.collecting'));
const status_core_version = ref(t('home.collecting'));
//
const status_memory = ref(t('home.collecting'));
const status_connection_count = ref(t('home.collecting'));
const status_upload_traffic = ref(t('home.collecting'));
const status_download_traffic = ref(t('home.collecting'));
const status_upload_speed = ref(t('home.collecting'));
const status_download_speed = ref(t('home.collecting'));
//
const operationLoading = ref(false);
const configList = ref([]);
const configSelectLoading = ref(false);
const configSelected = ref('');

const getBytesStr = (b) => {
  let b1 = b / 1024;
  if (b1 < 1024) {
    return b1.toFixed(2) + ' KB';
  }
  let b2 = b1 / 1024;
  if (b2 < 1024) {
    return b2.toFixed(2) + ' MB';
  }
  let b3 = b2 / 1024;
  if (b3 < 1024) {
    return b3.toFixed(2) + ' GB';
  }
  let b4 = b3 / 1024;
  return b4.toFixed(2) + ' TB';
};

const statusHandle = (obj) => {
  if (obj === null) {
    status_is_running.value = t('home.collecting');
    status_running_config.value = t('home.collecting');
    status_is_running_style.value = '';
    status_core_version.value = t('home.collecting');
    status_memory.value = t('home.collecting');
    status_connection_count.value = t('home.collecting');
    status_upload_traffic.value = t('home.collecting');
    status_download_traffic.value = t('home.collecting');
    status_upload_speed.value = t('home.collecting');
    status_download_speed.value = t('home.collecting');
    return;
  }
  if (obj.is_running === true) {
    status_is_running.value = t('home.running');
    status_is_running_style.value = 'success';
    status_running_config.value = obj.running_config;
  } else if (obj.is_running === false) {
    status_is_running.value = t('home.stopped');
    status_is_running_style.value = 'danger';
    status_running_config.value = t('home.stopped');
  }
  status_core_version.value = obj.core_version;
  status_memory.value = getBytesStr(obj.memory_usage);
  status_connection_count.value = obj.connection_count;
  status_upload_traffic.value = getBytesStr(obj.upload_traffic);
  status_download_traffic.value = getBytesStr(obj.download_traffic);
  status_upload_speed.value = getBytesStr(obj.upload_speed) + '/s';
  status_download_speed.value = getBytesStr(obj.download_speed) + '/s';
};

const refreshConfigList = () => {
  configSelectLoading.value = true;
  listConfig()
    .then((res) => {
      configList.value = res;
      let actived = res.find((item) => item.actived);
      if (actived) {
        configSelected.value = actived.id;
      } else {
        configSelected.value = '';
      }
      configSelectLoading.value = false;
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('generic.get_config_list_failed', { err: err }),
      });
      console.log('get config list failed: ', err);
      configSelectLoading.value = false;
    });
};

const setConfig = () => {
  configSelectLoading.value = true;
  setActiveConfig(configSelected.value)
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('generic.set_active_config_success'),
      });
      configSelectLoading.value = false;
      refreshConfigList();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('generic.set_active_config_failed', { err: err }),
      });
      console.log('set active config failed: ', err);
      configSelectLoading.value = false;
    });
};

// Operate

const startClick = () => {
  operationLoading.value = true;
  startService()
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('home.start_service_success'),
      });
      operationLoading.value = false;
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('home.start_service_failed', { err: err }),
      });
      console.log('start service failed: ', err);
      operationLoading.value = false;
    });
};

const stopClick = () => {
  operationLoading.value = true;
  stopService()
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('home.stop_service_success'),
      });
      operationLoading.value = false;
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('home.stop_service_failed', { err: err }),
      });
      console.log('stop service failed: ', err);
      operationLoading.value = false;
    });
};

const restartClick = () => {
  operationLoading.value = true;
  restartService()
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('home.restart_service_success'),
      });
      operationLoading.value = false;
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('home.restart_service_failed', { err: err }),
      });
      console.log('restart service failed: ', err);
      operationLoading.value = false;
    });
};

let ws = null;

const openWebsocketChannel = () => {
  let websocket = null;
  let prefix = getWebsocketPrefix();
  websocket = new WebSocket(
    `${prefix}/api/v1/service/status?secret=${localStorage.getItem('secret')}`
  );
  websocket.onopen = () => {
    console.log('websocket connected');
  };
  websocket.onmessage = (event) => {
    statusHandle(JSON.parse(event.data));
  };
  websocket.onclose = () => {
    console.log('websocket closed');
    statusHandle(null);
    setTimeout(() => {
      openWebsocketChannel();
    }, 2000);
  };
  websocket.onerror = (err) => {
    ElMessage({
      showClose: true,
      type: 'error',
      message: t('home.websocket_error', { err: err }),
    });
    console.log('websocket error: ', err);
    statusHandle(null);
  };
  ws = websocket;
};

const resizeHandle = () => {
  if (document.body.clientWidth < 768) {
    isMobile.value = true;
  } else {
    isMobile.value = false;
  }
};

onMounted(() => {
  window.addEventListener('resize', resizeHandle);
  refreshConfigList();
  openWebsocketChannel();
});

onUnmounted(() => {
  ws.close();
  window.removeEventListener('resize', resizeHandle);
});
</script>

<style scoped>
.status-card {
  border-radius: 10px;
}

.card-text {
  text-align: center;
  margin-left: 1%;
  margin-right: 1%;
  margin-top: 3%;
  margin-bottom: 3%;
}
.el-row {
  margin-bottom: 5px;
}

.button {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-top: 5vh;
}

.form {
  display: flex;
  justify-content: center;
  margin: auto;
  margin-top: 2vh;
}
</style>
