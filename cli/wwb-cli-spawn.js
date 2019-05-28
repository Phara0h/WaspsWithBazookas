#!/usr/bin/env node
var program = require('commander');

program
.command('local', 'Local instnace')
.command('aws', `AWS instance`)
.parse(process.argv);
