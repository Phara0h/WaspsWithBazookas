
 <img src="https://i.imgur.com/e5WUHB2.png" alt="https://www.npmjs.com/package/waspswithbazookas" data-canonical-src="https://i.imgur.com/e5WUHB2.png" width="350" height="350" />

# Wasps With Bazookas
Its like bees with machine guns but way more power

Do not use this to DDOS for the lulz or any other purpose on servers you don't own, it is illegal . Don't say I did not tell you so.

Postman REST Docs
[Postman REST Docs](https://documenter.getpostman.com/view/208035/S1a1YnXU?version=latest)
You can also check out the docs/API.md file (but it might be out of date)

# Install
install wrk and have the bin in your path.

```javascript
npm install -g waspswithbazookas
```

# How to run

On your choice of platform for example AWS. You would have one instance running the hive and x amount of other instances running wasps.

## wwb-cli

### Start
starts hive and 2 wasp servers on the local machine.
```javascript
wwb-cli spawn local start -w 2
```
### Run load test
Runs a load test with the defaults hitting localhost:1234 (This test server can be found under test/test-http-server.js)
```javascript
wwb-cli hive poke http://localhost:1234/
```
### Get report
Gets report of the load test after it is finished.
```javascript
wwb-cli hive report
```
### Stop
Stops all locally made machines
```javascript
wwb-cli spawn local stop
```

## Manual

### Start HIVE
**Note: Must start HIVE first**

```javascript
wwb-hive 4269
```
First argument is port number (optional) **Default: 4269**
You can also set it by setting your env with ``WWB_HIVE_PORT``

### Start WASP
```javascript
wwb-wasp http://hiveip:hiveport/ 4268
```
First argument is hive url (EX. http://localhost:4269/)
You can also set it by setting your env with ``WWB_HIVE_URL``

Second argument is port number (optional) **Default: 4268**
You can also set it by setting your env with ``WWB_WASP_PORT``


# Sample config file for remote control

```javascript
{
  "instance": {
    "type": "remote",
    "hive": {
      "ip": "0.0.0.0",
      "port": "4269"
    }
  }
}
```
