#!/usr/bin/env node
const http = require('http');
const fastify = require('fastify')();
const request = require('request');
var wasps = [];
var idCount = 0;

fastify.get('/wasp/checkin/:port', (req, res) => {

var found = null;
    for (var i = 0; i < wasps.length; i++) {
      if(wasps[i].ip == req.ip && wasps[i].port == req.params.port)
      {
        found = i;
        console.log(found)
        break;
      }
    }

    var wasp = {
      ip: req.ip,
      port: req.params.port,
      id: 'wasp'+idCount++
    }
    if(found == null)
    {
      wasps.push(wasp);
    }
    else
    {
      wasps[found] = wasp;
    }


    res.code('200').send({id:wasp.id});

    console.log(`Wasp ${idCount-1} checking in at ${wasp.ip}!`);
    console.log(`Total Wasps: ${wasps.length}`)

})

fastify.get('/wasp/list', (req, res) => {
    res.code('200').send(wasps);
})

fastify.put('/wasp/reportin/:id', (req, res) => {
  console.log(req.params.id,req.body)
  res.send();
})

fastify.put('/hive/poke', (req, res) => {
  for (var i = 0; i < wasps.length; i++) {
      request({
            method: 'PUT',
            uri: `http://${wasps[i].ip}:${wasps[i].port}/fire`,
            json: true,
            body: req.body
        })
  }
  res.code(200).send('Angry wasp noises');
  console.log('Sending command to fire!');
})

fastify.delete('/hive/torch', (req, res) => {
    res.code(200).send(`R.I.P All ${wasps.length} wasps. :'('`);
    wasps = [];
    console.log('f');
})

fastify.get('/hive/status/done', (req, res) => {

})

fastify.get('/hive/status/report', (req, res) => {

})

fastify.get('/hive/status', (req, res) => {

})

fastify.listen(process.argv[3] || process.env.WWB_HIVE_PORT || 4269)
