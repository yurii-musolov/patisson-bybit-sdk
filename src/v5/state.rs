use std::collections::HashMap;

use crate::v5::{Order, OrderUpdateMsg, Position, PositionUpdateMsg};

pub struct SymbolState {
    orders: HashMap<String, Order>,
    // one-way mode position
    one_way: Option<Position>,
    // Buy side of hedge-mode position
    buy: Option<Position>,
    // Sell side of hedge-mode position
    sell: Option<Position>,
}

impl SymbolState {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
            one_way: None,
            buy: None,
            sell: None,
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let id = order.order_id.clone();
        let _ = self.orders.insert(id, order);
    }

    pub fn update_order(&mut self, msg: OrderUpdateMsg) {
        let order = self.orders.get_mut(&msg.order_id).unwrap();
        order.update(msg);
    }

    pub fn remove_order(&mut self, order: Order) {
        let id = order.order_id.clone();
        let _ = self.orders.remove(&id);
    }

    pub fn add_position(&mut self, position: Position) {
        match position.position_idx {
            super::PositionIdx::OneWay => self.one_way = Some(position),
            super::PositionIdx::Buy => self.buy = Some(position),
            super::PositionIdx::Sell => self.sell = Some(position),
        }
    }

    pub fn update_position(&mut self, msg: PositionUpdateMsg) {
        match msg.position_idx {
            super::PositionIdx::OneWay => {
                if let Some(position) = self.one_way.as_mut() {
                    position.update(msg);
                }
            }
            super::PositionIdx::Buy => {
                if let Some(position) = self.buy.as_mut() {
                    position.update(msg);
                }
            }
            super::PositionIdx::Sell => {
                if let Some(position) = self.sell.as_mut() {
                    position.update(msg);
                }
            }
        }
    }

    pub fn remove_position(&mut self, position: Position) {
        match position.position_idx {
            super::PositionIdx::OneWay => self.one_way = None,
            super::PositionIdx::Buy => self.buy = None,
            super::PositionIdx::Sell => self.sell = None,
        }
    }
}
