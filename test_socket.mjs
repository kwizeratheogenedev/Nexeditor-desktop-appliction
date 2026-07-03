import { io } from 'socket.io-client';

const socket = io('http://127.0.0.1:3001', { transports: ['websocket', 'polling'] });

socket.on('connect', () => {
  console.log('socket connected', socket.id);
  socket.disconnect();
});

socket.on('connect_error', (err) => {
  console.error('connect_error', err.message);
  process.exit(1);
});

socket.on('error', (e) => {
  console.error('socket error', e);
  process.exit(1);
});

setTimeout(() => {
  console.log('timeout: no connect');
  process.exit(1);
}, 5000);
