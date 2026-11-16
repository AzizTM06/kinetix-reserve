# Модуль 8. Базовый каркас протокола `Kinetix Reserve`

Инициализация корневых структур данных и PDA-аккаунтов казначейского пула.

## Модель аккаунтов

| Аккаунт | Семена PDA | Размер (байт) | Назначение |
| :--- | :--- | :---: | :--- |
| `ReservePool` | `["kinetix_pool", base_mint]` | 156 | Параметры пула, куратор, учет `managed_assets` и `issued_receipts`, комиссия `fee_bps`, флаг паузы |
| `reserve_account` | `["reserve_vault", base_mint]` | SPL Token | Хранилище базовых SPL-токенов под управлением PDA `ReservePool` |
| `receipt_mint` | `["receipt_mint", base_mint]` | SPL Mint | Минт квитанций участия (6 decimals, `mint::authority = pool`) |
| `MemberLedger` | `["member_ledger", pool, participant]` | 97 | Персональный учет взносов, остатка квитанций и времени последней операции (`Clock`) |

## Проверка

```bash
cd week08_reserve_core
cargo test
```
