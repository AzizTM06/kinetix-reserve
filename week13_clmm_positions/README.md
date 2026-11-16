# Модуль 13. Жизненный цикл концентрированной позиции Raydium CLMM

Управление позициями ликвидности из смарт-контракта `Kinetix Reserve`: открытие диапазона с выпуском Token-2022 Position NFT на имя PDA `ReservePool` (`deploy_range_position`), накопление торговых комиссий внутри тикового интервала и закрытие с выводом средств и возвратом арендного депозита (`unwind_range_position`).

## Проверка

```bash
cd week13_clmm_positions
cargo test
```
