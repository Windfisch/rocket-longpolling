Rust + Rocket + Vue Long Polling Demo
=====================================

This is a very simple demo without too much boiler plate code to demonstrate
[long polling](https://www.pubnub.com/blog/http-long-polling/) with the new
Rocket v0.5.0-dev (fetched directly from the GitHub master).

Setup
-----

You need the nightly toolchain: `rustup override set nightly`

To setup the web stuff, `cd web; yarn install`.


Running in development mode
---------------------------

Run the server: `cargo run` (defaults to port 8000).

Serve the webpage: `cd web; yarn serve` (defaults to port 8080).

In a browser, visit [http://localhost:8080](http://localhost:8080).

Open a second browser window, copy the IDs into the text boxes and poke around.

You can also poke using `curl 'localhost:8000/notify?uuid=blah`


Running in production mode
--------------------------

Build the webpage: `cd web; yarn build`

Run the server: `cargo run`. It will serve API and webpage on port 8000.

In a browser, visit [http://localhost:8000](http://localhost:8000).


Changing the port
-----------------

To change the Rocket port, you must specify the `ROCKET_PORT=9001` environment variable and
you need to change the hard-coded port in `web/index.js`.


How it works
------------

Rocket is used to provide the REST API endpoints `/login`, `/notify` and `/poll`.

`/poll` blocks for a certain time or until the user id was notified using `/notify`, whatever
happens first. This is only possible thanks to Rocket 0.5.0-dev, which supports async handlers
that are able to block without locking up the whole server. This is called long-polling.

Also, we add a CORS fairing to the rocket server. Otherwise, due to the browsers' same origin policy,
the `yarn dev` development server (running on `http://localhost:8080`) is not allowed to make API
requests to our rust application (`http://localhost:8000`, which is considered a different server).
This would not be neccessary if the rust server did serve the HTML and JavaScript files.
