<template>
  <div class="table-container">
    <el-table
      v-loading="configTableLoading"
      :data="configList"
      @selection-change="handleConfigSelected"
    >
      <el-table-column type="selection" width="50" />
      <el-table-column prop="tag" align="center">
        <template #header>
          {{ $t('generic.tag') }}
        </template>
        <template #default="scope">
          <el-text>
            {{ scope.row.tag }}
          </el-text>
        </template>
      </el-table-column>
      <el-table-column align="center">
        <template #header>
          {{ $t('config.actived_config') }}
        </template>
        <template #default="scope">
          <el-tag v-if="scope.row.actived === true" type="success">
            {{ $t('config.actived_config') }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column align="center" width="350">
        <template #header>
          <el-button-group>
            <el-button
              type="primary"
              size="small"
              :icon="Plus"
              @click="addClick"
            >
              {{ $t('generic.add') }}
            </el-button>
            <el-button
              type="success"
              size="small"
              :icon="Plus"
              @click="configConvertDialogVisible = true"
            >
              {{ $t('config.add_from_link') }}
            </el-button>
            <el-button
              type="danger"
              size="small"
              :icon="Delete"
              @click="bulkDeleteClick"
            >
              {{ $t('generic.bulk_delete') }}
            </el-button>
          </el-button-group>
        </template>
        <template #default="scope">
          <el-button-group>
            <el-button
              type="primary"
              size="small"
              :icon="View"
              @click="viewClick(scope.row.id)"
            >
              {{ $t('generic.view') }}
            </el-button>
            <el-button
              type="warning"
              size="small"
              :icon="Edit"
              @click="editClick(scope.row.id)"
            >
              {{ $t('generic.edit') }}
            </el-button>
            <el-button
              type="success"
              size="small"
              :icon="Check"
              :disabled="scope.row.actived === true"
              @click="setActiveConfigHandle(scope.row.id)"
            >
              {{ $t('config.set_as_actived_config') }}
            </el-button>
            <el-popconfirm
              :confirm-button-text="$t('generic.yes')"
              :cancel-button-text="$t('generic.cancel')"
              :icon="InfoFilled"
              confirm-button-type="danger"
              icon-color="#f56c6c"
              :title="$t('generic.check_delete_this')"
              @confirm="deleteClick(scope.row.id)"
            >
              <template #reference>
                <el-button type="danger" size="small" :icon="Delete">
                  {{ $t('generic.delete') }}
                </el-button>
              </template>
            </el-popconfirm>
          </el-button-group>
        </template>
      </el-table-column>
    </el-table>

    <!-- Add From Link Dialog -->
    <config-convert
      v-model="configConvertDialogVisible"
      @config-handle="addFromLinkHandle"
    />

    <!-- Bulk Delete Check Dialog -->
    <el-dialog
      v-model="bulkDeleteDialogVisible"
      :title="$t('generic.bulk_delete')"
      width="30%"
      top="10%"
      destroy-on-close
      center
      :close-on-click-modal="false"
      :close-on-press-escape="false"
    >
      <div style="margin: auto">
        <p>{{ $t('generic.check_bulk_delete') }}</p>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="closeBulkDeleteDialog">
            {{ $t('generic.cancel') }}
          </el-button>
          <el-button type="danger" @click="bulkDeleteHandle">
            {{ $t('generic.yes') }}
          </el-button>
        </div>
      </template>
    </el-dialog>

    <!-- Editor Dialog -->
    <el-dialog
      v-model="editorVisible"
      :title="editorTitle"
      width="80%"
      top="3%"
      destroy-on-close
      center
      :close-on-click-modal="false"
      :close-on-press-escape="false"
      @closed="editorInitHandle"
    >
      <div style="margin: auto">
        <el-form>
          <el-form-item :label="$t('generic.tag')" required>
            <el-input v-model="editorTag" :disabled="editorType == 3" />
          </el-form-item>
          <el-form-item :label="$t('config.config')" required>
            <Codemirror
              v-model="editorData"
              :disabled="editorType == 3"
              :placeholder="$t('generic.please_input')"
              indent-with-tab
              :tab-size="2"
              :scrollbarStyle="null"
              :extensions="editorExtensions"
              :style="{ height: '400px', width: '100%' }"
              autoDestroy
            >
            </Codemirror>
          </el-form-item>
        </el-form>
      </div>
      <template #footer>
        <div class="dialog-footer">
          <el-button @click="editorCloseHandle">
            {{ $t('generic.cancel') }}
          </el-button>
          <el-button
            v-if="editorType != 3"
            type="primary"
            @click="editorConfirmHandle"
          >
            {{ $t('generic.confirm') }}
          </el-button>
        </div>
      </template>
    </el-dialog>
  </div>
</template>

<script setup>
import {
  Plus,
  View,
  Edit,
  Delete,
  InfoFilled,
  Check,
} from '@element-plus/icons-vue';
import { ref, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElLoading, ElMessage } from 'element-plus';
import { useDark } from '@vueuse/core';
import { Codemirror } from 'vue-codemirror';
import { json } from '@codemirror/lang-json';
import { oneDark } from '@codemirror/theme-one-dark';
import {
  listConfig,
  addConfig,
  getConfig,
  modifyConfig,
  deleteConfig,
  bulkDeleteConfig,
  setActiveConfig,
} from '@/api/config';
import ConfigConvert from './ConfigConvert.vue';

const { t } = useI18n();

const configList = ref([]);
const configTableLoading = ref(false);
const configSelected = ref([]);
const bulkDeleteDialogVisible = ref(false);
const configConvertDialogVisible = ref(false);

const closeBulkDeleteDialog = () => {
  bulkDeleteDialogVisible.value = false;
};

const handleConfigSelected = (val) => {
  configSelected.value = val.map((item) => item.id);
};

const refreshConfigList = () => {
  configTableLoading.value = true;
  listConfig()
    .then((res) => {
      configList.value = res;
      configTableLoading.value = false;
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('generic.get_config_list_failed', { err: err }),
      });
      console.log('get config list failed: ', err);
      configTableLoading.value = false;
    });
};

