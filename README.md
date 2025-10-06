# 📈 Rust Strategy Backtester

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Parallel](https://img.shields.io/badge/Parallel-Rayon-green.svg)]()

[English](#english) | [Português](#português)

---

## English

### Overview

A high-performance backtesting framework for trading strategies built with Rust. Supports parallel execution of multiple strategies with comprehensive performance metrics including Sharpe ratio, maximum drawdown, win rate, and profit factor.

### Key Features

- **Parallel Execution**: Test multiple strategies simultaneously using Rayon
- **Performance Metrics**: Sharpe ratio, max drawdown, win rate, profit factor, VWAP
- **Flexible Strategy API**: Simple trait-based strategy definition
- **Commission Modeling**: Realistic transaction cost simulation
- **Trade History**: Complete record of all trades with entry/exit details

### Quick Start

```rust
use rust_strategy_backtester::{Backtester, SimpleMovingAverageStrategy, Bar};
use chrono::Utc;

let bars = vec![
    Bar::new(Utc::now(), 100.0, 105.0, 98.0, 102.0, 1000.0),
    Bar::new(Utc::now(), 102.0, 108.0, 101.0, 106.0, 1200.0),
];

let strategy = SimpleMovingAverageStrategy::new(20, 50);
let backtester = Backtester::new(10000.0, 0.001);
let result = backtester.run(&strategy, &bars);

println!("Total return: {:.2}%", result.total_return * 100.0);
println!("Sharpe ratio: {:.2}", result.sharpe_ratio);
println!("Max drawdown: {:.2}%", result.max_drawdown * 100.0);
```

### Performance Metrics

- **Total Return**: Overall percentage gain/loss
- **Sharpe Ratio**: Risk-adjusted return (annualized)
- **Maximum Drawdown**: Largest peak-to-trough decline
- **Win Rate**: Percentage of profitable trades
- **Profit Factor**: Gross profit / gross loss ratio

### Use Cases

- **Strategy Development**: Test and validate trading ideas
- **Parameter Optimization**: Find optimal strategy parameters
- **Risk Analysis**: Evaluate strategy risk characteristics
- **Performance Comparison**: Compare multiple strategies
- **Educational**: Learn quantitative trading concepts

### Author

**Gabriel Demetrios Lafis**

---

## Português

### Visão Geral

Um framework de backtesting de alta performance para estratégias de trading construído com Rust. Suporta execução paralela de múltiplas estratégias com métricas abrangentes de performance incluindo índice de Sharpe, drawdown máximo, taxa de acerto e fator de lucro.

### Características Principais

- **Execução Paralela**: Teste múltiplas estratégias simultaneamente usando Rayon
- **Métricas de Performance**: Índice de Sharpe, drawdown máximo, taxa de acerto, fator de lucro
- **API Flexível de Estratégia**: Definição de estratégia baseada em trait simples
- **Modelagem de Comissões**: Simulação realista de custos de transação
- **Histórico de Trades**: Registro completo de todos os trades com detalhes de entrada/saída

### Início Rápido

```rust
use rust_strategy_backtester::{Backtester, SimpleMovingAverageStrategy, Bar};
use chrono::Utc;

let bars = vec![
    Bar::new(Utc::now(), 100.0, 105.0, 98.0, 102.0, 1000.0),
    Bar::new(Utc::now(), 102.0, 108.0, 101.0, 106.0, 1200.0),
];

let strategy = SimpleMovingAverageStrategy::new(20, 50);
let backtester = Backtester::new(10000.0, 0.001);
let result = backtester.run(&strategy, &bars);

println!("Retorno total: {:.2}%", result.total_return * 100.0);
println!("Índice de Sharpe: {:.2}", result.sharpe_ratio);
println!("Drawdown máximo: {:.2}%", result.max_drawdown * 100.0);
```

### Casos de Uso

- **Desenvolvimento de Estratégias**: Testar e validar ideias de trading
- **Otimização de Parâmetros**: Encontrar parâmetros ótimos de estratégia
- **Análise de Risco**: Avaliar características de risco da estratégia
- **Comparação de Performance**: Comparar múltiplas estratégias
- **Educacional**: Aprender conceitos de trading quantitativo

### Autor

**Gabriel Demetrios Lafis**
