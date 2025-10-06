use rust_strategy_backtester::*;
use chrono::{Utc, Duration};

fn main() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║     Rust Strategy Backtester - Demo                 ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    // Generate sample data
    let mut bars = Vec::new();
    let start_date = Utc::now() - Duration::days(365);
    let mut price = 100.0;
    
    for i in 0..252 {  // 252 trading days
        let date = start_date + Duration::days(i);
        let change = (rand::random::<f64>() - 0.5) * 4.0;  // ±2% daily change
        price += change;
        
        let open = price;
        let high = price + rand::random::<f64>() * 2.0;
        let low = price - rand::random::<f64>() * 2.0;
        let close = price + (rand::random::<f64>() - 0.5);
        let volume = 1000000.0 + rand::random::<f64>() * 500000.0;
        
        bars.push(Bar::new(date, open, high, low, close, volume));
    }

    println!("📊 Generated {} bars of historical data\n", bars.len());

    // Test multiple strategies
    let strategies = vec![
        ("SMA 10/30", SimpleMovingAverageStrategy::new(10, 30)),
        ("SMA 20/50", SimpleMovingAverageStrategy::new(20, 50)),
        ("SMA 50/200", SimpleMovingAverageStrategy::new(50, 200)),
    ];

    println!("🔄 Testing {} strategies...\n", strategies.len());

    for (name, strategy) in strategies {
        let backtester = Backtester::new(10000.0, 0.001);
        let result = backtester.run(&strategy, &bars);

        println!("Strategy: {}", name);
        println!("├─ Total Return:  {:>8.2}%", result.total_return * 100.0);
        println!("├─ Sharpe Ratio:  {:>8.2}", result.sharpe_ratio);
        println!("├─ Max Drawdown:  {:>8.2}%", result.max_drawdown * 100.0);
        println!("├─ Win Rate:      {:>8.2}%", result.win_rate * 100.0);
        println!("├─ Profit Factor: {:>8.2}", result.profit_factor);
        println!("└─ Total Trades:  {:>8}\n", result.total_trades);
    }

    println!("✅ Backtesting complete!");
}
