#!/usr/bin/env node
var program = require('commander');
var pwd = require('path').dirname(require.main.filename);
var config = require(pwd+'/../config/config.json');
const proc = require('child_process');
const fs = require('fs');

program.on('command:*', function () {
  console.error('Invalid command: %s\nSee --help for a list of available commands.', program.args.join(' '));
  process.exit(1);
});

program
.option('-w, --wasps <n>', 'How many wasps instances')
.option('-hp, --hive-port <n>', `Hive's port`,)
.parse(process.argv);

// Would be cool if someone made this


console.log('Under Construction, view github for more details')
