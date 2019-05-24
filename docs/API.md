#Menu
- Hive
  - [/hive/poke](#48752388-2f99-4fee-b382-4bc9637a0931)
  - [/hive/torch](#4ad64ec2-1ead-48aa-bda2-7fcc0c1b23c2)
  - [/hive/status](#f61c7ea6-b50d-4519-9278-a96a477f7238)
  - [/hive/status/report](#2ffd4bcf-1a4a-4619-b835-132d4238abd2)
  - [/hive/status/report/:field](#33081210-91ee-4346-a083-770745f1ce9f)
  - [/wasp/list](#62c2566b-bfec-4263-a33b-cdf9463826bf)
- Wasps: **These are used by the wasps and should not be used directly**
  - [/wasp/reportin/:id](#8fbb30b5-1b3b-4339-9d50-377f5d5f0483)
  - [/wasp/reportin/:id/failed](#0a7e4190-2b7b-4185-a508-4d7121aeac75)
  - [/wasp/checkin/:port](#fc2c08fa-597f-47c6-a349-97f74177d5cf)
- Wasp: Wasps endpoints, you should let Hive exclusive use them instead of directly hitting them.
  - [/fire](#340ee629-9d3f-4fd9-9114-4b43b66826d2)

#Hive
## <i id="48752388-2f99-4fee-b382-4bc9637a0931"></i>/hive/poke
`PUT` `{{HIVE}}/hive/poke`

starts the load test.

**Heders**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Body**

```json
{
	"t":10,
	"c":50,
	"d":5,
	"target":"http://127.0.0.1:1234/"
}
```
**Sample cURL**

```shell
$ curl -X PUT \
    {{HIVE}}/hive/poke \
    -H "Content-Type": application/json \
    -H "cache-control": no-cache \
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

**Heders**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Sample cURL**

```shell
$ curl -X DELETE \
    {{HIVE}}/hive/torch \
    -H "Content-Type": application/json \
    -H "cache-control": no-cache
```

## <i id="f61c7ea6-b50d-4519-9278-a96a477f7238"></i>/hive/status
`GET` `{{HIVE}}/hive/status`

Gives the current oprational status of the hive.

**Heders**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/hive/status \
    -H "Content-Type": application/json \
    -H "cache-control": no-cache
```

## <i id="2ffd4bcf-1a4a-4619-b835-132d4238abd2"></i>/hive/status/report
`GET` `{{HIVE}}/hive/status/report`

Gives full report of the load test.

**Heders**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/hive/status/report \
    -H "Content-Type": application/json \
    -H "cache-control": no-cache
```

## <i id="33081210-91ee-4346-a083-770745f1ce9f"></i>/hive/status/report/:field
`GET` `{{HIVE}}/hive/status/report/:field`

Getting just what you want from the report.

**Heders**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Path Variables**

| key | value | description |
| ---- | ---- | ---- |
| field | totalRPS |  |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/hive/status/report/:field \
    -H "Content-Type": application/json \
    -H "cache-control": no-cache
```

## <i id="62c2566b-bfec-4263-a33b-cdf9463826bf"></i>/wasp/list
`GET` `{{HIVE}}/wasp/list`

Lists all current wasps that have checked in.

**Heders**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/wasp/list \
    -H "Content-Type": application/json \
    -H "cache-control": no-cache
```
#Hive - Wasps
## <i id="8fbb30b5-1b3b-4339-9d50-377f5d5f0483"></i>/wasp/reportin/:id
`PUT` `{{HIVE}}/wasp/reportin/:id`

Wasps hits this endpoint when the load test is done with its results

**This is used by the wasp and should not be used directly**

**Heders**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Path Variables**

| key | value | description |
| ---- | ---- | ---- |
| id | waspsid |  |

**Sample cURL**

```shell
$ curl -X PUT \
    {{HIVE}}/wasp/reportin/:id \
    -H "Content-Type": application/json \
    -H "cache-control": no-cache
```

## <i id="0a7e4190-2b7b-4185-a508-4d7121aeac75"></i>/wasp/reportin/:id/failed
`PUT` `{{HIVE}}/wasp/reportin/:id/failed`

Wasps hits this endpoint when the load test it has failed its loadtest

**This is used by the wasp and should not be used directly**

**Heders**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Path Variables**

| key | value | description |
| ---- | ---- | ---- |
| id | waspsid |  |

**Sample cURL**

```shell
$ curl -X PUT \
    {{HIVE}}/wasp/reportin/:id/failed \
    -H "Content-Type": application/json \
    -H "cache-control": no-cache
```

## <i id="fc2c08fa-597f-47c6-a349-97f74177d5cf"></i>/wasp/checkin/:port
`GET` `{{HIVE}}/wasp/checkin/:port`

Wasps hits this endpoint when it first starts up to let hive know its IP and port number.

**This is used by the wasp and should not be used directly**

**Heders**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Path Variables**

| key | value | description |
| ---- | ---- | ---- |
| port | 1234 |  |

**Sample cURL**

```shell
$ curl -X GET \
    {{HIVE}}/wasp/checkin/:port \
    -H "Content-Type": application/json \
    -H "cache-control": no-cache
```
#Wasp
## <i id="340ee629-9d3f-4fd9-9114-4b43b66826d2"></i>/fire
`PUT` `{{WASP}}/fire`

Have the wasp start the loadtest.

**Heders**

| key | type | value | description |
| ---- | ---- | ---- | ---- |
| Content-Type | `text` | application/json |  |

**Body**

```json
{
	"t":10,
	"c":50,
	"d":5,
	"target":"http://127.0.0.1:1234/"
}
```
**Sample cURL**

```shell
$ curl -X PUT \
    {{WASP}}/fire \
    -H "Content-Type": application/json \
    -H "cache-control": no-cache \
    -d '{
	"t":10,
	"c":50,
	"d":5,
	"target":"http://127.0.0.1:1234/"
}'
```
