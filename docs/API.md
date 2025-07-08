# Menu
- Hive
  - [/hive/poke](#48752388-2f99-4fee-b382-4bc9637a0931)
  - [/hive/torch](#4ad64ec2-1ead-48aa-bda2-7fcc0c1b23c2)
  - [/hive/ceasefire](#ceasefire-endpoint)
  - [/hive/spawn/local/:amount](#spawn-local-endpoint)
  - [/hive/status](#f61c7ea6-b50d-4519-9278-a96a477f7238)
  - [/hive/status/report](#2ffd4bcf-1a4a-4619-b835-132d4238abd2)
  - [/hive/status/report/:field](#33081210-91ee-4346-a083-770745f1ce9f)
  - [/hive/status/done](#status-done-endpoint)
  - [/wasp/list](#62c2566b-bfec-4263-a33b-cdf9463826bf)
  - [/wasp/boop/snoots](#wasp-boop-snoots-endpoint)
- Wasps: **These are used by the wasps and should not be used directly**
  - [/wasp/reportin/:id](#8fbb30b5-1b3b-4339-9d50-377f5d5f0483)
  - [/wasp/reportin/:id/failed](#0a7e4190-2b7b-4185-a508-4d7121aeac75)
  - [/wasp/checkin/:port](#fc2c08fa-597f-47c6-a349-97f74177d5cf)
  - [/wasp/heartbeat/:port](#wasp-heartbeat-endpoint)
- Wasp: Wasps endpoints, you should let Hive exclusive use them instead of directly hitting them.
  - [/fire](#340ee629-9d3f-4fd9-9114-4b43b66826d2)
  - [/die](#wasp-die-endpoint)
  - [/boop](#wasp-boop-endpoint)
  - [/ceasefire](#wasp-ceasefire-endpoint)

# Hive
## <i id="48752388-2f99-4fee-b382-4bc9637a0931"></i>/hive/poke
`PUT` `{{HIVE}}/hive/poke`

starts the load test.

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

### Variables
#### t
The amount of threads per wasp
**Default:**10
***Optional***

#### c
The amount of concurrency per wasp
**Default:**50
***Optional***

#### d
How long to run the test in seconds.
**Default:**30
***Optional***

#### timeout
Socket timeout in seconds
**Default:**2
***Optional***

#### target
The target url to hit

#### method
The method to use to hit the target url
**Default:**GET
***Optional***

#### headers
Headers to be sent to the target url
**Default:**{}
***Optional***

#### body
Body to be sent to the target url
***Optional***

#### script
Wrk lua script code to execute
***Optional***

**Body**

```json
{
	"t":10,
	"c":50,
	"d":5,
	"target":"http://127.0.0.1:1234/",
    "method":"GET",
    "headers": {
        "content-type": "application/json",
        "some-random-header": "hi"
    },
    "body" : "{\n  \"foo\": \"bar\",\n  \"lar\": \"moo\"\n}",
    "timeout": 2,
    "script": "-- Lua script code here"
}
```
**Sample cURL**

```shell
$ curl -X PUT \
    {{HIVE}}/hive/poke \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache" \
    -d '{
	"t":10,
	"c":50,
	"d":5,
	"target":"http://127.0.0.1:1234/"
}'
```

## <i id="4ad64ec2-1ead-48aa-bda2-7fcc0c1b23c2"></i>/hive/torch
`DELETE` `{{HIVE}}/hive/torch`

Deletes all wasps that have checked in.

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Sample cURL**

```shell
$ curl -X DELETE \
    {{HIVE}}/hive/torch \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache"
```

## <i id="ceasefire-endpoint"></i>/hive/ceasefire
`GET` `{{HIVE}}/hive/ceasefire`

Stops all the wasps from shooting.

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/hive/ceasefire \
    -H "cache-control: no-cache"
```

## <i id="spawn-local-endpoint"></i>/hive/spawn/local/:amount
`GET` `{{HIVE}}/hive/spawn/local/:amount`

Spawns local wasps by the specified amount.

**Path Variables**

| key | value | description |
| ---- | ---- | ---- |
| amount | 2 | Number of wasps to spawn |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/hive/spawn/local/2 \
    -H "cache-control: no-cache"
```

## <i id="f61c7ea6-b50d-4519-9278-a96a477f7238"></i>/hive/status
`GET` `{{HIVE}}/hive/status`

Gives the current operational status of the hive.

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/hive/status \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache"
```

## <i id="2ffd4bcf-1a4a-4619-b835-132d4238abd2"></i>/hive/status/report
`GET` `{{HIVE}}/hive/status/report`

Gives full report of the load test.

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/hive/status/report \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache"
```

## <i id="33081210-91ee-4346-a083-770745f1ce9f"></i>/hive/status/report/:field
`GET` `{{HIVE}}/hive/status/report/:field`

Getting just what you want from the report.

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Path Variables**

| key | value | description |
| ---- | ---- | ---- |
| field | totalRPS | The field you want to retrieve |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/hive/status/report/totalRPS \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache"
```

## <i id="status-done-endpoint"></i>/hive/status/done
`GET` `{{HIVE}}/hive/status/done`

Returns 200 when the loadtest is done.

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/hive/status/done \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache"
```

## <i id="62c2566b-bfec-4263-a33b-cdf9463826bf"></i>/wasp/list
`GET` `{{HIVE}}/wasp/list`

Lists all current wasps that have checked in.

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/wasp/list \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache"
```

## <i id="wasp-boop-snoots-endpoint"></i>/wasp/boop/snoots
`GET` `{{HIVE}}/wasp/boop/snoots`

Force a health check on all the wasps to see if they're alive.

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/wasp/boop/snoots \
    -H "cache-control: no-cache"
```

# Hive - Wasps
## <i id="8fbb30b5-1b3b-4339-9d50-377f5d5f0483"></i>/wasp/reportin/:id
`PUT` `{{HIVE}}/wasp/reportin/:id`

Wasps hits this endpoint when the load test is done with its results

**This is used by the wasp and should not be used directly**

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Path Variables**

| key | value | description |
| ---- | ---- | ---- |
| id | waspsid | Wasp's ID |

**Sample cURL**

```shell
$ curl -X PUT \
    {{HIVE}}/wasp/reportin/wasp1 \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache"
```

## <i id="0a7e4190-2b7b-4185-a508-4d7121aeac75"></i>/wasp/reportin/:id/failed
`PUT` `{{HIVE}}/wasp/reportin/:id/failed`

Wasps hits this endpoint when the load test has failed

**This is used by the wasp and should not be used directly**

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Path Variables**

| key | value | description |
| ---- | ---- | ---- |
| id | waspsid | Wasp's ID |

**Sample cURL**

```shell
$ curl -X PUT \
    {{HIVE}}/wasp/reportin/wasp1/failed \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache"
```

## <i id="fc2c08fa-597f-47c6-a349-97f74177d5cf"></i>/wasp/checkin/:port
`GET` `{{HIVE}}/wasp/checkin/:port`

Wasps hits this endpoint when it first starts up to let hive know its IP and port number.

**This is used by the wasp and should not be used directly**

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Path Variables**

| key | value | description |
| ---- | ---- | ---- |
| port | 1234 | Wasp's port number |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/wasp/checkin/4268 \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache"
```

## <i id="wasp-heartbeat-endpoint"></i>/wasp/heartbeat/:port
`GET` `{{HIVE}}/wasp/heartbeat/:port`

Wasps hit this endpoint every 5 seconds to let Hive know they are not dead.

**This is used by the wasp and should not be used directly**

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Path Variables**

| key | value | description |
| ---- | ---- | ---- |
| port | 1234 | Wasp's port number |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/wasp/heartbeat/4268 \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache"
```

# Wasp
## <i id="340ee629-9d3f-4fd9-9114-4b43b66826d2"></i>/fire
`PUT` `{{WASP}}/fire`

Have the wasp start the loadtest.

**Headers**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

### Variables
#### t
The amount of threads
**Default:**10
***Optional***

#### c
The amount of concurrency
**Default:**50
***Optional***

#### d
How long to run the test in seconds.
**Default:**30
***Optional***

#### timeout
Socket timeout in seconds
**Default:**2
***Optional***

#### target
The target url to hit

#### method
The method to use to hit the target url
**Default:**GET
***Optional***

#### headers
Headers to be sent to the target url
**Default:**{}
***Optional***

#### body
Body to be sent to the target url
***Optional***

#### script
Wrk lua script code to execute
***Optional***

**Body**

```json
{
	"t":12,
	"c":400,
	"d":10,
	"target":"https://127.0.0.1:3001/",
    "method": "GET",
    "headers": {
        "content-type": "application/json",
        "some-random-header": "hi"
    },
    "body" : "{\n  \"foo\": \"bar\",\n  \"lar\": \"moo\"\n}",
    "timeout": 2,
    "script": "-- Lua script code here"
}
```
**Sample cURL**

```shell
$ curl -X PUT \
    {{WASP}}/fire \
    -H "Content-Type: application/json" \
    -H "cache-control: no-cache" \
    -d '{
	"t":10,
	"c":50,
	"d":5,
	"target":"http://127.0.0.1:1234/"
}'
```

## <i id="wasp-die-endpoint"></i>/die
`DELETE` `{{WASP}}/die`

Kill the wasp like the fearful god you are.

**Sample cURL**

```shell
$ curl -X DELETE \
    {{WASP}}/die \
    -H "cache-control: no-cache"
```

## <i id="wasp-boop-endpoint"></i>/boop
`GET` `{{WASP}}/boop`

Boop the snoot of the wasp to see if it's still alive.

**Sample cURL**

```shell
$ curl -X GET \
    {{WASP}}/boop \
    -H "cache-control: no-cache"
```

## <i id="wasp-ceasefire-endpoint"></i>/ceasefire
`GET` `{{WASP}}/ceasefire`

Tells the wasp to stop shooting.

**Sample cURL**

```shell
$ curl -X GET \
    {{WASP}}/ceasefire \
    -H "cache-control: no-cache"
```
