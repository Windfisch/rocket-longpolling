Rust + Rocket + Vue Long Polling Demo
=====================================

This is a very simple demo without too much boiler plate code to demonstrate
[long polling](https://www.pubnub.com/blog/http-long-polling/) with the new
Rocket v0.5.0-pre (fetched directly from the GitHub master).

Building
--------

You need the nightly toolchain: `rustup override set nightly`

To setup the web stuff, `cd web; yarn install`.


Running
-------

Run the server: `ROCKET_PORT=8001 cargo run`

Serve the webpage: `cd web; yarn serve`

In a browser, visit [http://localhost:8080](http://localhost:8080).

Open a second browser window, copy the IDs into the text boxes and poke around.

You can also poke using `curl 'localhost:8001/notify?uuid=blah`
