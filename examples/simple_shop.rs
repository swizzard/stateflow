//! # A simple example of creating a [`Predicate`] that evaluates data from a [`Source`]

use stateflow::prelude::*;
use std::collections::HashMap;

/// Items we might have in stock
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Item {
    Pant,
    Shirt,
    Skirt,
}

/// Our inventory
#[derive(Clone, Debug, Default)]
pub struct StockData {
    stock: HashMap<Item, usize>,
}

impl StockData {
    /// Update our stock with specific [`Item`]s
    pub fn set_stock(&mut self, item: Item, amount: usize) {
        self.stock.insert(item, amount);
    }
}

/// Create a [`ConstSource`] representing our inventory management system
fn source() -> ConstSource<StockData> {
    let mut data = StockData::default();
    data.set_stock(Item::Pant, 100);
    data.set_stock(Item::Shirt, 200);
    ConstSource::new(data)
}

/// Predicate testing low stock
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

/// Create a [`Predicate`] to decide if we need more inventory
fn decider() -> impl Predicate<Data = StockData> {
    let low_pants = LowStock::new(Item::Pant, 200usize);
    let low_shirts = LowStock::new(Item::Shirt, 100usize);
    Or::combine(low_pants, low_shirts).finalize()
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
