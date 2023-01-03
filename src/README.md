# The async drama
[The original discussion](https://github.com/emilk/egui/discussions/521)

Because Egui is immediate mode (updated 60 times every second or smth), async stuff works differently.
We want to update our UI using data from websocket feeds. But some of this stuff is blocking, so we can't run
the websocket stuff on the same thread as the main thread (the one running the egui frame). Otherwise, our await on websocket feeds would simply block the GUI process and it'd run super slow.

Solution: 
It's possible, you just have to run the async runtime off the main thread (because most operating systems want the main thread to handle UI events, and the event loop blocks). You can spawn the tokio executor in a thread instead of trying to transform the main function into an async function with the #[tokio::main] proc macro.

[Example implementation](https://github.com/parasyte/egui-tokio-example/blob/main/src/main.rs)
[Plotting time series using this method](https://github.com/mikael-nilsson-github/egui-alpaca-crypto-trading/blob/main/src/app.rs)

`26/12`: Trying to figure how to do it via the MPSC method to make the program lock-free and run faster. We need to initialize the channel within Machine::default()

`27/12`: Lock-free concurrency with `std::sync::mpsc` implemented. A new tokio runtime is initiated in the main fn and pressing the "Connect" button spawns a new thread that will run the websocket loop. It's also passed a tx, which is the transmitter. The channel is instantiated in Machine::default(). The websocket messages are passed to the rx, receiver that will be handled in the App::update() loop, with `try_recv` and update the data accordingly. 
Plan is to append to a VecDeque<BoxElem> or something.

`28/12`: parasyte replied to my issue [here](https://github.com/parasyte/egui-tokio-example/issues/1). Basically, he suggested that in order to stop the spawned threads, u can simply pass in a "stop event" channel, that will listen for stop events. But leave this to the end, because, it's very unlikely we will hit any sort of performance issue this early. 


# Todo List
- [ ] Persistence, with serde. Make sure not to serialize the fields containing marketevents. There's some #[dont serialize] i think.
- [ ] OHLCV is messed up, need to fix the x axis and stuff.
`01/01`: Need to add