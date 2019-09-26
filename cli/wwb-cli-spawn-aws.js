#!/usr/bin/env node
var program = require('commander');
var pwd = require('path').dirname(require.main.filename);
var config = require(pwd+'/../config/config.json');
const proc = require('child_process');
const fs = require('fs');
const EC2 = require('aws-sdk/clients/ec2');
const ec2 = new AWS.EC2();

// program.on('command:*', function () {
//   console.error('Invalid command: %s\nSee --help for a list of available commands.', program.args.join(' '));
//   process.exit(1);
// });

program
.option('-w, --wasps <n>', 'How many wasps instances')
.option('-hp, --hive-port <n>', `Hive's port`,)
.parse(process.argv);

// Would be cool if someone made this

if(program.config)
{
  config =  require(require('path').resolve(program.config));
}

console.log('Under Construction, view github for more details')

var user-data-script = `
#!/bin/bash
echo "export WWB_HIVE_IP="ec2-3-92-208-159.compute-1.amazonaws.com""
echo "export WWB_HIVE_PORT="4269""
`
// wasp AMI ami-011939fefc51137e9
// hive AMI ami-0b20c6ff3c67d3b07
