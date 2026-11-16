# Практический модуль Helius RPC & SPL Token Lifecycle

Интеграция с выделенным RPC-узлом Helius на кластере Solana Devnet через `@solana/web3.js` и `@solana/spl-token`.

## Состав

- `src/rpc.ts` — конфигурация подключения из `.env` (`HELIUS_RPC_URL` / `HELIUS_API_KEY`) с безопасной загрузкой ключа без вывода секретов в консоль.
- `src/memo_dispatch.ts` (`npm run memo`) — атомарная транзакция перевода `0.015 SOL` с записью метки расчетного пакета в `MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr`.
- `src/ktx_token_lab.ts` (`npm run token-lab`) — выпуск токена `KTXLAB` (8 decimals), эмиссия 50 000 единиц, рассылка по 250 единиц на три адреса, сжигание 125 единиц и сверка остатка (49 125).

## Проверка

```bash
cd helius_rpc_lab
npx tsc --noEmit
npm run memo
npm run token-lab
```
