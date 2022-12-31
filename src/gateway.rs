use crate::utils;
use barter_data::{
    builder::Streams,
    model::{subscription::SubKind, MarketEvent},
    ExchangeId,
};
use barter_integration::model::InstrumentKind;
use futures::StreamExt;
use std::sync::mpsc::Sender;

pub fn add_trades(tx: Sender<MarketEvent>, ticker: String) {
    if let Ok((ticker, base)) = utils::split_ticker(ticker) {
        println!("Successfully Connected");
        tokio::spawn(async move {
            loop {
                let streams = Streams::builder()
                    .subscribe([(
                        ExchangeId::BinanceFuturesUsd,
                        ticker.as_str(),
                        base.as_str(),
                        InstrumentKind::FuturePerpetual,
                        SubKind::Trade,
                    )])
                    .init()
                    .await
                    .unwrap();
                let mut joined_stream = streams.join_map::<MarketEvent>().await;

                while let Some((_exchange, event)) = joined_stream.next().await {
                    println!("{:?}", event);
                    let _result = tx.send(event);
                }
            }
        });
    };
}

pub fn add_liqs(tx: Sender<MarketEvent>, ticker: String) {
    if let Ok((ticker, base)) = utils::split_ticker(ticker) {
        println!("Successfully Connected");
        tokio::spawn(async move {
            loop {
                let streams = Streams::builder()
                    .subscribe([(
                        ExchangeId::BinanceFuturesUsd,
                        ticker.as_str(),
                        base.as_str(),
                        InstrumentKind::FuturePerpetual,
                        SubKind::Liquidation,
                    )])
                    .init()
                    .await
                    .unwrap();
                let mut joined_stream = streams.join_map::<MarketEvent>().await;

                while let Some((_exchange, event)) = joined_stream.next().await {
                    println!("{:?}", event);
                    let _result = tx.send(event);
                }
            }
        });
    };
}
