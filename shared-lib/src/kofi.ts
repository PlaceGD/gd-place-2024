export type KofiTxId = `${number}-${number}-${number}-${number}`;
export const VALID_KOFI_TRANSACTION_ID = /^\d{8}-\d{4}-\d{4}-\d{4}-\d{12}$/;
