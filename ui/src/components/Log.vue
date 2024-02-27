<template>
  <div class="log-container">
    <el-card class="log-card" body-style="padding: 20px;">
      <div ref="logBox" class="log-box">
        <div v-for="(log, index) in logs" :key="index" class="log-item">
          {{ log }}
        </div>
      </div>
    </el-card>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { getWebsocketPrefix } from '@/api/axios';

const logs = ref([]);

const addLog = (log) => {
  logs.value.push(log);
  scrollToBottom();
};

const scrollToBottom = () => {
  const logBox = document.querySelector('.log-box');
  if (logBox) {
    logBox.scrollTop = logBox.scrollHeight;
  }
};

let ws = null;

const openWebsocketChannel = () => {
  let websocket = null;
  let prefix = getWebsocketPrefix();
  websocket = new WebSocket(
    `${prefix}/api/v1/service/log?secret=${localStorage.getItem('secret')}`
  );
  websocket.onopen = () => {
    console.log('websocket connected');
  };
  websocket.onmessage = (event) => {
    addLog(event.data);
  };
  websocket.onclose = () => {
    console.log('websocket closed');
    setTimeout(() => {
      openWebsocketChannel();
    }, 2000);
  };
  websocket.onerror = (err) => {
    console.log('websocket error: ', err);
  };
  ws = websocket;
};

onMounted(() => {
  openWebsocketChannel();
});

onUnmounted(() => {
  ws.close();
});
</script>

<style>
.log-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 80vh;
}

.log-card {
  width: 100%;
  overflow: hidden;
}

.log-box {
  height: 70vh;
  overflow-y: auto;
}

.log-item {
  margin-bottom: 10px;
}
</style>
