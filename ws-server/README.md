# Quackers Websocket Backend

Backend server for the Quackers mmo game.  

--- 

## Running the server

Simply use the use the `cargo run` command to run the program. There is no additional configuration needed.

The server will run on 127.0.0.1:8000.


## Requests Accepted

Once connected, clients can send these types of messages 

_Note: Requests must come in to the server as properly formed JSON! And likewise, responses will be JSON..._

The structure of the request:

{
    actionType: String,
    data:       Value
}


# 1) Quack

Allows a user to send a message that he or she is quacking. 

Request 
```
{
    requestType: "quack",
    data: {}
}
```

The server will then blast a message to all connected users, letting them know which user is quacking.

Response
```
{
    responseType: "quack",
    data: {
        userId: String
    }
}
```


# 2) Move

Allows a user to send a message that he or she is quacking. 

Request 
```
{
    requestType: "move",
    data: {
        direction: {
            x: u64,
            y: u64
        }
    }
}
```

The server will then accept the direction a player wants to move, calculates the player's new position, and blast a message to all connected users, letting them the user's new position.

Response
```
{
    responseType: "move",
    data: {
        userId: String,
        position: {
            x: u64,
            y: u64
        }
    }
}
```


# 3) Collecting Crackers

There is no request needed for this event. When each player moves the server will calculate if the player is touching a "cracker". If so:
- player is awarded points
- leaderboard is updated
- a new "cracker" is spawned randomly on the map

Everyone see that the user gained crackers:

Response
```
{
    responseType: "cracker",
    data: {
        userId: String,
        newScore: u64
        old_position: {
            x: u64,
            y: u64
        },
        new_position: {
            x: u64,
            y: u64
        }
    }
}
```


# Inspired by: Rust websocket server tutorial

This code was inspired by the article and example project that can be found here: [TMS Blog - Rust Warp WebSocket server](https://tms-dev-blog.com/build-basic-rust-websocket-server/)

Thanks tmsdev82!

