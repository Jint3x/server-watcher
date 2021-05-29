# Setting up your .env file
In order to use one of the two logging programs, you need to have an `.env` file **in the root of the program type you use (node/rust)**

<br />
<br />

## Mode
There're two modes that can run - `interval` and `warn`. The interval mode runs every n seconds and logs the enabled metrics' usage. Use one of the two modes, ex:

```
mode=interval
```

<br />
<br />


## Interval between each log check
Both `interval` and `warn` modes run on a specified by you interval. The interval is configured in seconds:
```
interval=100
```


Generally, you would like to have a lower interval for the `warn` mode
 and higher for the `interval` one. But the decision is up to you.

<br />
<br />

## Interval Settings
When using the interval mode, you need to specify the following parameters (which can be either true or false):
```
ram=true
cpu=true
cpu_average=false
system_uptime=false
disk=true
swap=true
```

<br />
<br />

## Warn Settings
When using the warn mode, you need to specify the following parameters (which work in integer percentages):
```
ram_limit=10
cpu_limit=67
disk_limit=15
swap_limit=20
```

<br />
<br />

## What logging type will be used
As of now, only logging through discord is implemented. It will send a message to a discord channel containing the used metrics. The `warn` mode will only send messages if one of the warn limits is triggered.

```
type=discord
```

<br />
<br />

## Log credentials
Each logging type, will have its own set of credentials that they need to provide to work.

<br />

### Discord Logging
Make sure you have a registered bot and its secret token, more about this can be found [here](https://github.com/reactiflux/discord-irc/wiki/Creating-a-discord-bot-&-getting-a-token). Also make sure to invite that bot in the discord server from which contains the `discord_channel` channel. 

```
discord_key=SECRET_BOT_TOKEN
discord_channel=channel_id
```