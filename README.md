# Mina

A simple terminal app to manage the playlist for a music streamer. Built in Rust using [tui-rs](https://github.com/fdehau/tui-rs).

## Background

I have a music streamer which is very good but is controlled by an android app which is appaling when trying to use it with a local UPNP server (MiniDLNA). It consistently shows folder contents as blank until the app is restarted. It's not a problem with MiniDLNA since lots of other open source apps are able to view the contents just fine. The text interface is also much faster to use than browsing directories using DLNA.

I want to be able to easily navigate the contents and add tracks to the streamer's playlist. A bit of investigation showed that it uses a simple API over HTTP to manage the playlist. DLNA is only used for browsing the server contents from the app. DLNA is really hideous. I managed to avoid using SOAP back when it was in fashion so have no interest in starting now :). Instead I decided to dump the track list directly from MiniDLNA's sqlite database into a CSV file and use that. I don't add music to the server that often so it's not a big issue to do it that way.

Although it is possible to have an app (such as upplay) maintain a playlist and play through the streamer, the playlist stops playing when the app does. I want to be able to use the on-board playlist that the streamer maintains itself.

## Dumping MiniDLNA DB to CSV File

This creates a CSV file containing all the tracks in the database and ignoring the DLNA folder structure.

```
sqlite3 -csv -header < dump_tracks.sql minidlna.db > tracks.csv
```

## Playlist API

The playlist is updated by making a POST repuest containing list of JSON track objects with some additional parameters to say where they should be added in the queue.

### List contents

```
GET /inputs/playqueue
```

```
{
    "version": "1.4.0",
    "changestamp": "0",
    "name": "Playqueue",
    "ussi": "inputs\/playqueue",
    "class": "object.input.playqueue",
    "capacity": "500", "cpu": "434",
    "current": "inputs\/playqueue\/2",
    "loading": "0",
    "multiroomMaster": "1",
    "selectable": "1",
    "totalCount": "2",
    "children":
        [
            {
                "name": "Some Track",
                "ussi": "inputs\/playqueue\/2",
                "class": "object.track.upnp",
                "albumName": "Some Album",
                "artistName": "Some Artist",
                "artwork": "http:\/\/192.168.0.123:8200\/AlbumArt\/1654-3590.jpg",
                "mimeType": "audio\/x-flac",
                "serverId": "4d696e69-444c-164e-9d41-0001c0059ea7",
                "track": "64$5$2$2$0",
                "uri": "http:\/\/192.168.0.123:8200\/MediaItems\/3590.flac"
            },
            {
              ...
            }
        ]
}
```

### Add to playlist examples

If there is nothing playing currently, adding `current=0&play=true` to the query will start playing at the added item. Setting the current playlist item (as described below) will also start playing.

```
POST /inputs/playqueue?where=end&clear=false
[
    {
        "albumName": "Some Album",
        "artistName": "Some Artist",
        "artwork": "",
        "class": "object.track.upnp",
        "genre": "",
        "mimeType": "audio/x-flac",
        "name": "Some track title",
        "serverId": "4d696e69-444c-164e-9d41-0001c0059ea7",
        "track": "64$9$5$1$0",
        "uri": "http://192.168.0.123:8200/MediaItems/5475.flac"
    }
]
```


```
POST /inputs/playqueue?where=next&clear=false
[
    {
        "albumName": "Some Other Album",
        "artistName": "Some Other Artist",
        "artwork": "http://192.168.1.23:8200/AlbumArt/2951-5476.jpg",
        "class": "object.track.upnp",
        "genre": "Alternative",
        "mimeType": "audio/x-flac",
        "name": "Some track title",
        "serverId": "4d696e69-444c-164e-9d41-0001c0059ea7",
        "track": "64$5$2F$0$0",
        "uri": "http://192.168.0.123:8200/MediaItems/5476.flac"
    }
]
```

The artwork URL is a combination of the album_art and id columns from the CSV file. The file URL is the id.

### Clear Playlist

```
POST /inputs/playqueue?clear=true
```

### Jump to an item in the playlist

```
PUT /inputs/playqueue?current=inputs%2Fplayqueue%2F3
```

The `current` parameter is the `ussi` value from the playlist entry object, not the index in the playlist.



## "Nowplaying" Controls

Note that these use GET.

```
GET /nowplaying?cmd={cmd}
```

where `cmd` is one of:

* `playpause` toggle play/pause
* `stop` stop playing current track
* `next` play next track
* `prev` play previous track

## Volume

```
GET /levels
```

```
{
    "balance": "0",
    "changestamp": "0",
    "class": "object.levels",
    "cpu": "48293",
    "headphoneDetect": "0",
    "mode": "1",
    "mute": "0",
    "name": "levels",
    "ussi": "levels",
    "version": "1.4.0",
    "volume": "26"
}
```

To update:

```
PUT /levels?volume=28
```

## Turn on when suspended

```
PUT /power?system=on
```

## Tidal Login

Password and username are sent as part of a URL to the streamer (over HTTP).

PUT /inputs/tidal?cmd=login&username=me&password=mypassword

## Other API endpoints

These also show up.

`/system`, `/analytics`, `/inputs`
