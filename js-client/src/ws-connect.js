const WebSocket = require('ws');

const socket = new WebSocket('wss://quackers-beta.jimlynchcodes.com/ws');

// Connection opened
socket.addEventListener('open', function (event) {
    console.log('Connected to the WebSocket server');
    // Optionally, you can send a message once connected
    socket.send('Hello Server!');
});

// Listen for messages
socket.addEventListener('message', function (event) {
    console.log('Message from server:', event.data);
});

// Handle connection close
socket.addEventListener('close', function (event) {
    console.log('WebSocket connection closed');
});

// Handle errors
socket.addEventListener('error', function (event) {
    console.error('WebSocket error:', event);
});