// Editor
const editorVisible = ref(false);
const editorTitle = ref('');
const editorID = ref('');
const editorTag = ref('');
const editorData = ref('');
const editorType = ref(0); // 1: add, 2: edit, 3: view

const editorExtensions = ref([]);

const editorInitHandle = () => {
  editorID.value = '';
  editorTag.value = '';
  editorData.value = '';
  editorTitle.value = '';
  editorType.value = 0;
};

const editorCloseHandle = () => {
  editorVisible.value = false;
};

const editorConfirmHandle = () => {
  if (editorType.value === 1) {
    // add
    const confirming = ElLoading.service({
      lock: true,
      text: t('generic.confirming'),
    });
    if (editorTag.value == '') {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('generic.tag_cannot_be_empty'),
      });
      confirming.close();
      return;
    }
    let obj = {
      tag: editorTag.value,
      config: {},
    };
    try {
      obj.config = JSON.parse(editorData.value);
    } catch (err) {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('config.config_must_be_json'),
      });
      confirming.close();
      return;
    }
    addConfig(obj)
      .then(() => {
        ElMessage({
          showClose: true,
          type: 'success',
          message: t('config.add_config_success'),
        });
        refreshConfigList();
        confirming.close();
        editorVisible.value = false;
      })
      .catch((err) => {
        ElMessage({
          showClose: true,
          type: 'error',
          message: t('config.add_config_failed', { err: err }),
        });
        console.log('add config failed: ', err);
        confirming.close();
      });
  } else if (editorType.value === 2) {
    // edit
    const confirming = ElLoading.service({
      lock: true,
      text: t('generic.confirming'),
    });
    if (editorTag == '') {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('generic.tag_cannot_be_empty'),
      });
      confirming.close();
      return;
    }
    let obj = {
      tag: editorTag.value,
      config: {},
    };
    try {
      obj.config = JSON.parse(editorData.value);
    } catch (err) {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('config.config_must_be_json'),
      });
      confirming.close();
      return;
    }
    modifyConfig(editorID.value, obj)
      .then(() => {
        ElMessage({
          showClose: true,
          type: 'success',
          message: t('config.edit_config_success'),
        });
        refreshConfigList();
        confirming.close();
        editorVisible.value = false;
      })
      .catch((err) => {
        ElMessage({
          showClose: true,
          type: 'error',
          message: t('config.edit_config_failed', { err: err }),
        });
        console.log('edit config failed: ', err);
        confirming.close();
      });
  }
};

