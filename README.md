# Quackers Websocket Backend Server

Backend server for the Quackers mmo game.  

--- 



## Running the server

Simply use the use the `cargo run` command to run the program. There is no additional configuration needed.

The server will run on 0.0.0.0:8000.


## Frontend

To get the full game experience, you can also run the [front-end code](https://github.com/JimLynchCodes/Quackers-Frontend) for this (also a Rust project, made with Bevy).

## Deploying

Add secrets in github actions environment secrets:

BACKEND_WS_ENDPOINT =

SERVER_IP_ADDRESS = 


then make a new git tag that includes the phrase "beta" and push it.
eg:
```bash
git tag v0.0.1-beta
git push --tags
```


# Manual Way (Not Recommended)

## Compiling for Ubuntu
The regular cargo build won't deploy to ubuntu linx so we'll use _cross_ to compile a build that will work.

Install cross into cargo if you haven't already:
```bash
cargo install cross --git https://github.com/cross-rs/cross
```

Add target for the linux distro you will be deploying to:
```bash
rustup target add x86_64-unknown-linux-gnu
```

Compile for ubuntu
```bash
cross build --target x86_64-unknown-linux-gnu --release
```

## SSH Into Server

```
ssh root@your_ip
```



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

{ "action_type": "bar", "data": "hey" }



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

