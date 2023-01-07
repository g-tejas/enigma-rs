use crate::utils;
use barter_data::model::subscription::Subscription;
use barter_data::{
    builder::Streams,
    model::{
        subscription::{Interval, SubKind},
        MarketEvent,
    },
    ExchangeId,
};
use barter_integration::model::InstrumentKind;
use futures::StreamExt;
use std::sync::mpsc::Sender;

pub fn add_stream(tx: Sender<MarketEvent>, subscription: Subscription) {
    println!("Connecting to {:?}", subscription);

    tokio::spawn(async move {
        let streams = Streams::builder()
            .subscribe([subscription])
            .init()
            .await
            .unwrap();
        let mut joined_stream = streams.join_map::<MarketEvent>().await;

        while let Some((_exchange, event)) = joined_stream.next().await {
            let _result = tx.send(event);
        }
    });
}

pub fn add_ohlcv(tx: Sender<MarketEvent>, ticker: &'static str) {
    if let Ok((base, quote)) = utils::split_ticker(ticker) {
        println!("Successfully Connected");
        tokio::spawn(async move {
            loop {
                let streams = Streams::builder()
                    .subscribe([(
                        ExchangeId::Kraken,
                        base,
                        quote,
                        InstrumentKind::Spot,
                        SubKind::Candle(Interval::Minute1),
                    )])
                    .init()
                    .await
                    .unwrap();
                let mut joined_stream = streams.join_map::<MarketEvent>().await;

                while let Some((_exchange, event)) = joined_stream.next().await {
                    let _result = tx.send(event);
                }
            }
        });
    };
}

pub fn add_orderbook(tx: Sender<MarketEvent>, ticker: &'static str) {
    if let Ok((base, quote)) = utils::split_ticker(ticker) {
        println!("Successfully Connected to Orderbook");
        tokio::spawn(async move {
            loop {
                let streams = Streams::builder()
                    .subscribe([(
                        ExchangeId::BinanceFuturesUsd,
                        base,
                        quote,
                        InstrumentKind::FuturePerpetual,
                        SubKind::OrderBook, // Different
                    )])
                    .init()
                    .await
                    .unwrap();
                let mut joined_stream = streams.join_map::<MarketEvent>().await;

                while let Some((_exchange, event)) = joined_stream.next().await {
                    let _result = tx.send(event);
                }
            }
        });
    };
}
