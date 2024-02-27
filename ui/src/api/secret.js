import { service } from './axios';

const checkSecret = async (secret) => {
  const response = await service.get('/api/v1/check_secret', {
    headers: {
      Authorization: `Bearer ${secret}`,
    },
  });
  return response.status === 200;
};

export { checkSecret };
