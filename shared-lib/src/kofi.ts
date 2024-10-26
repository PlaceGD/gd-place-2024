export type KofiTxId = `${number}-${number}-${number}-${number}`;
export const VALID_KOFI_TRANSACTION_ID =
    /^[A-Fa-f0-9]{8}-[A-Fa-f0-9]{4}-[A-Fa-f0-9]{4}-[A-Fa-f0-9]{4}-[A-Fa-f0-9]{12}$/;
export const VALID_KOFI_TRANSACTION_ID_CHARS = /^[A-Fa-f0-9-]*$/;

export const MAX_GRADIENT_STOPS = 10;
