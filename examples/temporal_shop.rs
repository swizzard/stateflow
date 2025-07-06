use chrono::prelude::*;
use stateflow::*;
use std::collections::HashMap;
use std::future::{Future, ready};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Item {
    Pant,
    Shirt,
    Skirt,
}

#[derive(Clone, Debug, Default)]
pub struct StockData {
    stock: HashMap<Item, usize>,
}

impl StockData {
    pub fn set_stock(&mut self, item: Item, amount: usize) {
        self.stock.insert(item, amount);
    }
}

pub struct StockSource {}

impl Source for StockSource {
    type Data = StockData;

    fn get_data(&self) -> impl Future<Output = Result<Self::Data, SFError>> + Send {
        let mut data = StockData::default();
        data.set_stock(Item::Pant, 100);
        data.set_stock(Item::Shirt, 200);
        ready(Ok(data))
    }
}
#[derive(Clone)]
pub struct LowStock {
    item: Item,
    threshold: usize,
}

impl LowStock {
    pub fn new(item: Item, threshold: usize) -> Self {
        Self { item, threshold }
    }
}

impl Predicate for LowStock {
    type Data = StockData;

    fn evaluate(&self, data: &Self::Data) -> bool {
        &self.threshold < data.stock.get(&self.item).unwrap_or(&0usize)
    }
}

#[derive(Clone)]
pub struct OnDoW<D> {
    on_day: chrono::Weekday,
    _data: std::marker::PhantomData<D>,
}

impl<D> OnDoW<D>
where
    D: Clone,
{
    fn new(on_day: chrono::Weekday) -> Self {
        Self {
            on_day,
            _data: std::marker::PhantomData,
        }
    }
    pub fn on_mon() -> Self {
        OnDoW::new(chrono::Weekday::Mon)
    }
    pub fn on_tues() -> Self {
        OnDoW::new(chrono::Weekday::Tue)
    }
    pub fn on_wed() -> Self {
        OnDoW::new(chrono::Weekday::Wed)
    }
    pub fn on_thu() -> Self {
        OnDoW::new(chrono::Weekday::Thu)
    }
    pub fn on_fri() -> Self {
        OnDoW::new(chrono::Weekday::Fri)
    }
    pub fn on_sat() -> Self {
        OnDoW::new(chrono::Weekday::Sat)
    }
    pub fn on_sun() -> Self {
        OnDoW::new(chrono::Weekday::Sun)
    }
}

impl<D> TemporalPredicate for OnDoW<D>
where
    D: Send,
{
    type Data = D;

    fn evaluate_with_datetime(&self, _data: &Self::Data, date: DateTime<Utc>) -> bool {
        date.weekday() == self.on_day
    }
}

fn decider() -> impl Predicate<Data = StockData> {
    let on_tuesday: OnDoW<StockData> = OnDoW::on_tues();
    let low_pants = LowStock::new(Item::Pant, 200usize);
    let low_shirts = LowStock::new(Item::Shirt, 100usize);
    And::combine(on_tuesday, Or::combine(low_pants, low_shirts)).finalize()
}

#[tokio::main]
async fn main() {
    let source = StockSource {};
    let evaluator = decider();
    let data = source.get_data().await.unwrap();
    let decision = if evaluator.evaluate(&data) {
        "should"
    } else {
        "shouldn't"
    };
    println!("we {decision} order more inventory");
}
