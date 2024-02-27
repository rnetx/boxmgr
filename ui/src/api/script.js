import { service } from './axios';

// Add Script
//  Input: Object { tag: String, content: String }
//  Output: <Null>
const addScript = async (script) => {
  let response = await service.post('/api/v1/script', script, {
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

// Get Script
//  Input: id: String
//  Output: Object { id: String, tag: String, content: String, run_type: Number }
const getScript = async (id) => {
  let response = await service.get(`/api/v1/script/${id}`);
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

// Modify Script
//  Input: id: String, Object { tag: String, content: String }
//  Output: <Null>
const modifyScript = async (id, script) => {
  let response = await service.patch(`/api/v1/script/${id}`, script, {
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

// Delete Script
//  Input: id: String
//  Output: <Null>
const deleteScript = async (id) => {
  let response = await service.delete(`/api/v1/script/${id}`);
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

// Bulk Delete Script
//  Input: Array of String
//  Output: <Null>
const bulkDeleteScript = async (ids) => {
  let response = await service.post(
    '/api/v1/bluk_script_delete',
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

// List Script
//  Input: <Null>
//  Output: Array of Object { id: String, tag: String, content: String, run_type: Number }
const listScript = async () => {
  // Get Simple List
  let response = await service.get('/api/v1/script?simple=true');
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

// Set Script Run Type
//  Input: urlRunType: String, id: String
//  Output: <Null>
const setScriptRunType = async (urlRunType, id) => {
  let response = await service.put(`/api/v1/${urlRunType}/${id}`);
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

// Get Script Run Type
//  Input: urlRunType: String
//  Output: Object { id: String, tag: String, content: String, run_type: Number } or <Null>
const getScriptRunType = async (urlRunType) => {
  let response = await service.get(`/api/v1/${urlRunType}`);
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

// Clean Script Run Type
//  Input: id: String
//  Output: <Null>
const cleanScriptRunType = async (id) => {
  let response = await service.delete(`/api/v1/script_run_type/${id}`);
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

const setBeforeStartScript = async (id) => {
  return setScriptRunType('before_start_script', id);
};
const getBeforeStartScript = async () => {
  return getScriptRunType('before_start_script');
};
const setAfterStartScript = async (id) => {
  return setScriptRunType('after_start_script', id);
};
const getAfterStartScript = async () => {
  return getScriptRunType('after_start_script');
};
const setBeforeCloseScript = async (id) => {
  return setScriptRunType('before_close_script', id);
};
const getBeforeCloseScript = async () => {
  return getScriptRunType('before_close_script');
};
const setAfterCloseScript = async (id) => {
  return setScriptRunType('after_close_script', id);
};
const getAfterCloseScript = async () => {
  return getScriptRunType('after_close_script');
};

export {
  addScript,
  getScript,
  modifyScript,
  deleteScript,
  bulkDeleteScript,
  listScript,
  cleanScriptRunType,
  setBeforeStartScript,
  getBeforeStartScript,
  setAfterStartScript,
  getAfterStartScript,
  setBeforeCloseScript,
  getBeforeCloseScript,
  setAfterCloseScript,
  getAfterCloseScript,
};
