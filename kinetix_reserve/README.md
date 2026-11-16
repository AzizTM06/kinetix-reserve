# Kinetix Reserve — Протокол программируемого казначейства и управления концентрированной ликвидностью

Сквозная программа на фреймворке Anchor (`anchor-lang 0.31.1`) для управления коллективным резервом SPL-активов с выпуском квитанционных долей (receipt tokens), защитой от инфляционных атак первого депозита и интеграцией с Raydium CLMM через CPI.

## Инструкции программы (`kinetix_reserve`)

1. `bootstrap_pool(fee_bps)` — создает PDA `ReservePool` (`[b"kinetix_pool", base_mint]`, 156 байт), хранилище `reserve_account` (`[b"reserve_vault", base_mint]`) и минт квитанций `receipt_mint` (`[b"receipt_mint", base_mint]`).
2. `register_member()` — инициализирует учетную запись участника `MemberLedger` (`[b"member_ledger", pool, participant]`, 97 байт) с фиксацией временной метки через `Clock`.
3. `contribute_liquidity(amount)` — переводит базовые токены в резерв, вычисляет число квитанций с виртуальным смещением `VIRTUAL_OFFSET = 1000` и чеканит их на аккаунт участника с подписью PDA пула.
4. `redeem_liquidity(receipts_amount)` — сжигает квитанции пользователя и возвращает базовые активы из `reserve_account` за вычетом комиссии выхода (`fee_bps`), которая остается в резерве в пользу оставшихся держателей.
5. `set_pool_pause(paused)` — аварийная приостановка притока ликвидности и ребалансировок куратором пула.
6. `execute_rebalance_swap(amount_in, min_amount_out, sqrt_price_limit_x64)` — CPI в `raydium_clmm_cpi::cpi::swap_v2` с временным делегированием (`Approve`/`Revoke`), порогом проскальзывания и ценовым лимитом `Q64.64`.
7. `deploy_range_position(...)` — открытие концентрированной позиции в Raydium CLMM (Token-2022 Position NFT, принадлежащий PDA `ReservePool`).
8. `unwind_range_position(...)` — вывод ликвидности и комиссий (`decrease_liquidity_v2`) с последующим сжиганием Position NFT (`close_position`).

## Тестирование

```bash
cd kinetix_reserve
cargo test
```
