use std::collections::HashMap;

use crate::v5::{Category, Order, OrderUpdateMsg, Position, PositionIdx, PositionUpdateMsg};

pub struct UserState {
    spot: HashMap<String, SymbolState>,
    linear: HashMap<String, SymbolState>,
    inverse: HashMap<String, SymbolState>,
    option: HashMap<String, SymbolState>,
}

impl Default for UserState {
    fn default() -> Self {
        Self::new()
    }
}

impl UserState {
    pub fn new() -> Self {
        Self {
            inverse: HashMap::new(),
            linear: HashMap::new(),
            option: HashMap::new(),
            spot: HashMap::new(),
        }
    }

    fn symbol_state(&mut self, category: Category, symbol: String) -> &mut SymbolState {
        match category {
            Category::Inverse => &mut self.inverse,
            Category::Linear => &mut self.linear,
            Category::Option => &mut self.option,
            Category::Spot => &mut self.spot,
        }
        .entry(symbol)
        .or_insert_with(|| SymbolState::default())
    }

    pub fn add_order(&mut self, category: Category, order: Order) {
        self.symbol_state(category, order.symbol.clone())
            .add_order(order);
    }

    pub fn update_order(&mut self, category: Category, msg: OrderUpdateMsg) {
        self.symbol_state(category, msg.symbol.clone())
            .update_order(msg);
    }

    pub fn remove_order(&mut self, category: Category, order: Order) {
        self.symbol_state(category, order.symbol.clone())
            .remove_order(order);
    }

    pub fn add_position(&mut self, category: Category, position: Position) {
        self.symbol_state(category, position.symbol.clone())
            .add_position(position);
    }

    pub fn update_position(&mut self, category: Category, msg: PositionUpdateMsg) {
        self.symbol_state(category, msg.symbol.clone())
            .update_position(msg);
    }

    pub fn remove_position(&mut self, category: Category, position: Position) {
        self.symbol_state(category, position.symbol.clone())
            .remove_position(position);
    }
}

pub struct SymbolState {
    orders: HashMap<String, Order>,
    // one-way mode position
    one_way: Option<Position>,
    // Buy side of hedge-mode position
    buy: Option<Position>,
    // Sell side of hedge-mode position
    sell: Option<Position>,
}

impl Default for SymbolState {
    fn default() -> Self {
        Self::new()
    }
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
        let _ = self.orders.remove(&order.order_id);
    }

    pub fn add_position(&mut self, position: Position) {
        match position.position_idx {
            PositionIdx::OneWay => self.one_way = Some(position),
            PositionIdx::Buy => self.buy = Some(position),
            PositionIdx::Sell => self.sell = Some(position),
        }
    }

    pub fn update_position(&mut self, msg: PositionUpdateMsg) {
        let position = match msg.position_idx {
            PositionIdx::OneWay => &mut self.one_way,
            PositionIdx::Buy => &mut self.buy,
            PositionIdx::Sell => &mut self.sell,
        };
        if let Some(position) = position.as_mut() {
            position.update(msg);
        }
    }

    pub fn remove_position(&mut self, position: Position) {
        match position.position_idx {
            PositionIdx::OneWay => self.one_way = None,
            PositionIdx::Buy => self.buy = None,
            PositionIdx::Sell => self.sell = None,
        }
    }
}
