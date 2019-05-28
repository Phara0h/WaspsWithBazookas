#!/usr/bin/env node
var program = require('commander');
var pwd = require('path').dirname(require.main.filename);
var config = require(pwd + '/../config/config.json');



program
.usage('<command> [options]')
.command('poke', 'Starts loadtest')
.command('wasps', 'Lists the current wasps')
.command('torch', 'Deletes wasps from checkin list')
.command('status', 'Gets hives status')
.command('report', 'Gets the loadtests report')
.command('start', 'Start hive server')
.parse(process.argv);
