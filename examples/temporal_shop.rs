use stateflow::prelude::*;
use std::collections::HashMap;

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

fn source() -> ConstSource<StockData> {
    let mut data = StockData::default();
    data.set_stock(Item::Pant, 100);
    data.set_stock(Item::Shirt, 200);
    ConstSource::new(data)
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

fn decider() -> impl Predicate<Data = StockData> {
    let on_tuesday: OnDoW<StockData> = OnDoW::on_tues();
    let low_pants = LowStock::new(Item::Pant, 200usize);
    let low_shirts = LowStock::new(Item::Shirt, 100usize);
    And::combine(Not::new(on_tuesday), Or::combine(low_pants, low_shirts)).finalize()
}

#[tokio::main]
async fn main() {
    let evaluator = decider();
    let data = source().get_data().await.unwrap();
    let decision = if evaluator.evaluate(&data) {
        "should"
    } else {
        "shouldn't"
    };
    println!("we {decision} order more inventory");
}
