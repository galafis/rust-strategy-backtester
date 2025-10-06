//! # Rust Strategy Backtester
//!
//! A high-performance backtesting framework for trading strategies with parallel execution support.

use chrono::{DateTime, Utc};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// OHLCV bar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

impl Bar {
    pub fn new(timestamp: DateTime<Utc>, open: f64, high: f64, low: f64, close: f64, volume: f64) -> Self {
        Self { timestamp, open, high, low, close, volume }
    }
}

/// Trading signal
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

/// Strategy trait
pub trait Strategy: Send + Sync {
    fn generate_signal(&self, bars: &[Bar], index: usize) -> Signal;
    fn name(&self) -> &str;
}

/// Simple Moving Average crossover strategy
pub struct SimpleMovingAverageStrategy {
    short_period: usize,
    long_period: usize,
}

impl SimpleMovingAverageStrategy {
    pub fn new(short_period: usize, long_period: usize) -> Self {
        Self { short_period, long_period }
    }

    fn calculate_sma(&self, bars: &[Bar], period: usize, index: usize) -> Option<f64> {
        if index < period - 1 {
            return None;
        }
        let sum: f64 = bars[index - period + 1..=index].iter().map(|b| b.close).sum();
        Some(sum / period as f64)
    }
}

impl Strategy for SimpleMovingAverageStrategy {
    fn generate_signal(&self, bars: &[Bar], index: usize) -> Signal {
        if let (Some(short_sma), Some(long_sma)) = (
            self.calculate_sma(bars, self.short_period, index),
            self.calculate_sma(bars, self.long_period, index),
        ) {
            if index > 0 {
                let prev_short = self.calculate_sma(bars, self.short_period, index - 1);
                let prev_long = self.calculate_sma(bars, self.long_period, index - 1);
                
                if let (Some(ps), Some(pl)) = (prev_short, prev_long) {
                    if short_sma > long_sma && ps <= pl {
                        return Signal::Buy;
                    } else if short_sma < long_sma && ps >= pl {
                        return Signal::Sell;
                    }
                }
            }
        }
        Signal::Hold
    }

    fn name(&self) -> &str {
        "SMA Crossover"
    }
}

/// Trade record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub entry_time: DateTime<Utc>,
    pub exit_time: DateTime<Utc>,
    pub entry_price: f64,
    pub exit_price: f64,
    pub quantity: f64,
    pub pnl: f64,
    pub return_pct: f64,
}

/// Backtest result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    pub strategy_name: String,
    pub initial_capital: f64,
    pub final_capital: f64,
    pub total_return: f64,
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub win_rate: f64,
    pub profit_factor: f64,
    pub max_drawdown: f64,
    pub sharpe_ratio: f64,
    pub trades: Vec<Trade>,
}

/// Backtester
pub struct Backtester {
    initial_capital: f64,
    commission_rate: f64,
}

impl Backtester {
    pub fn new(initial_capital: f64, commission_rate: f64) -> Self {
        Self { initial_capital, commission_rate }
    }

