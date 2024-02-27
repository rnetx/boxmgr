import axios from 'axios';

const baseURL = 'http://127.0.0.1:9077';
//const baseURL = '';

const service = axios.create({
  baseURL: baseURL,
  timeout: 5000,
});

service.interceptors.request.use((config) => {
  const secret = localStorage.getItem('secret');
  if (secret) {
    config.headers['Authorization'] = `Bearer ${secret}`;
  }
  return config;
});

const getWebsocketPrefix = () => {
  if (baseURL !== '') {
    return baseURL.replace('http', 'ws');
  }
  let protocol = document.location.protocol === 'https:' ? 'wss:' : 'ws:';
  let host = document.location.host;
  return `${protocol}//${host}`;
};

export { getWebsocketPrefix, service };
