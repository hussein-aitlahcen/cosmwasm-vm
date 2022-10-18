import { Binary, Coin, ContractInfoResponse, ContractMeta, CosmWasmEvent, Env, MessageInfo, Option, Order, Result, Unit } from "./common"
import { vm_query, vm_instantiate, vm_execute, vm_migrate, vm_continue_instantiate, vm_continue_execute, vm_continue_migrate } from "../../typescript-bindings/pkg/typescript_bindings";

export type VMStep = {
  data: Option<Binary>,
  events: Array<CosmWasmEvent>
};

export type StorageKey = Array<number>;
export type StorageValue = Array<number>;
export type StorageIterator = number;

export type VM = {
  // Environment
  info: () => MessageInfo,
  env: () => Env,

  // Running contract state
  running_contract_meta: () => Result<ContractMeta, Error>,
  set_contract_meta: (address: string, metadata: ContractMeta) => Result<Unit, Error>,

  // Transactional
  transaction_begin: () => Result<Unit, Error>,
  transaction_commit: () => Result<Unit, Error>,
  transaction_rollback: () => Result<Unit, Error>,

  // Gas
  // TODO: add strongly typed VmGas
  charge: (value: any) => Result<Unit, Error>,
  gas_checkpoint_push: () => Result<Unit, Error>,
  gas_checkpoint_pop: () => Result<Unit, Error>,
  gas_ensure_available: () => Result<Unit, Error>,

  // Address validation
  addr_validate: (addr: string) => Result<Result<Unit, Error>, Error>,
  addr_canonicalize: (addr: string) => Result<Result<Array<number>, Error>, Error>,
  addr_humanize: (addr: Array<number>) => Result<Result<string, Error>, Error>,

  // Database
  db_write: (key: StorageKey, value: StorageValue) => Result<Unit, Error>,
  db_read: (key: StorageKey) => Result<StorageValue, Error>,
  db_remove: (key: StorageKey) => Result<Unit, Error>,
  db_scan: (start: Option<StorageKey>, end: Option<StorageKey>, order: Order) => Result<StorageIterator, Error>,
  db_next: (iterator_id: StorageIterator) => Result<[StorageKey, StorageValue], Error>,

  // Execution
  query_continuation: (address: string, message: Array<number>) => Result<Result<Binary, Error>, Error>,
  continue_instantiate: (metadata: ContractMeta, funds: Array<Coin>, message: Array<number>, event_handler: any) => Result<[string, Option<Binary>], Error>,
  continue_execute: (address: string, funds: Array<Coin>, message: Array<number>, event_handler: any) => Result<Option<Binary>, Error>,
  continue_migrate: (address: string, funds: Array<Coin>, message: Array<number>, event_handler: any) => Result<Option<Binary>, Error>,
  query_raw: (address: string, key: StorageKey) => Result<Option<Array<number>>, Error>,
  query_info: (address: string) => Result<ContractInfoResponse, Error>,

  // Bank
  transfer: (to: string, funds: Array<Coin>) => Result<Unit, Error>,
  burn: (funds: Array<Coin>) => Result<Unit, Error>,
  balance: (address: string, denom: string) => Result<Coin, Error>,
  all_balance: (address: string) => Result<Array<Coin>, Error>,

  // Debug
  abort: (message: string) => Result<Unit, Error>,
  debug: (message: Array<number>) => Result<Unit, Error>,

  // TODO: crypto functions
}

export const vmInstantiate = <T>(host: VM, code: Uint8Array, message: T): Result<VMStep, Error> =>
  vm_instantiate(host, code, JSON.stringify(message));

export const vmExecute = <T>(host: VM, code: Uint8Array, message: T): Result<VMStep, Error> =>
  vm_execute(host, code, JSON.stringify(message));

export const vmMigrate = <T>(host: VM, code: Uint8Array, message: T): Result<VMStep, Error> =>
  vm_migrate(host, code, JSON.stringify(message));

export const vmContinueInstantiate = <T>(host: VM, code: Uint8Array, message: T, event_handler: any): Result<Option<Binary>, Error> =>
  vm_continue_instantiate(host, code, JSON.stringify(message), event_handler);

export const vmContinueExecute = <T>(host: VM, code: Uint8Array, message: T, event_handler: any): Result<Option<Binary>, Error> =>
  vm_continue_execute(host, code, JSON.stringify(message), event_handler);

export const vmContinueMigrate = <T>(host: VM, code: Uint8Array, message: T, event_handler: any): Result<Option<Binary>, Error> =>
  vm_continue_migrate(host, code, JSON.stringify(message), event_handler);

export const vmQuery = <T>(host: VM, code: Uint8Array, query: T): Result<Result<Result<Binary, Error>, Error>, Error> =>
  vm_query(host, code, JSON.stringify(query));
