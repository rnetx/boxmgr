<template>
  <div class="table-container">
    <el-table
      v-loading="scriptSelectLoading"
      :data="scriptList"
      @selection-change="handleScriptSelected"
    >
      <el-table-column type="selection" />
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
          {{ $t('script.script_type') }}
        </template>
        <template #default="scope">
          <el-tag v-if="scope.row.run_type === 1" type="primary">
            {{ $t('script.before_start_script') }}
          </el-tag>
          <el-tag v-else-if="scope.row.run_type === 2" type="success">
            {{ $t('script.after_start_script') }}
          </el-tag>
          <el-tag v-else-if="scope.row.run_type === 3" type="info">
            {{ $t('script.before_close_script') }}
          </el-tag>
          <el-tag v-else-if="scope.row.run_type === 4" type="danger">
            {{ $t('script.after_close_script') }}
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
            <el-dropdown
              trigger="click"
              @command="
                (run_type) => {
                  setScriptType(scope.row.id, run_type);
                }
              "
            >
              <el-button type="success" size="small" :icon="Setting">
                {{ $t('script.set_script_type') }}
              </el-button>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item
                    command="0"
                    :disabled="scope.row.run_type === 0"
                  >
                    {{ $t('script.disabled') }}
                  </el-dropdown-item>
                  <el-dropdown-item
                    command="1"
                    :disabled="scope.row.run_type === 1"
                  >
                    {{ $t('script.before_start_script') }}
                  </el-dropdown-item>
                  <el-dropdown-item
                    command="2"
                    :disabled="scope.row.run_type === 2"
                  >
                    {{ $t('script.after_start_script') }}
                  </el-dropdown-item>
                  <el-dropdown-item
                    command="3"
                    :disabled="scope.row.run_type === 3"
                  >
                    {{ $t('script.before_close_script') }}
                  </el-dropdown-item>
                  <el-dropdown-item
                    command="4"
                    :disabled="scope.row.run_type === 4"
                  >
                    {{ $t('script.after_close_script') }}
                  </el-dropdown-item>
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </el-button-group>
        </template>
      </el-table-column>
    </el-table>

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
          <el-form-item :label="$t('script.script')" required>
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
  Setting,
} from '@element-plus/icons-vue';
import { ref, onMounted } from 'vue';
import { useI18n } from 'vue-i18n';
import { ElLoading, ElMessage } from 'element-plus';
import { useDark } from '@vueuse/core';
import { Codemirror } from 'vue-codemirror';
import { oneDark } from '@codemirror/theme-one-dark';
import {
  listScript,
  addScript,
  getScript,
  modifyScript,
  deleteScript,
  bulkDeleteScript,
  cleanScriptRunType,
  setBeforeStartScript,
  setAfterStartScript,
  setBeforeCloseScript,
  setAfterCloseScript,
} from '@/api/script';

const { t } = useI18n();

const scriptList = ref([]);
const scriptSelectLoading = ref(false);
const scriptSelected = ref([]);
const bulkDeleteDialogVisible = ref(false);

const closeBulkDeleteDialog = () => {
  bulkDeleteDialogVisible.value = false;
};

const handleScriptSelected = (val) => {
  scriptSelected.value = val.map((item) => item.id);
};

const refreshScriptList = () => {
  scriptSelectLoading.value = true;
  listScript()
    .then((res) => {
      scriptList.value = res;
      scriptSelectLoading.value = false;
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('generic.get_script_list_failed', { err: err }),
      });
      console.log('get script list failed: ', err);
      scriptSelectLoading.value = false;
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
      content: editorData.value,
    };
    addScript(obj)
      .then(() => {
        ElMessage({
          showClose: true,
          type: 'success',
          message: t('script.add_script_success'),
        });
        refreshScriptList();
        confirming.close();
        editorVisible.value = false;
      })
      .catch((err) => {
        ElMessage({
          showClose: true,
          type: 'error',
          message: t('script.add_script_failed', { err: err }),
        });
        console.log('add script failed: ', err);
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
      content: editorData.value,
    };
    modifyScript(editorID.value, obj)
      .then(() => {
        ElMessage({
          showClose: true,
          type: 'success',
          message: t('script.edit_script_success'),
        });
        refreshScriptList();
        confirming.close();
        editorVisible.value = false;
      })
      .catch((err) => {
        ElMessage({
          showClose: true,
          type: 'error',
          message: t('script.edit_script_failed', { err: err }),
        });
        console.log('edit script failed: ', err);
        confirming.close();
      });
  }
};

