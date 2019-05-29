#!/usr/bin/env node
var program = require('commander');



program
  .command('spawn <location]', 'Spawn hive and wasps somewhere.')
  .command('hive <cmd>', 'Run a hive command.')
  .parse(process.argv);
