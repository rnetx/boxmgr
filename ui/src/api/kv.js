import { service } from './axios';

// Set Kv
//  Input: key: String, value: <Any>
//  Output: <Null>
const setKv = async (key, value) => {
  let response = await service.post(
    `/api/v1/kv/${key}`,
    { value },
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

// Get Kv
//  Input: key: String
//  Output: value: <Any>
const getKv = async (key) => {
  let response = await service.get(`/api/v1/kv/${key}`);
  return new Promise((resolve, reject) => {
    if (response.status === 200) {
      resolve(response.data.data.value);
    } else if (response.status === 400) {
      reject(response.data.message);
    } else {
      console.log(response.status, response.data);
      reject('server response code: ' + response.status);
    }
  });
};

// Delete Kv
//  Input: key: String
//  Output: <Null>
const deleteKv = async (key) => {
  let response = await service.delete(`/api/v1/kv/${key}`);
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

// List Kv
//  Input: <Null>
//  Output: Array of Object { key: String, value: <Any> }
const listKv = async () => {
  let response = await service.get('/api/v1/kv');
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

export { setKv, getKv, deleteKv, listKv };
