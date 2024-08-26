const http = require('http');

// Define the hostname and port the server will listen on
const hostname = '127.0.0.1';
const port = 3000;

// Create the HTTP server
const server = http.createServer((req, res) => {
  // Set the response header content type
  res.statusCode = 200;
  res.setHeader('Content-Type', 'text/html');
  
  // Write the response body
  res.end('<h1>Hello, World  !</h1><p>Welcome to your simple Node.js app!</p>');
});

// Start the server and listen on the specified port and hostname
server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
