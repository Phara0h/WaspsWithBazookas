#!/usr/bin/env node

var program = require('commander');
var pwd = require('path').dirname(require.main.filename);
var config = require(pwd + '/../config/config.json');
const proc = require('child_process');
const fs = require('fs');

program.on('command:*', function () {
  console.error('Invalid command: %s\nSee --help for a list of available commands.', program.args.join(' '));
  process.exit(1);
});

var spawn = function() {
  console.log('Starting Hive...')
  var h = require('child_process').spawn('node', [pwd + '/../hive/hive.js', config.instance.hive.port, require('path').resolve(program.log)], {
    detached: true
  });
  setTimeout(()=>{
    config.instance.hive.pid = h.pid;
    var wCount = program.wasps || 1;
    var port = 4268;
    console.log('Starting '+wCount+' Wasps...')
    for (var i = 0; i < wCount; i++) {

      var s = require('child_process').spawn('node', [pwd + '/../wasp/wasp.js', `http://127.0.0.1:${config.instance.hive.port}/`, port, require('path').resolve(program.log)], {
        detached: true
      });
      config.wasps.push({
        port: port,
        pid: s.pid
      });
      port -= 1;
    }
    console.log('done');
    saveConfig();
    process.exit();
  },500)

}

var stopInstances = function() {
  if (config.instance && config.instance.type == 'local' && config.wasps) {
    console.log('Stopping Hive...')
    try {
      process.kill(config.instance.hive.pid)
      console.log('Stopped')
    } catch (e) {
      console.log('Cant stop Hive')
    }

    for (var i = 0; i < config.wasps.length; i++) {
      console.log('Stopping Wasp' + config.wasps[i].pid + '...')
      try {
        process.kill(config.wasps[i].pid)
        console.log('Stopped')
      } catch (e) {
        console.log('Cant stop Wasp' + config.wasps[i].pid + '...');
      }
    }

    config.instance.hive.pid = null;
    config.wasps = [];
    saveConfig();
  } else {
    console.log('Does not seem to be any instance to stop.')
  }
}

var saveConfig = function() {
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
  .action((cmd, prog) => {
    if (!config.instance || config.instance.type != 'local') {
      config = {
        instance: {
          type: 'local',
          hive: {
            ip: '0.0.0.0',
            port: program.hivePort || '4269'
          },
          wasp: {
            ip: '0.0.0.0'
          }
        }
      }
      config.wasps = [];
    } else {
      stopInstances();
    }

    spawn();
  });

program
  .command('stop')
  .description('Stop all WWB services that are running')
  .action((cmd, prog) => {
    stopInstances();
  });

program.parse(process.argv);
