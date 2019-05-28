
const port = 1234;
const http = require('http');
const hostname = '0.0.0.0';


const server = http.createServer((req,res)=>{
    res.statusCode = Math.floor(Math.random() * 2) ? 200 : 400;
  res.end();
});



server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
