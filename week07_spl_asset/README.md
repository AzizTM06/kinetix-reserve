# Модуль 7. Выпуск и управление SPL-токеном (`KTX`)

Набор TypeScript-сценариев для полного жизненного цикла токена стандарта SPL: регистрация Mint-аккаунта (6 знаков после запятой), привязка метаданных Metaplex Token Metadata V3 (`Kinetix Protocol Token`, тикер `KTX`), первичная эмиссия 250 000 токенов на Associated Token Account (ATA), перевод 4 250 токенов на адрес контрагента и аудит эмиссии в блокчейне.

## Команды запуска

```bash
cd week07_spl_asset
npx tsc --noEmit
npm start
```

Поэтапный запуск: `npm run init-mint`, `npm run metadata`, `npm run issue`, `npm run transfer`, `npm run audit`.
