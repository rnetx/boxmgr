import { service } from './axios';

// Start Service
//  Input: <Null>
//  Output: <Null>
const startService = async () => {
  let response = await service.get('/api/v1/service/start');
  return new Promise((resolve, reject) => {
    if (response.status === 200) {
      resolve();
    } else if (response.status === 400) {
      reject(response.data.message);
    } else {
      console.log(response.status, response.data);
      reject('server response code: ' + response.status);
    }
  });
};

// Stop Service
//  Input: <Null>
//  Output: <Null>
const stopService = async () => {
  let response = await service.get('/api/v1/service/stop');
  return new Promise((resolve, reject) => {
    if (response.status === 200) {
      resolve();
    } else if (response.status === 400) {
      reject(response.data.message);
    } else {
      console.log(response.status, response.data);
      reject('server response code: ' + response.status);
    }
  });
};

// Restart Service
//  Input: <Null>
//  Output: <Null>
const restartService = async () => {
  let response = await service.get('/api/v1/service/restart');
  return new Promise((resolve, reject) => {
    if (response.status === 200) {
      resolve();
    } else if (response.status === 400) {
      reject(response.data.message);
    } else {
      console.log(response.status, response.data);
      reject('server response code: ' + response.status);
    }
  });
};

// Set Core Path
//  Input: path: String
//  Output: <Null>
const setCorePath = async (path) => {
  let response = await service.put(
    '/api/v1/service/core_path',
    { path },
    {
      headers: {
        'Content-Type': 'application/json',
      },
    }
  );
  return new Promise((resolve, reject) => {
    if (response.status === 200) {
      resolve();
    } else if (response.status === 400) {
      reject(response.data.message);
    } else {
      console.log(response.status, response.data);
      reject('server response code: ' + response.status);
    }
  });
};

// Upload Core
//  Input: file_form_data: FormData
//  Output: path: String
const uploadCore = async (formData) => {
  let response = await service.post('/api/v1/service/core_path', formData, {
    headers: {
      'Content-Type': 'multipart/form-data',
    },
  });
  return new Promise((resolve, reject) => {
    if (response.status === 200) {
      resolve(response.data.data.path);
    } else if (response.status === 400) {
      reject(response.data.message);
    } else {
      console.log(response.status, response.data);
      reject('server response code: ' + response.status);
    }
  });
};

// Get Core Path
//  Input: <Null>
//  Output: path: String or Null
const getCorePath = async () => {
  let response = await service.get('/api/v1/service/core_path');
  return new Promise((resolve, reject) => {
    if (response.status === 200) {
      resolve(response.data.data.path);
    } else if (response.status === 400) {
      reject(response.data.message);
    } else {
      console.log(response.status, response.data);
      reject('server response code: ' + response.status);
    }
  });
};

// Get Config
//  Input: <Null>
//  Output: Object { id: String, tag: String, config: Object } or Null
const getConfig = async () => {
  let response = await service.get('/api/v1/service/config');
  return new Promise((resolve, reject) => {
    if (response.status === 200) {
      resolve(response.data.data);
    } else if (response.status === 400) {
      reject(response.data.message);
    } else {
      console.log(response.status, response.data);
      reject('server response code: ' + response.status);
    }
  });
};

// Set Auto Start
//  Input: status: bool
//  Output: <Null>
const setAutoStart = async (b) => {
  let response = await service.put(
    '/api/v1/service/auto_start',
    { status: b },
    {
      headers: {
        'Content-Type': 'application/json',
      },
    }
  );
  return new Promise((resolve, reject) => {
    if (response.status === 200) {
      resolve();
    } else if (response.status === 400) {
      reject(response.data.message);
    } else {
      console.log(response.status, response.data);
      reject('server response code: ' + response.status);
    }
  });
};

// Get Auto Start
//  Input: <Null>
//  Output: status: bool
const getAutoStart = async () => {
  let response = await service.get('/api/v1/service/auto_start');
  return new Promise((resolve, reject) => {
    if (response.status === 200) {
      resolve(response.data.data);
    } else if (response.status === 400) {
      reject(response.data.message);
    } else {
      console.log(response.status, response.data);
      reject('server response code: ' + response.status);
    }
  });
};

export {
  startService,
  stopService,
  restartService,
  setCorePath,
  uploadCore,
  getCorePath,
  setAutoStart,
  getAutoStart,
  getConfig,
};
