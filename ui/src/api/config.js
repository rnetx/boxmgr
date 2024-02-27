import { service } from './axios';

// Add Config
//  Input: Object { tag: String, config: Object }
//  Output: <Null>
const addConfig = async (config) => {
  let response = await service.post('/api/v1/config', config, {
    headers: {
      'Content-Type': 'application/json',
    },
  });
  return new Promise((resolve, reject) => {
    if (response.status === 200) {
      resolve();
    } else if (response.status === 400) {
      try {
        let obj = JSON.parse(response.data);
        reject(obj.message);
      } catch (e) {
        reject(e);
      }
    } else {
      console.log(response.status, response.data);
      reject('server response code: ' + response.status);
    }
  });
};

// Get Config
//  Input: id: String
//  Output: Object { id: String, tag: String, config: Object }
const getConfig = async (id) => {
  let response = await service.get(`/api/v1/config/${id}`);
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

// Modify Config
//  Input: id: String, Object { tag: String, config: Object }
//  Output: <Null>
const modifyConfig = async (id, config) => {
  let response = await service.patch(`/api/v1/config/${id}`, config, {
    headers: {
      'Content-Type': 'application/json',
    },
  });
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

// Delete Config
//  Input: id: String
//  Output: <Null>
const deleteConfig = async (id) => {
  let response = await service.delete(`/api/v1/config/${id}`);
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

// Bulk Delete Config
//  Input: Array of String
//  Output: <Null>
const bulkDeleteConfig = async (ids) => {
  let response = await service.post(
    '/api/v1/bluk_config_delete',
    { ids: ids },
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

// List Config
//  Input: <Null>
//  Output: Array of Object { id: String, tag: String, config: Object }
const listConfig = async () => {
  // Get Simple List
  let response = await service.get('/api/v1/config?simple=true');
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

// Set Active Config
//  Input: id: String
//  Output: <Null>
const setActiveConfig = async (id) => {
  let response = await service.put(`/api/v1/active_config/${id}`);
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

// Get Active Config
//  Input: <Null>
//  Output: Object { id: String, tag: String, config: Object }
const getActiveConfig = async () => {
  let response = await service.get('/api/v1/active_config');
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
  addConfig,
  getConfig,
  modifyConfig,
  deleteConfig,
  bulkDeleteConfig,
  listConfig,
  setActiveConfig,
  getActiveConfig,
};