//

// Operate
const addClick = () => {
  editorInitHandle();
  editorTitle.value = t('config.add_config');
  editorType.value = 1; // add
  editorVisible.value = true;
};

const editClick = (id) => {
  editorInitHandle();
  const loading = ElLoading.service({
    lock: true,
    text: t('generic.loading'),
  });
  getConfig(id)
    .then((res) => {
      editorID.value = res.id;
      editorTag.value = res.tag;
      editorData.value = JSON.stringify(res.config, null, 2);
      editorTitle.value = t('config.edit_config', { tag: editorTag.value });
      editorType.value = 2; // edit
      editorVisible.value = true;
      loading.close();
    })
    .catch((err) => {
      editorInitHandle();
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('config.get_config_failed', { err: err }),
      });
      console.log('delete config failed: ', err);
      loading.close();
    });
};

const viewClick = (id) => {
  editorInitHandle();
  const loading = ElLoading.service({
    lock: true,
    text: t('generic.loading'),
  });
  getConfig(id)
    .then((res) => {
      editorTag.value = res.tag;
      editorData.value = JSON.stringify(res.config, null, 2);
      editorTitle.value = t('config.view_config', { tag: editorTag.value });
      editorType.value = 3; // view
      editorVisible.value = true;
      loading.close();
    })
    .catch((err) => {
      editorInitHandle();
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('config.get_config_failed', { err: err }),
      });
      console.log('delete config failed: ', err);
      loading.close();
    });
};

const deleteClick = (id) => {
  const deleting = ElLoading.service({
    lock: true,
    text: t('generic.deleting'),
  });
  deleteConfig(id)
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('config.delete_config_success'),
      });
      refreshConfigList();
      deleting.close();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('config.delete_config_failed', { err: err }),
      });
      console.log('delete config failed: ', err);
      deleting.close();
    });
};

const bulkDeleteClick = () => {
  if (configSelected.value.length === 0) {
    ElMessage({
      showClose: true,
      type: 'warning',
      message: t('config.no_config_selected'),
    });
    return;
  }
  bulkDeleteDialogVisible.value = true;
};

const bulkDeleteHandle = () => {
  closeBulkDeleteDialog();
  const deleting = ElLoading.service({
    lock: true,
    text: t('generic.deleting'),
  });
  bulkDeleteConfig(configSelected.value)
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('config.delete_config_success'),
      });
      refreshConfigList();
      deleting.close();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('config.delete_config_failed', { err: err }),
      });
      console.log('delete config failed: ', err);
      deleting.close();
    });
};

const addFromLinkHandle = (tag, config) => {
  let obj = {
    tag: tag,
    config: config,
  };
  const confirming = ElLoading.service({
    lock: true,
    text: t('generic.confirming'),
  });
  addConfig(obj)
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('config.add_config_success'),
      });
      refreshConfigList();
      confirming.close();
      configConvertDialogVisible.value = false;
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('config.add_config_failed', { err: err }),
      });
      console.log('add config failed: ', err);
      confirming.close();
    });
};

const setActiveConfigHandle = (id) => {
  const confirming = ElLoading.service({
    lock: true,
    text: t('generic.confirming'),
  });
  setActiveConfig(id)
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('generic.set_active_config_success'),
      });
      confirming.close();
      refreshConfigList();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('generic.set_active_config_failed', { err: err }),
      });
      console.log('set active config failed: ', err);
      confirming.close();
    });
};

onMounted(() => {
  editorExtensions.value.push(json());
  if (useDark().value === true) {
    editorExtensions.value.push(oneDark);
  }
  refreshConfigList();
});
</script>

<style>
.table-container {
  display: flex;
  justify-content: center;
  margin-left: 10vh;
  margin-right: 10vh;
}

@media (max-width: 120vh) {
  .table-container {
    margin: auto;
  }
}
</style>
