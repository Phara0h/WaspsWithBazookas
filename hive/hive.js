#!/usr/bin/env node

process.title = 'Hive';

const http = require('http');
const fastify = require('fastify')();
const request = require('request');
var convert = require('convert-units');
const fs = require('fs');
var pwd = require('path').dirname(require.main.filename);

var hivePort = process.argv[2] || process.env.WWB_HIVE_PORT || 4269;
var heartBeatInt = 15000; // ms
var running = false;
var runTimeout = null;
var duration = 0;
var wasps = [];
var localWasps = [];
var waspDoneCount = 0;
var waspsRunningCount = 0;
var waspLocalPortIndex = 42690; //decrements per wasp
var runTimeStamp = 0;
var idCount = 0;
var report = null;
var logPath = null;


if(process.argv[3])
{
  logPath = require('path').resolve(process.argv[3]);

  console.log = console.error = function(d)
  {
    fs.appendFileSync(logPath, d + '\n');
  };
}

fastify.get('/wasp/heartbeat/:port', (req, res) =>
{
  var found = findWasp(req.ip, req.params.port);

  if(found === null)
  {
    console.log(`A random wasp is reporting a heartbeat that is not part of the hive!`);
    res.code(400).send();
  }
  else
  {
    if(wasps[found])
    {
      wasps[found].lastHeartbeat = Number(process.hrtime.bigint()) / 1000000;
      res.code(200).send();
    }
    else
    {
      res.code(400).send();
    }
  }
})

fastify.get('/wasp/checkin/:port', (req, res) =>
{
  var found = findWasp(req.ip, req.params.port);

  var wasp = {
    ip: req.ip,
    port: req.params.port,
    id: 'BuzzyBoi' + idCount++,
    lastHeartbeat: Number(process.hrtime.bigint()) / 1000000
  }
  if(found === null)
  {
    wasps.push(wasp);
  }
  else
  {
    wasps[found] = wasp;
  }

  res.code('200').send(
  {
    id: wasp.id
  });

  console.log(`Wasp ${wasp.id} checking in at ${wasp.ip}!`);
  console.log(`Total Wasps: ${wasps.length}`)
})

fastify.get('/wasp/list', (req, res) =>
{
  res.code('200').send(wasps);
})

fastify.get('/wasp/boop/snoots', (req, res) =>
{
  console.log('Booping the snoots of the buzzy bois');

  if(!isRunningRes(res, 200))
  {
    if(wasps.length > 0)
    {
      var completed_requests = 0;
      var numOfWasps = wasps.length;

      for(var i = 0; i < numOfWasps; i++)
      {
        var waspIndex = i;
        request(
        {
          method: 'GET',
          uri: `http://${wasps[i].ip}:${wasps[i].port}/boop`
        }, (err) =>
        {
          completed_requests++;

          if(err)
          {
            var found = findWasp(err.address, err.port);
            if(wasps[found])
            {
              wasps[found].lastHeartbeat = 0;
            }
            else
            {
              console.error(err);
            }
          }

          if(completed_requests >= numOfWasps)
          {
            checkHealtStatus();
            res.code(200).send(`Hive is operational with ${wasps.length} wasps ready and waiting orders.`);
            console.log(`Total Wasps: ${wasps.length}`)
          }
        });
      }
    }
    else
    {
      res.code(400).send('There are no wasps to boop.');
    }
  }
})

fastify.put('/wasp/reportin/:id', (req, res) =>
{
  var wasp = wasps.find(w =>
  {
    return w.id == req.params.id;
  });

  if(wasp)
  {
    waspDoneCount++;
    report.status.completed += 1;
    report.wasp.reports.push(
    {
      wasp: wasp,
      status: 'complete',
      stats: req.body
    })
    report.totalRPS += Number(req.body.totalRPS);
    report.read += Number(req.body.read);
    report.totalRequests += Number(req.body.totalRequests);
    report.tps += Number(req.body.tps);
    report.errors.connect += Number(req.body.errors.connect) || 0;
    report.errors.read += Number(req.body.errors.read) || 0;
    report.errors.write += Number(req.body.errors.write) || 0;
    report.errors.timeout += Number(req.body.errors.timeout) || 0;
    report.nonSuccessRequests += req.body.nonSuccessRequests || 0;

    res.send();
  }
  else
  {
    gError('/wasp/reportin/:id', res);
  }
  if(waspDoneCount >= waspsRunningCount)
  {
    genReport();
  }
})

