#!/usr/bin/env node

process.title = 'Hive';

const fastify = require('fastify')();
const os = require('os');
const proc = require('child_process');
const request = require('request');
const fs = require('fs');
var convert = require('convert-units');

var pwd = require('path').dirname(require.main.filename);

var running = false;
var id = 0;
var hive = process.argv[2] || process.env.WWB_HIVE_URL;
var port = process.argv[3] || process.env.WWB_WASP_PORT || 4268;

if (!hive)
{
  console.error(`Need to set HIVE URL either through WWB_HIVE_URL env or 'wwb-wasp http://<hiveip>:<hiveport>/'`)
  process.exit();
}
else
{
  request.get(hive + 'wasp/checkin/' + port, (error, response, body) =>
  {
    if (error)
    {
      console.error(`Hive is not responding! ${error}`)
      process.exit();
    }
    else
    {
      id = JSON.parse(body).id;
      fastify.listen(port, '0.0.0.0')
      console.log(id + ' ready and listing for your orders!')
    }
  })
}

fastify.put('/fire', (req, res) =>
{
  if (!running)
  {
    running = true;
    req.body = JSON.parse(req.body);

    if (req.body.script)
    {
      fs.writeFileSync(pwd + "/wrk.lua", decodeURI(req.body.script));
    }

    runWRK(req.body.t, req.body.c, req.body.d, req.body.target, req.body.script, cmd =>
    {
      if (cmd.status != 'failed')
      {
        console.log('Buzzz buz buz ugh, oof, I mean target destroyed!');
        sendStats(cmd);
      }
      else
      {
        console.error('Ahhh buzzzz rocket jam.');
        request(
        {
          method: 'PUT',
          uri: `${hive}wasp/reportin/${id}/failed`,
          json: true,
          body: cmd.response,
        }, (err, res, body) =>
        {
          running = false;
          if (!err)
          {
            console.log('Hive transmission complete.');
          }
          else
          {
            console.error('Hive do you read me? HIVE? Hive not responding... ');
          }
        })
      }


    });
    res.code('200').send('Rockets launching!');
  }
  else
  {
    res.code('409').send(`I'm already shooting...`);
  }
})

var runWRK = function(t, c, d, target, script, cb)
{
  console.log(`Shooting ${target} with bazookas!`);
  var cmd = {
    cmd: `wrk -t${t} -c${c} -d${d} ` + (script ? `-s${pwd}/wrk.lua ` : '') + target
  }
  var bat = os.platform() == 'win32' ? proc.spawn('cmd.exe', ['/c', cmd.cmd]) : proc.spawn('sh', ['-c', cmd.cmd]);
  cmd.bat = bat;
  cmd.status = 'running';
  cmd.response = "";
  bat.stdout.on('data', function(data)
  {
    cmd.response += data;
  });
  bat.stderr.on('data', function(data)
  {
    cmd.response += data;
    cmd.status = 'failed';
  });

  bat.on('exit', code =>
  {
    if (cmd.status != 'failed' && code == 0)
    {
      cmd.status = 'done';
    }
    cmd.bat = null;
    cb(cmd);
  });
}

var sendStats = function(cmd)
{
  var tmp = cmd.response.slice(cmd.response.indexOf('Latency')).replace(/\n/g, ' ').split(' ').filter(Boolean);

  var stats = {
    latency:
    {
      avg: convertStat(tmp[1].replace('us', 'mu'), 'ms') || 0,
      stdev: convertStat(tmp[2].replace('us', 'mu'), 'ms') || 0,
      max: convertStat(tmp[3].replace('us', 'mu'), 'ms') || 0,
      stdevPercent: tmp[4],
    },
    rps:
    {
      avg: convertMetric(tmp[6]) || 0,
      stdev: convertMetric(tmp[7]) || 0,
      max: convertMetric(tmp[8]) || 0,
      stdevPercent: tmp[9],
    },
    read: convertStat(tmp[14], 'B'),
    totalRequests: Number(tmp[10]),
    totalRPS: Number(tmp[tmp.indexOf('Requests/sec:') + 1].replace(/([^0-9\.])/g, '')),
    tps: convertStat(tmp[tmp.indexOf('Transfer/sec:') + 1], 'B'),
    errors:
    {
      connect: 0,
      read: 0,
      write: 0,
      timeout: 0
    },
  }


  if (tmp.indexOf('errors:') > -1)
  {
    stats.errors.connect = Number(tmp[tmp.indexOf('connect') + 1].replace(/([^0-9\.])/g, ''));
    stats.errors.read = Number(tmp[tmp.indexOf('connect') + 3].replace(/([^0-9\.])/g, ''));
    stats.errors.write = Number(tmp[tmp.indexOf('connect') + 5].replace(/([^0-9\.])/g, ''));
    stats.errors.timeout = Number(tmp[tmp.indexOf('connect') + 7].replace(/([^0-9\.])/g, ''));

  }

  if (tmp.indexOf('3xx') > -1)
  {
    stats.nonSuccessRequests = Number(tmp[tmp.indexOf('3xx') + 2]) || 0;
  }

  request(
  {
    method: 'PUT',
    uri: `${hive}wasp/reportin/${id}`,
    json: true,
    body: stats
  }, (err, res, body) =>
  {
    running = false;
    if (!err)
    {
      console.log('Hive transmission complete.');
    }
    else
    {
      console.error('Hive do you read me? HIVE? Hive not responding... ');
    }

  })
  console.log('Reporting back to hive.');
}

var convertStat = function(stat, unit)
{
  return convert(Number(stat.replace(/([^0-9\.])/g, ''))).from(stat.replace(/([0-9\.])/g, '')).to(unit);
}

var convertMetric = function(stat)
{
  var metVar = stat.replace(/([0-9\.])/g, '');
  var num = Number(stat.replace(/([^0-9\.])/g, ''));
  switch (metVar)
  {
    case 'k':
      num = num * 1000;
      break;
    case 'm':
      num = num * 1000000;
      break;
    case 'g':
      num = num * 1000000000;
      break;
  }

  return num;
}
