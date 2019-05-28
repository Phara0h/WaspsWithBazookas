#!/usr/bin/env node
var program = require('commander');
var pwd = require('path').dirname(require.main.filename);
var config = require(pwd + '/../config/config.json');
const request = require('request');
const fs = require('fs');
program
.usage('[command] [options] <target>')
.option('-cf, --config <path>', 'Path to WWB config if using a custom one.')
.option('-t, --threads <n>', 'How many threads', 10)
.option('-c, --concurrency <n>', 'The amount of concurrency', 50)
.option('-d, --duration <n>', 'How long to run the test in seconds', 10)
.option('-s, --script <path>', 'Path to wrk lua script')
.arguments('<target>')
.action(function (target) {
    if(target.indexOf('http') <= -1)
    {
      program.outputHelp();
    }
    else {
      program.target = target;
    }
    console.log(program.target)
  })
.parse(process.argv);

if(program.config)
{
  config =  require(require('path').resolve(program.config));
}

request(
{
  method: 'PUT',
  uri: `http://${config.instance.hive.ip}:${config.instance.hive.port}/hive/poke`,
  json: true,
  body: {
    t: program.threads,
    c: program.concurrency,
    d: program.duration,
    script: program.script ?   fs.writeReadSync(require('path').resolve(program.script)) : null,
    target: program.target
  }
},(err, httpResponse, body)=> {
  console.log(body)
})
