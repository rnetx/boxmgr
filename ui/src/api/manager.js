import { service } from './axios';

// Request to exit
//  Input: <Null>
//  Output: <Null>
const requestToExit = async () => {
  let response = await service.get('/api/v1/manager/request_to_exit');
  return new Promise((resolve, reject) => {
    if (response.status === 200 || response.status === 204) {
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

export { requestToExit };