//

// Operate
const addClick = () => {
  editorInitHandle();
  editorTitle.value = t('script.add_script');
  editorType.value = 1; // add
  editorVisible.value = true;
};

const editClick = (id) => {
  editorInitHandle();
  const loading = ElLoading.service({
    lock: true,
    text: t('generic.loading'),
  });
  getScript(id)
    .then((res) => {
      editorID.value = res.id;
      editorTag.value = res.tag;
      editorData.value = JSON.stringify(res.script, null, 2);
      editorTitle.value = t('script.edit_script', { tag: editorTag.value });
      editorType.value = 2; // edit
      editorVisible.value = true;
      loading.close();
    })
    .catch((err) => {
      editorInitHandle();
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('script.get_script_failed', { err: err }),
      });
      console.log('delete script failed: ', err);
      loading.close();
    });
};

const viewClick = (id) => {
  editorInitHandle();
  const loading = ElLoading.service({
    lock: true,
    text: t('generic.loading'),
  });
  getScript(id)
    .then((res) => {
      editorTag.value = res.tag;
      editorData.value = JSON.stringify(res.script, null, 2);
      editorTitle.value = t('script.view_script', { tag: editorTag.value });
      editorType.value = 3; // view
      editorVisible.value = true;
      loading.close();
    })
    .catch((err) => {
      editorInitHandle();
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('script.get_script_failed', { err: err }),
      });
      console.log('delete script failed: ', err);
      loading.close();
    });
};

const deleteClick = (id) => {
  const deleting = ElLoading.service({
    lock: true,
    text: t('generic.deleting'),
  });
  deleteScript(id)
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('script.delete_script_success'),
      });
      refreshScriptList();
      deleting.close();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('script.delete_script_failed', { err: err }),
      });
      console.log('delete script failed: ', err);
      deleting.close();
    });
};

const bulkDeleteClick = () => {
  if (scriptSelected.value.length === 0) {
    ElMessage({
      showClose: true,
      type: 'warning',
      message: t('script.no_script_selected'),
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
  bulkDeleteScript(scriptSelected.value)
    .then(() => {
      ElMessage({
        showClose: true,
        type: 'success',
        message: t('script.delete_script_success'),
      });
      refreshScriptList();
      deleting.close();
    })
    .catch((err) => {
      ElMessage({
        showClose: true,
        type: 'error',
        message: t('script.delete_script_failed', { err: err }),
      });
      console.log('delete script failed: ', err);
      deleting.close();
    });
};

const setScriptType = (id, run_type) => {
  const confirming = ElLoading.service({
    lock: true,
    text: t('generic.confirming'),
  });
  let pr = null;
  let ty = Number(run_type);
  if (ty === 0) {
    pr = cleanScriptRunType(id);
  } else if (ty === 1) {
    pr = setBeforeStartScript(id);
  } else if (ty === 2) {
    pr = setAfterStartScript(id);
  } else if (ty === 3) {
    pr = setBeforeCloseScript(id);
  } else if (ty === 4) {
    pr = setAfterCloseScript(id);
  }
  pr.then(() => {
    ElMessage({
      showClose: true,
      type: 'success',
      message: t('script.set_script_type_success'),
    });
    refreshScriptList();
    confirming.close();
  }).catch((err) => {
    ElMessage({
      showClose: true,
      type: 'error',
      message: t('script.set_script_type_failed', { err: err }),
    });
    console.log('set script type failed: ', err);
    confirming.close();
  });
};

onMounted(() => {
  if (useDark().value === true) {
    editorExtensions.value.push(oneDark);
  }
  refreshScriptList();
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
