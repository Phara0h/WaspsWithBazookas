#!/usr/bin/env node
var program = require('commander');
var pwd = require('path').dirname(require.main.filename);
var config = require(pwd + '/../config/config.json');
const request = require('request');

program
.option('-cf, --config <path>', 'Path to WWB config if using a custom one.')
.parse(process.argv)


if(program.config)
{
  config =  require(require('path').resolve(program.config));
}

request(
{
  method: 'GET',
  uri: `http://${config.instance.hive.ip}:${config.instance.hive.port}/hive/status`
},(err, httpResponse, body)=> {
  console.log(body)
})
