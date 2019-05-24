#!/usr/bin/env node
const fastify = require('fastify')();
const os = require('os');
const proc = require('child_process');
const request = require('request');

var running = false;
var id = 0;
var hive = process.argv[2] || process.env.WWB_HIVE_URL;
var port = process.argv[3] || process.env.WWB_WASP_PORT || 4268;

if(!hive)
{
  console.error(`Need to set HIVE URL either through WWB_HIVE_URL env or 'wwb-wasp http://<hiveip>:<hiveport>/'`)
  process.exit();
}
else
{
  request.get(hive+'wasp/checkin/'+port,(error, response, body) => {

    if(error)
    {
      console.error(`Hive is not responding! ${error}`)
      process.exit();
    }
    else
    {
      id = JSON.parse(body).id;
      fastify.put('/fire', (req, res) => {
        if(!running)
        {
          running = true;
          runWRK(req.body.t,req.body.c,req.body.d,req.body.target,cmd=>{
            sendStats(cmd);
          });
          res.code('200').send('Rockets launching!');
        }
        else
        {
          res.code('409').send(`I'm already shooting...`);
        }
      })

      var runWRK = function(t,c,d,target,cb)
      {
         console.log(`Shooting ${target} with bazookas!`);
         var cmd = {cmd:`wrk -t${t} -c${c} -d${d} ${target}`}
         var bat = os.platform() == 'win32' ? proc.spawn('cmd.exe', ['/c', cmd.cmd]) : proc.spawn('sh', ['-c', cmd.cmd]);
         cmd.bat = bat;
         cmd.status = 'running';
         cmd.response = "";
         bat.stdout.on('data', function(data)
         {
           cmd.response+=data;
         });
         bat.stderr.on('data', function(data)
         {
           console.log(data)
           cmd.response+=data;
           cmd.status = 'failed';
         });

         bat.on('exit', code =>
         {
           if(cmd.status != 'failed')
           {
             cmd.status = 'done';
             console.log('Buzzz buz buz ugh, oof, I mean target destroyed!');
           }
           else
           {
             console.error('Ahhh buzzzz a rocket jam.');
           }
           cmd.bat = null;
           cb(cmd);
         });
      }

      var sendStats = function(cmd)
      {
        var tmp = cmd.response.slice(cmd.response.indexOf('Latency')).replace(/\n/g,'').split(' ').filter(Boolean);

        var stats = {
          latency: {
            avg: tmp[1],
            stdev: tmp[2],
            max: tmp[3],
            stdevPercent: tmp[4],
          },
          rps: {
            avg : tmp[6],
            stdev: tmp[7],
            max: tmp[8],
            stdevPercent: tmp[9],
          },
          totalRequests: tmp[10],
          read: tmp[14],
          totalRPS: tmp[16].replace('Transfer/sec:',''),
          tps: tmp[17]
        }
        request({
              method: 'PUT',
              uri: `${hive}wasp/reportin/${id}`,
              json: true,
              body: stats
          },(err,res,body)=>{
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


      fastify.listen(port)
      console.log(id+' ready and listing for your orders!')
    }
  })
}