fastify.put('/wasp/reportin/:id/failed', (req, res) =>
{

  var wasp = wasps.find(w =>
  {
    return w.id == req.params.id;
  });

  if(wasp)
  {
    waspDoneCount++;
    report.wasp.reports.push(
    {
      wasp: wasp,
      status: 'failed',
      error: req.body
    });
    report.status.failed += 1;

    res.send();
  }
  else
  {
    gError('/wasp/reportin/:id/failed', res);
  }
  if(waspDoneCount >= waspsRunningCount)
  {
    genReport();
  }

})

fastify.put('/hive/poke', (req, res) =>
{
  if(wasps.length <= 0)
  {
    res.code(400).send('There are no wasps to make angry.');
  }
  else if(!isRunningRes(res))
  {
    if(!req.body.target || /^(?:http(s)?:\/\/)?[\w.-]+(?:\.[\w\.-]+)+[\w\-\._~:/?#[\]@!\$&'\(\)\*\+,;=.]+$/gm.exec(req.body.target) == null)
    {
      res.code(400).send(`need a vaild target, cant shoot into the darkness...`);
      console.log('Invalid target')
    }
    else
    {
      req.body.t = Number(req.body.t) || 10;
      req.body.c = Number(req.body.c) || 50;
      req.body.d = Number(req.body.d) || 30;

      duration = req.body.d * 1000;

      setRunning(true);

      report.target = req.body.target;
      report.threads = req.body.t;
      report.concurrency = req.body.c;
      report.duration = req.body.d;
      report.timeout = Number(req.body.timeout || 2);
      report.script = req.body.script || '';

      for(var i = 0; i < wasps.length; i++)
      {
        request(
        {
          method: 'PUT',
          uri: `http://${wasps[i].ip}:${wasps[i].port}/fire`,
          json: true,
          body: req.body
        }, err=>{
          if(err)
          {
            console.error(err);
          }
        })
      }

      res.code(200).send('Angry wasp noises');
      console.log('Sending command to fire!');

      //shit went down if they don't all respond in duration + 5 seconds
      runTimeout = setTimeout(() =>
      {
        if(running)
        {
          genReport();
        }
      }, (req.body.d + 5) * 1000);
    }
  }
})

fastify.delete('/hive/torch/local', (req, res) =>
{
  res.code(200).send(`R.I.P All ${localWasps.length} wasps. :'(`);
  console.log(`R.I.P All ${localWasps.length} wasps. :'(`);

  for(var i = 0; i < localWasps.length; i++)
  {
    process.kill(localWasps[i].pid);
  }

  wasps = wasps.filter(localWasp =>
  {
    return localWasp.ip != this.ip && localWasp.port != this.port
  }, localWasps)

  localWasps = [];
})

fastify.delete('/hive/torch', (req, res) =>
{
  res.code(200).send(`R.I.P All ${wasps.length} wasps. :'(`);
  console.log(`R.I.P All ${wasps.length} wasps. :'(`);

  for(var i = 0; i < wasps.length; i++)
  {
    request(
    {
      method: 'DELETE',
      uri: `http://${wasps[i].ip}:${wasps[i].port}/die`
    }, err =>
    {
      if(err)
      {
        console.error(err);
      }
    });
  }

  wasps = [];

  for(var i = 0; i < localWasps.length; i++)
  {
    process.kill(localWasps[i].pid);
  }
  localWasps = [];
})

fastify.get('/hive/status/done', (req, res) =>
{
  if(!isRunningRes(res))
  {
    res.code(200).send('done');
  }
})

fastify.get('/hive/status/report', (req, res) =>
{
  if(!isRunningRes(res))
  {
    if(report)
    {
      res.code(200).send(report);
    }
    else
    {
      res.code(400).send('No report yet.');
    }
  }
})

fastify.get('/hive/status/report/:val', (req, res) =>
{
  if(!isRunningRes(res))
  {
    if(report && report[req.params.val])
    {
      res.code(200).send(report[req.params.val]);
    }
    else
    {
      res.code(400).send('No hive information on that.');
    }
  }
})



fastify.get('/hive/status', (req, res) =>
{
  if(!isRunningRes(res, 200))
  {
    res.code(200).send(`Hive is operational with ${wasps.length} wasps ready and waiting orders.`);
  }
})

fastify.get('/hive/spawn/local/:amount', (req, res) =>
{
  if(!isRunningRes(res, 200))
  {
    var wCount = req.params.amount;

    spawnWasps(wCount);

    res.code(200).send(`Attempted to spawn ${localWasps.length}.`);
  }
});

var isRunningRes = function(res, code)
{
  if(running)
  {
    res.code(code || 425).send(
      Math.round((((((Number(process.hrtime.bigint()) / 1000000) - runTimeStamp))) / duration) * 100) + "% complete, eta " +
      Math.round((duration - ((Number(process.hrtime.bigint()) / 1000000) - runTimeStamp))) + "ms to go."
    );
    return true;
  }
  return false;
}

var setRunning = function(run)
{
  if(run)
  {
    running = true;
    runTimeStamp = Number(process.hrtime.bigint()) / 1000000;
    report = {
      target: '',
      threads: 0,
      concurrency: 0,
      duration: 0,
      timeout: 2,
      script: '',
      startTime: new Date(),
      wasp:
      {
        reports: []
      },
      status:
      {
        completed: 0,
        failed: 0
      },
      latency:
      {
        avg: 0,
        max: 0,
      },
      rps:
      {
        avg: 0,
        max: 0,
      },
      totalRPS: 0,
      totalRequests: 0,
      read: 0,
      tps: 0,
      nonSuccessRequests: 0,
      errors:
      {
        connect: 0,
        read: 0,
        write: 0,
        timeout: 0
      },
    };

    waspsRunningCount = wasps.length;
  }
  else
  {
    runTimeStamp = 0;
    waspDoneCount = 0;
    waspsRunningCount = 0;
    running = false;
    clearTimeout(runTimeout);
  }
}

var genReport = function()
{
  console.log(`Reports are in lets see how they are.`);
  for(var i = 0; i < report.wasp.reports.length; i++)
  {
    var wasp = report.wasp.reports[i];
    if(wasp.stats)
    {
      report.latency.avg += wasp.stats.latency.avg;
      report.rps.avg += wasp.stats.rps.avg;

      if(wasp.stats.latency.max > report.latency.max)
      {
        report.latency.max = wasp.stats.latency.max;
      }
      if(wasp.stats.rps.max > report.rps.max)
      {
        report.rps.max = wasp.stats.rps.max;
      }
    }
  }
  report.latency.avg = (report.latency.avg / report.status.completed) || 0;
  report.rps.avg = (report.rps.avg / report.status.completed) || 0;
  report.read = (
  {
    val,
    unit
  } = convert(report.read).from('B').toBest(),
  {
    val,
    unit
  });
  report.tps = (
  {
    val,
    unit
  } = convert(report.tps).from('B').toBest(),
  {
    val,
    unit
  });

  setRunning(false);
}

var gError = function(route, res)
{
  res.code(412).send(`I'm a little wasp`);
  console.log(`Bad thingz happened in the ${route} sectorz.`);
}

var spawnWasps = function(wCount)
{
  console.log('Starting ' + wCount + ' Wasps...')
  for(var i = 0; i < wCount; i++)
  {

    var s = require('child_process').spawn('node', [pwd + '/../wasp/wasp.js', `http://127.0.0.1:${hivePort}/`, waspLocalPortIndex, logPath],
    {
      detached: true
    });
    localWasps.push(
    {
      ip: '127.0.0.1',
      port: waspLocalPortIndex,
      pid: s.pid
    });
    waspLocalPortIndex -= 1;
  }
}

var findWasp = function(ip, port)
{
  var found = null;
  for(var i = 0; i < wasps.length; i++)
  {
    if(wasps[i].ip == ip && wasps[i].port == port)
    {
      found = i;
      break;
    }
  }

  return found;
}

var checkHealtStatus = function()
{
  if(wasps.length > 0)
  {
    wasps = wasps.filter((wasp) =>
    {
      var isGood = ((Number(process.hrtime.bigint()) / 1000000) - wasp.lastHeartbeat) < heartBeatInt;
      if(!isGood)
      {
        localWasps = localWasps.filter((localWasp) =>
        {
          var isNotDedBoi = localWasp.ip != wasp.ip && localWasp.port != wasp.port;
          if(!isNotDedBoi)
          {
            console.log(`RIP local wasp ${wasp.id}. Trying to spawn a replacement`);
            spawnWasps(1);
          }
          return isNotDedBoi;
        })
      }
      return isGood;
    })
  }
}

setInterval(checkHealtStatus, heartBeatInt);


console.log('Hive ready to release the wasps!')
fastify.listen(hivePort, '0.0.0.0')

process.on('uncaughtException', function(err)
{
  console.log(err);
  process.exit();
});
