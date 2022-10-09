# zoom-cli

This is a command line interface for zoom.

It requires a JWT of zoom to access data in your zoom account.

https://marketplace.zoom.us/docs/guides/auth/jwt/

## Features

- list meetings
- create a new meeting
- start an existing meeting (MacOS only)

## Usage

### Configuration

Before anything, you need to create a configuration file (`~/.zoom-cli`) and put some information. Configuration file should look like this.

```
{
  "token": "YOUR ZOOM JWT",
  "timezone": "Asia/Tokyo"
}
```

### Supported Commands

```
Usage: zoom [COMMAND]

Commands:
  ls     List upcoming meetings.
  new    Create a new meeting.
  start  Start an existing meeting.
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

Here's an example of creating a meeting.
```
zoom new 'Some meeting' --start(-s) '2022-10-09 09:00' --duration(-d) 60
```
