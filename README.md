# WaspsWithBazookas
Its like bees with machine guns but way more power

Do not use this to DDOS for the lulz or any other purpose on servers you don't own, it is illegal . Don't say I did not tell you so.

Postman REST Docs
[Postman REST Docs](https://documenter.getpostman.com/view/7072151/S1TR4zsf)
You can also check out the docs/API.md file (but it might be out of date)

# Install
install wrk and have the bin in your path.

```javascript
npm install -g waspswithbazookas
```

# How to run

On your choice of platform for example AWS. You would have one instance running the hive and x amount of other instances running wasps.

## Start HIVE
**Note: Must start HIVE first**

```javascript
wwb-hive 4269
```
First argument is port number (optional) **Default: 4269**
You can also set it by setting your env with ``WWB_HIVE_PORT``

## Start WASP
```javascript
wwb-wasp http://hiveip:hiveport/ 4268
```
First argument is hive url (EX. http://localhost:4269/)
You can also set it by setting your env with ``WWB_HIVE_URL``

Second argument is port number (optional) **Default: 4268**
You can also set it by setting your env with ``WWB_WASP_PORT``
