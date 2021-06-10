# Setting up your .env file
In order to use one of the two logging programs, you need to have a `.env` file **which will be used relative to your current working directory**. 

<br />
<br />

## Mode
There're two modes - `interval` and `warn`. The `interval` mode runs every N seconds and logs the enabled metrics' usage. The `warn` mode runs every N seconds as well, but logs information only if some of the configured limits are surpassed. Use one of the two modes, ex:

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
When using the warn mode, you need to specify the following parameters (which work in integer percentages). Specifying a 0 on a metric will disable it.:
```
ram_limit=10
cpu_limit=67
disk_limit=15
swap_limit=0 // This metric is disabled
```

<br />
<br />

## Specify logging type
Two logging methods are available, `discord` and `file`. The `discord` method will send a message to a discord
channel containing the used metrics. The `file` method will store metrics in a folder (created and specified by you).
Each file type will create its own folder (`warn` mode will have a warn folder, `interval` mode will have an interval 
folder). In both logging methods, if the `warn` mode does not have any warnings, it will be ignored.

```
type=discord
// OR 
type=file
```

<br />
<br />

## Log credentials
Each logging method, will have its own set of credentials that they need in order to work.

<br />

### Discord Logging
Make sure you have a registered bot and its secret token, more about this can be found [here](https://github.com/reactiflux/discord-irc/wiki/Creating-a-discord-bot-&-getting-a-token). Also make sure to invite that bot in the discord server from which contains the `discord_channel` channel. 

```
discord_key=SECRET_BOT_TOKEN
discord_channel=channel_id
```


### File Logging
You need to have a `logging_directory` variable. It needs to be an absolute path to an existent directory. It will 
be used to store the metrics.
```
logging_directory=C:/absolute/path/to/a/directory
```