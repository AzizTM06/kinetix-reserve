# Модуль 12. Маршрутизация свопов через Raydium CLMM CPI (`execute_rebalance_swap`)

Интеграция с программой Raydium CLMM (`devi51mZmdwUJGU9hjN27vEz64Gps7uUefqxg27EAtH`) для ребалансировки активов резерва через `raydium_clmm_cpi::cpi::swap_v2`.

## Защита сделки

- Передача явного лимита `min_amount_out` (`other_amount_threshold`) ограничивает допустимое проскальзывание.
- Параметр `sqrt_price_limit_x64` задает ценовую отсечку в формате `Q64.64`, останавливая исполнение при выходе цены за пределы допустимого коридора.
- Временное делегирование `Approve` для токенов резерва автоматически отзывается (`Revoke`) сразу после завершения CPI.

## Проверка

```bash
cd week12_clmm_router
cargo test
```
