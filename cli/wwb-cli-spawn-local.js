#!/usr/bin/env node

var program = require('commander');
var pwd = require('path').dirname(require.main.filename);
var config = require(pwd + '/../config/config.json');
const proc = require('child_process');
const fs = require('fs');
const request = require('request');

program.on('command:*', function()
{
  console.error('Invalid command: %s\nSee --help for a list of available commands.', program.args.join(' '));
  process.exit(1);
});

var spawn = function()
{
  console.log('Starting Hive...')
  var h = require('child_process').spawn('node', [pwd + '/../hive/hive.js', config.instance.hive.port, program.log ? require('path').resolve(program.log) : ''],
  {
    detached: true
  });
  setTimeout(() =>
  {
    config.instance.hive.pid = h.pid;
    console.log('Starting Wasps...')
    request(
    {
      method: 'GET',
      uri: `http://${config.instance.hive.ip}:${config.instance.hive.port}/hive/spawn/local/${program.wasps || 1}`
    }, (err, httpResponse, body) =>
    {
      console.log(body)
      saveConfig();
      process.exit();
    })

  }, 500)

}

var stopInstances = function(cb)
{
  if (config.instance && config.instance.type == 'local' && config.wasps)
  {
    console.log('Stopping wasps...')
    request(
    {
      method: 'DELETE',
      uri: `http://${config.instance.hive.ip}:${config.instance.hive.port}/hive/torch/local`
    }, (err, httpResponse, body) =>
    {
      console.log(body)
      console.log('Stopping Hive...')
      try
      {
        process.kill(config.instance.hive.pid)
        console.log('Stopped')
      }
      catch (e)
      {
        console.log('Cant stop Hive')
      }
      config.instance.hive.pid = null;
      config.wasps = [];
      saveConfig();
      if(cb)
      {
        cb();
      }
    })
  }
  else
  {
    console.log('Does not seem to be any instance to stop.')
    cb();
  }
}

var saveConfig = function()
{
  fs.writeFileSync(pwd + "/../config/config.json", JSON.stringify(config));
  console.log('Config saved')
}

program
  .usage('[command] [options]')
  .option('-w, --wasps <n>', 'How many wasps instances')
  .option('-hp, --hive-port <n>', `Hive's port`, )
  .option('-l, --log <path>', 'Log file path')

program
  .command('start')
  .description('Start WWB services')
  .action(async (cmd, prog) =>
  {
    if (!config.instance || config.instance.type != 'local')
    {
      config = {
        instance:
        {
          type: 'local',
          hive:
          {
            ip: '0.0.0.0',
            port: program.hivePort || '4269'
          }
        }
      }
      config.wasps = [];
      spawn();
    }
    else
    {
      stopInstances(spawn);
    }


  });

program
  .command('stop')
  .description('Stop all WWB services that are running')
  .action((cmd, prog) =>
  {
    stopInstances();
  });

program.parse(process.argv);
