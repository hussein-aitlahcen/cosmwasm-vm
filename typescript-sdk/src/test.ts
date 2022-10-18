import { ContractMeta, Coin, Ok, Err, None, encode, decode, toHex, toBinary } from "./common";
import * as fs from "fs";
import { VM, vmInstantiate, vmExecute, vmContinueInstantiate } from "./vm";

const main = async () => {
  const code = new Uint8Array(fs.readFileSync("./reflect.wasm"));
  const db = new Map<String, Object>();
  const host: Partial<VM> = {
    info: () => ({
      sender: "1",
      funds: [],
    }),
    env: () => ({
      block: {
        height: 1,
        time: "0",
        chain_id: "frontend-chain",
      },
      transaction: None(),
      contract: {
        address: "1",
      }
    }),
    transaction_begin: () => {
      return Ok(None());
    },
    transaction_rollback: () => {
      return Ok(None());
    },
    transaction_commit: () => {
      return Ok(None());
    },
    charge: (_value: object) => {
      return Ok(None());
    },
    addr_validate: (_input: string) => {
      return Ok(Ok(None()));
    },
    db_write: (key: Array<number>, value: Array<number>) => {
      db.set(toHex(key), JSON.parse(decode(value)));
      return Ok(None());
    },
    db_read: (key: Array<number>) => Ok(encode(db.get(toHex(key)))),
    running_contract_meta: () => {
      return {
        Ok: {
          code_id: 0,
          admin: None<string>(),
          label: "label",
        }
      };
    },
    gas_checkpoint_push: () => Ok(None()),
    gas_checkpoint_pop: () => Ok(None()),
    continue_instantiate: (_contractMeta: ContractMeta, _funds: Array<Coin>, message: Array<number>, event_handler: any) => {
      // TODO: normally we reload a new host with the new meta for the newly running contract, we update the env to reflect sender/funds
      const result = vmContinueInstantiate(<VM>host, code, JSON.parse(decode(message)), event_handler);
      if ("Ok" in result) {
        // new contract address and result of instantiate call
        return Ok(["2", result.Ok]);
      }
      else if (result.Err) {
        return Err(result.Err);
      }
    }
  };
  const result = vmInstantiate(<VM>host, code, {});
  console.log(JSON.stringify(result));
  console.log(db);
  const result1 = vmExecute(<VM>host, code, {
    reflect_msg: {
      msgs: [{
        wasm: {
          instantiate: {
            admin: None,
            code_id: 1,
            msg: toBinary(JSON.stringify({})),
            funds: [],
            label: "hello"
          }
        }
      }]
    }
  });
  console.log(JSON.stringify(result1));
  console.log(db);
};

main();
