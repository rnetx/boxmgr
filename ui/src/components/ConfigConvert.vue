<template>
  <div>
    <el-dialog
      v-model="dialogVisible"
      :title="$t('config.convert_title')"
      width="80%"
      top="3%"
      destroy-on-close
      center
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      @closed="closedHandle"
    >
      <div style="margin: auto">
        <el-form>
          <el-form-item :label="$t('generic.tag')" required>
            <el-input v-model="tag" />
          </el-form-item>
          <el-form-item :label="$t('config.link')" required>
            <el-input
              v-model="link"
              :placeholder="$t('generic.please_input')"
            />
          </el-form-item>
          <el-form-item :label="$t('config.convert_link')" required>
            <el-input
              v-model="convertLink"
              :placeholder="$t('generic.please_input')"
            />
          </el-form-item>
        </el-form>
        <div style="margin: auto">
          <Codemirror
            v-model="config"
            :placeholder="$t('generic.please_input')"
            indent-with-tab
            :tab-size="2"
            :scrollbarStyle="null"
            :extensions="editorExtensions"
            :style="{ height: '50vh', width: '100%' }"
            autoDestroy
          >
          </Codemirror>
        </div>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="closeHandle">
            {{ $t('generic.cancel') }}
          </el-button>
          <el-button type="success" @click="convertHandle">
            {{ $t('config.convert') }}
          </el-button>
          <el-button type="primary" @click="confirmHandle">
            {{ $t('generic.confirm') }}
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import { ref, defineEmits, defineModel, onMounted } from 'vue';
import { ElLoading, ElMessage } from 'element-plus';
import axios from 'axios';
import { useI18n } from 'vue-i18n';
import { useDark } from '@vueuse/core';
import { Codemirror } from 'vue-codemirror';
import { json } from '@codemirror/lang-json';
import { oneDark } from '@codemirror/theme-one-dark';

const { t } = useI18n();
const emits = defineEmits(['config-handle']);
const dialogVisible = defineModel({ required: true });
const tag = ref('');
const link = ref('');
const defaultConvertLink =
  'https://api.v1.mk/sub?target=singbox&url={{url}}&insert=false&config=https%3A%2F%2Fraw.githubusercontent.com%2FACL4SSR%2FACL4SSR%2Fmaster%2FClash%2Fconfig%2FACL4SSR_Online_Full_NoAuto.ini&emoji=true&list=false&xudp=false&udp=false&tfo=false&expand=true&scv=false&fdn=false';
const convertLink = ref(defaultConvertLink);
const config = ref('');
const editorExtensions = ref([json()]);

const closedHandle = () => {
  tag.value = '';
  link.value = '';
  config.value = '';
};

const closeHandle = () => {
  dialogVisible.value = false;
};

const convertHandle = () => {
  const convertLoading = ElLoading.service({
    lock: true,
    text: t('config.converting'),
  });
  const l = convertLink.value.replace('{{url}}', encodeURI(link.value));
  axios
    .get(l)
    .then((response) => {
      if (response.status === 200) {
        const data = response.data;
        if (typeof data === 'object') {
          config.value = JSON.stringify(data, null, 2);
        } else {
          try {
            let obj = JSON.parse(data);
            config.value = JSON.stringify(obj, null, 2);
          } catch (e) {
            config.value = data;
          }
        }
        ElMessage({
          showClose: true,
          type: 'success',
          message: t('config.convert_success'),
        });
      } else {
        ElMessage({
          showClose: true,
          type: 'error',
          message: t('config.convert_failed', {
            err: 'server response code: ' + response.status,
          }),
        });
      }
      convertLoading.close();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('config.convert_failed', {
          err: err,
        }),
      });
      convertLoading.close();
    });
};

const confirmHandle = () => {
  if (tag.value === '') {
    ElMessage({
      showClose: true,
      type: 'error',
      message: t('config.tag_cannot_be_empty'),
    });
    return;
  }
  let configObj = {};
  try {
    configObj = JSON.parse(config.value);
  } catch (err) {
    ElMessage({
      showClose: true,
      type: 'error',
      message: t('config.config_must_be_json'),
    });
    return;
  }
  emits('config-handle', tag.value, configObj);
};

onMounted(() => {
  if (useDark().value === true) {
    editorExtensions.value.push(oneDark);
  }
});
</script>
