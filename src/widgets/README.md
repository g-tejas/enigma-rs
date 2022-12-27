# Widgets 🎛️

### Candlestick Plot
[Code for creating the BoxElem's](https://github.com/SwayStar123/chart_bot/blob/master/src/chartbot.rs)
Keep the BoxElem's in a VecDeque instead, so you can remove the older values, like a rolling window.

Follow this [tutorial](https://www.youtube.com/watch?v=zUvHkkkrmIY) for the sliding window.

### Trades
Can take inspiration from [aggr](https://charts.aggr.trade/s3r1)
Add a slider for size, combobox for ticker, connect button, checkbox for which exchanges you want to listen to.
Columns: 
1. Exchange (with icon)
2. Price (color coded according to buy/sell also)
3. Size (Color code background opacity)
4. Time (of execution, not received)

### Watchtower
Follow the AlphaTerm Watchlist widget. Background color green/red based on return, with ticker name and price. Like the widget on the left in the picture below.

![image](https://media.discordapp.net/attachments/832178723515138071/945339677693575168/update.png?width=2482&height=1321)

### Depth Chart
Tradinglite-ish liquidity map kinda depth chart? Can agggregate data from different exchanges as well.

### Liquidations
Same stuff as trades I guess.

### Portfolio


### Microstructure Browser
Leave this to the last. A major problem is performance, which is why we need to implement striding. As mentioned by BahamasTrading [here](https://twitter.com/BahamasTrading/status/1377351224748605442), we need to upsample/downsample based on how zoomed in you are. egui hasn't implemented this yet, implot has though, so we need to implement it in rust.
[Link to tweet](https://twitter.com/BahamasTrading/status/1506729892326608901)

https://user-images.githubusercontent.com/76802638/209623672-22191104-5f69-47c7-a359-19326c5f8c14.mp4