    pub fn run(&self, strategy: &dyn Strategy, bars: &[Bar]) -> BacktestResult {
        let mut capital = self.initial_capital;
        let mut position: Option<(f64, f64)> = None;
        let mut trades = Vec::new();
        let mut equity_curve = Vec::new();
        
        for (i, bar) in bars.iter().enumerate() {
            let signal = strategy.generate_signal(bars, i);
            
            match signal {
                Signal::Buy if position.is_none() => {
                    let quantity = (capital * 0.95) / bar.close;
                    let commission = quantity * bar.close * self.commission_rate;
                    capital -= commission;
                    position = Some((bar.close, quantity));
                }
                Signal::Sell if position.is_some() => {
                    if let Some((entry_price, quantity)) = position {
                        let exit_value = quantity * bar.close;
                        let commission = exit_value * self.commission_rate;
                        let pnl = exit_value - (entry_price * quantity) - commission;
                        let return_pct = (bar.close - entry_price) / entry_price;
                        
                        capital += exit_value - commission;
                        
                        trades.push(Trade {
                            entry_time: bars[i.saturating_sub(1)].timestamp,
                            exit_time: bar.timestamp,
                            entry_price,
                            exit_price: bar.close,
                            quantity,
                            pnl,
                            return_pct,
                        });
                        
                        position = None;
                    }
                }
                _ => {}
            }
            
            let current_equity = if let Some((_, quantity)) = position {
                capital + (quantity * bar.close)
            } else {
                capital
            };
            equity_curve.push(current_equity);
        }
        
        if let Some((entry_price, quantity)) = position {
            let last_bar = bars.last().unwrap();
            let exit_value = quantity * last_bar.close;
            let commission = exit_value * self.commission_rate;
            let pnl = exit_value - (entry_price * quantity) - commission;
            let return_pct = (last_bar.close - entry_price) / entry_price;
            
            capital += exit_value - commission;
            
            trades.push(Trade {
                entry_time: bars[bars.len() - 2].timestamp,
                exit_time: last_bar.timestamp,
                entry_price,
                exit_price: last_bar.close,
                quantity,
                pnl,
                return_pct,
            });
        }
        
        self.calculate_metrics(strategy.name(), capital, &trades, &equity_curve)
    }

    fn calculate_metrics(&self, strategy_name: &str, final_capital: f64, trades: &[Trade], equity_curve: &[f64]) -> BacktestResult {
        let total_return = (final_capital - self.initial_capital) / self.initial_capital;
        let total_trades = trades.len();
        let winning_trades = trades.iter().filter(|t| t.pnl > 0.0).count();
        let losing_trades = trades.iter().filter(|t| t.pnl < 0.0).count();
        let win_rate = if total_trades > 0 { winning_trades as f64 / total_trades as f64 } else { 0.0 };
        
        let gross_profit: f64 = trades.iter().filter(|t| t.pnl > 0.0).map(|t| t.pnl).sum();
        let gross_loss: f64 = trades.iter().filter(|t| t.pnl < 0.0).map(|t| t.pnl.abs()).sum();
        let profit_factor = if gross_loss > 0.0 { gross_profit / gross_loss } else { 0.0 };
        
        let max_drawdown = self.calculate_max_drawdown(equity_curve);
        let sharpe_ratio = self.calculate_sharpe_ratio(trades);
        
        BacktestResult {
            strategy_name: strategy_name.to_string(),
            initial_capital: self.initial_capital,
            final_capital,
            total_return,
            total_trades,
            winning_trades,
            losing_trades,
            win_rate,
            profit_factor,
            max_drawdown,
            sharpe_ratio,
            trades: trades.to_vec(),
        }
    }

    fn calculate_max_drawdown(&self, equity_curve: &[f64]) -> f64 {
        let mut max_drawdown = 0.0;
        let mut peak = equity_curve[0];
        
        for &equity in equity_curve {
            if equity > peak {
                peak = equity;
            }
            let drawdown = (peak - equity) / peak;
            if drawdown > max_drawdown {
                max_drawdown = drawdown;
            }
        }
        
        max_drawdown
    }

    fn calculate_sharpe_ratio(&self, trades: &[Trade]) -> f64 {
        if trades.is_empty() {
            return 0.0;
        }
        
        let returns: Vec<f64> = trades.iter().map(|t| t.return_pct).collect();
        let mean_return = returns.iter().sum::<f64>() / returns.len() as f64;
        let variance = returns.iter().map(|r| (r - mean_return).powi(2)).sum::<f64>() / returns.len() as f64;
        let std_dev = variance.sqrt();
        
        if std_dev > 0.0 {
            (mean_return / std_dev) * (252.0_f64).sqrt()
        } else {
            0.0
        }
    }

    pub fn run_parallel(&self, strategies: Vec<Box<dyn Strategy>>, bars: &[Bar]) -> Vec<BacktestResult> {
        strategies.par_iter().map(|strategy| self.run(strategy.as_ref(), bars)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtester() {
        let backtester = Backtester::new(10000.0, 0.001);
        assert_eq!(backtester.initial_capital, 10000.0);
    }
}
