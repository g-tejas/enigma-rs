# Tabs
- Candlestick plots
- Portfolio tab
- Microstructure browser (similar to Bahamas trading)
- Watchlist Widget (https://alphaticks.io/#/alphaterm)
- Depth Chart (https://alphaticks.io/#/alphaterm)
- Trades (like AGGR)
	- Exchange
	- Price (color coded)
	- Size (color coded based on size)
	- Time

# Async Stuff
Because Egui is immediate mode (updated 60 times every second or smth), async stuff works differently.
We want to update our UI using data from websocket feeds. But some of this stuff is blocking, so we can't run
the websocket stuff on the same thread as the main thread (the one running the egui frame). 

Solution: 
It's possible, you just have to run the async runtime off the main thread (because most operating systems want the main thread to handle UI events, and the event loop blocks). You can spawn the tokio executor in a thread instead of trying to transform the main function into an async function with the #[tokio::main] proc macro.

[Example implementation](https://github.com/parasyte/egui-tokio-example/blob/main/src/main.rs)
[Plotting time series using this method](https://github.com/mikael-nilsson-github/egui-alpaca-crypto-trading/blob/main/src/app.rs)


# Notifications
We can add notifications too using egui-notify.

# Naming conventions
https://rust-lang.github.io/api-guidelines/naming.html