#!/usr/bin/env node

process.title = 'Wasp';

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
var currentWRKProcess = null;

if(process.argv[4] && process.argv[4] != 'null')
{

  var path = require('path').resolve(process.argv[4]);

  console.log = console.error = function(d)
  {
    fs.appendFileSync(path, d + '\n');
  };
}

if(!hive)
{
  console.error(`Need to set HIVE URL either through WWB_HIVE_URL env or 'wwb-wasp http://<hiveip>:<hiveport>/'`)
  process.exit();
}


fastify.put('/fire', (req, res) =>
{
  // If wrk hangs kill it first;
  stopWRK();

  if(!running)
  {
    if(/^(?:http(s)?:\/\/)?[\w.-]+(?:\.[\w\.-]+)+[\w\-\._~:/?#[\]@!\$&'\(\)\*\+,;=.]+$/gm.exec(req.body.target) == null)
    {
      res.code('400').send(`Don't understands target`);
      console.log('Invalid target')
    }
    else
    {
      running = true;

      if(req.body.script)
      {
        fs.writeFileSync(pwd + "/wrk.lua", decodeURI(req.body.script));
      }

      req.body.t = Number(req.body.t) || 10;
      req.body.c = Number(req.body.c) || 50;
      req.body.d = Number(req.body.d) || 30;
      if(req.body.timeout)
      {
        req.body.timeout = Number(req.body.timeout);
      }

      runWRK(req.body.t, req.body.c, req.body.d, req.body.timeout, req.body.target, req.body.script, cmd =>
      {
        if(cmd.status == 'stopped')
        {
          console.log('No need to let the hive know.');
        }
        else if(cmd.status == 'done')
        {
          console.log('Buzzz buz buz ugh, oof, I mean target destroyed!');
          sendStats(cmd);
        }
        else
        {
          console.error('Ahhh buzzzz rocket jam.');
          console.error(cmd.response);
          request(
          {
            method: 'PUT',
            uri: `${hive}wasp/reportin/${id}/failed`,
            json: true,
            body: cmd.response,
          }, (err, res, body) =>
          {
            running = false;
            if(!err)
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
  }
  else
  {
    res.code('400').send(`I'm already shooting...`);
  }
})

fastify.delete('/die', (req, res) =>
{
  if(!running)
  {
    res.code(200).send('iz ded');
    console.log('Hive killed me...');
    process.exit();
  }
  else
  {
    res.code('400').send(`I'm already shooting cant die yet...`);
  }
});

fastify.get('/boop', (req, res) =>
{
  res.code(200).send('Oh hi');
});

fastify.get('/ceasefire', (req, res) =>
{
  console.log('Ceasefire called!');
  if(stopWRK())
  {
    res.code(200).send('Ok i stops');
  }
  else
  {
    res.code(400).send('Was not firing 0__0');
  }
});

var runWRK = function runWRK(t, c, d, timeout, target, script, cb)
{
  console.log(`Shooting ${target} with bazookas!`);
  var cmd = {
    cmd: `wrk -t${t} -c${c} -d${d} --timeout ${timeout ? timeout :'2'} ` + (script ? `-s${pwd}/wrk.lua ` : '') + target
  }

  var bat = os.platform() == 'win32' ? proc.spawn('cmd.exe', ['/c', cmd.cmd]) : proc.spawn('sh', ['-c', cmd.cmd]);
  currentWRKProcess = bat.pid;
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
    if(cmd.status != 'failed' && code === 0)
    {
      cmd.status = 'done';
      currentWRKProcess = cmd.bat = null;
    }
    else if (code === null)
    {
      cmd.status = 'stopped';
    }
    else
    {
      currentWRKProcess = cmd.bat = null;
    }

    cb(cmd);
  });
}

var stopWRK = function()
{
  if(currentWRKProcess)
  {
    var pid = currentWRKProcess;
    currentWRKProcess = null;
    process.kill(pid);
    running = false;
    console.log('Ok i stops firing now.');
    return true;
  }

  return false;
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


  if(tmp.indexOf('errors:') > -1)
  {
    stats.errors.connect = Number(tmp[tmp.indexOf('connect') + 1].replace(/([^0-9\.])/g, ''));
    stats.errors.read = Number(tmp[tmp.indexOf('connect') + 3].replace(/([^0-9\.])/g, ''));
    stats.errors.write = Number(tmp[tmp.indexOf('connect') + 5].replace(/([^0-9\.])/g, ''));
    stats.errors.timeout = Number(tmp[tmp.indexOf('connect') + 7].replace(/([^0-9\.])/g, ''));

  }

  if(tmp.indexOf('3xx') > -1)
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
    if(!err)
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

var checkIn = function()
{
  request.get(hive + 'wasp/checkin/' + port, (error, response, body) =>
  {
    if(error)
    {
      console.error(`Hive is not responding! ${error}`)
    }
    else
    {
      id = JSON.parse(body).id;
      console.log(id + ' ready and listing for your orders!')
    }
  });
}

var heartBeat = function()
{
  request(
  {
    method: 'GET',
    uri: `${hive}wasp/heartbeat/${port}`
  }, (err, res, body) =>
  {
    if(err)
    {
      console.log(`Hive is not responding! ${err}`)
    }
    else if(res.statusCode == 400)
    {
      checkIn();
    }
  })
}


process.on('uncaughtException', function(err)
{
  console.log(err);
  process.exit();
});

fastify.listen(port, '0.0.0.0');

heartBeat();
setInterval(heartBeat, 5000)
