use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::v5::{
    Category, Order, OrderMsg, Position, PositionIdx, PositionMsg, WalletBalance, WalletCoin,
    WalletMsg,
};

/// State for the user. An instance for each user ID.
pub struct UserState {
    spot: HashMap<String, SymbolState>,
    linear: HashMap<String, SymbolState>,
    inverse: HashMap<String, SymbolState>,
    option: HashMap<String, SymbolState>,
    wallet: Rc<WalletState>,
}

impl UserState {
    pub fn new(balance: WalletBalance) -> Self {
        Self {
            inverse: HashMap::new(),
            linear: HashMap::new(),
            option: HashMap::new(),
            spot: HashMap::new(),
            wallet: Rc::new(WalletState::new(balance)),
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
        .or_insert_with(|| SymbolState::new(Rc::clone(&self.wallet)))
    }

    pub fn add_order(&mut self, category: Category, order: Order) {
        self.symbol_state(category, order.symbol.clone())
            .add_order(order);
    }

    pub fn update_order(&mut self, category: Category, msg: OrderMsg) {
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

    pub fn update_position(&mut self, category: Category, msg: PositionMsg) {
        self.symbol_state(category, msg.symbol.clone())
            .update_position(msg);
    }

    pub fn remove_position(&mut self, category: Category, position: Position) {
        self.symbol_state(category, position.symbol.clone())
            .remove_position(position);
    }

    pub fn update_wallet(&mut self, msg: WalletMsg) {
        msg.coin.iter().for_each(|(symbol, coin)| {
            if let Some(state) = self.inverse.get_mut(symbol) {
                state.update_position_with_a_wallet_coin(coin);
            }
            if let Some(state) = self.linear.get_mut(symbol) {
                state.update_position_with_a_wallet_coin(coin);
            }
            if let Some(state) = self.option.get_mut(symbol) {
                state.update_position_with_a_wallet_coin(coin);
            }
            if let Some(state) = self.spot.get_mut(symbol) {
                state.update_position_with_a_wallet_coin(coin);
            }
        });

        self.wallet.update_wallet(msg);
    }
}

/// State for the symbol. An instance for each symbol (ticker).
pub struct SymbolState {
    /// User wallet.
    #[allow(unused)]
    wallet: Rc<WalletState>,
    /// State with orders for only current symbol (ticker)
    orders: HashMap<String, Order>,
    // one-way mode position
    one_way: Option<Position>,
    // Buy side of hedge-mode position
    buy: Option<Position>,
    // Sell side of hedge-mode position
    sell: Option<Position>,
}

impl SymbolState {
    pub fn new(wallet: Rc<WalletState>) -> Self {
        Self {
            wallet,
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

    pub fn update_order(&mut self, msg: OrderMsg) {
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

    pub fn update_position(&mut self, msg: PositionMsg) {
        let position = match msg.position_idx {
            PositionIdx::OneWay => &mut self.one_way,
            PositionIdx::Buy => &mut self.buy,
            PositionIdx::Sell => &mut self.sell,
        };
        if let Some(position) = position.as_mut() {
            position.update(msg);
        }
    }

    pub fn update_position_with_a_wallet_coin(&mut self, msg: &WalletCoin) {
        if let Some(position) = self.one_way.as_mut() {
            position.update_with_a_wallet_coin(msg);
        }
        // TODO: Check please:
        // self.buy.position_im == self.sell.position_im
        // self.buy.position_mm == self.sell.position_mm
        // self.buy.unrealised_pnl == self.sell.unrealised_pnl
        if let Some(position) = self.buy.as_mut() {
            position.update_with_a_wallet_coin(msg);
        }
        if let Some(position) = self.sell.as_mut() {
            position.update_with_a_wallet_coin(msg);
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

/// State for the user wallet.
pub struct WalletState {
    balance: RefCell<WalletBalance>,
}
impl WalletState {
    pub fn new(balance: WalletBalance) -> Self {
        Self {
            balance: RefCell::new(balance),
        }
    }

    pub fn update_wallet(&self, msg: WalletMsg) {
        self.balance.borrow_mut().update(msg);
    }
}
