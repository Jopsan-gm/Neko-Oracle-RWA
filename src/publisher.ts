// TODO: Uncomment when implementing actual Soroban contract interaction
// import {
//   Contract,
//   Networks,
//   SorobanRpc,
//   Keypair,
//   TransactionBuilder,
//   xdr,
// } from "@stellar/stellar-sdk";

import { Networks, Keypair } from "@stellar/stellar-sdk";

export interface PublishParams {
  assetId: string;
  price: number;
  timestamp: number;
  commit: string;
}

export interface PublishResult {
  txHash: string;
  success: boolean;
}

export class SorobanPublisher {
  private rpcUrl: string;
  private contractId: string;
  private keypair: Keypair | null;
  private publicKey: string;
  // private server: SorobanRpc.Server; // TODO: Uncomment when implementing
  private networkPassphrase: string;
  private maxRetries: number = 3;
  private retryDelay: number = 2000; // 2 seconds

  constructor(rpcUrl: string, contractId: string, secretKey: string) {
    this.rpcUrl = rpcUrl;
    this.contractId = contractId;

    // Only create Keypair if secret key is valid and not empty
    // Since we're in logging mode, we don't strictly need it
    if (secretKey && secretKey.trim() !== "") {
      try {
        this.keypair = Keypair.fromSecret(secretKey);
        this.publicKey = this.keypair.publicKey();
      } catch (error) {
        console.warn(
          "[PUBLISHER] Invalid secret key format. Using empty placeholder for logging mode."
        );
        this.keypair = null;
        this.publicKey = "(invalid key)";
      }
    } else {
      console.warn(
        "[PUBLISHER] Secret key not provided. Using placeholder for logging mode."
      );
      this.keypair = null;
      this.publicKey = "(no key provided)";
    }

    // TODO: Uncomment when implementing actual Soroban interaction
    // this.server = new SorobanRpc.Server(rpcUrl, {
    //   allowHttp: rpcUrl.startsWith("http://"),
    // });

    // Determine network passphrase based on RPC URL
    if (rpcUrl.includes("futurenet")) {
      this.networkPassphrase = Networks.FUTURENET;
    } else if (rpcUrl.includes("testnet")) {
      this.networkPassphrase = Networks.TESTNET;
    } else {
      this.networkPassphrase = Networks.PUBLIC;
    }
  }

  /**
   * Retry wrapper for RPC calls
   */
  private async retry<T>(
    fn: () => Promise<T>,
    retries: number = this.maxRetries
  ): Promise<T> {
    try {
      return await fn();
    } catch (error) {
      if (retries <= 0) {
        throw error;
      }
      console.warn(
        `RPC retry attempt ${this.maxRetries - retries + 1}/${this.maxRetries}`
      );
      await new Promise((resolve) => setTimeout(resolve, this.retryDelay));
      return this.retry(fn, retries - 1);
    }
  }

  // TODO: Uncomment when implementing actual Soroban contract interaction
  /**
   * Convert string to Soroban ScVal
   */
  // private stringToScVal(value: string): xdr.ScVal {
  //   return xdr.ScVal.scvString(value);
  // }

  /**
   * Convert number to Soroban ScVal (i128)
   */
  // private numberToScVal(value: number): xdr.ScVal {
  //   const valueBigInt = BigInt(value);
  //   const hi = valueBigInt >> BigInt(64);
  //   const lo = valueBigInt & BigInt("0xFFFFFFFFFFFFFFFF");
  //   return xdr.ScVal.scvI128(
  //     new xdr.Int128Parts({
  //       hi: xdr.Int64.fromString(hi.toString()),
  //       lo: xdr.Uint64.fromString(lo.toString()),
  //     })
  //   );
  // }

  /**
   * Publish price data to Soroban contract
   * TODO: Implement actual Soroban contract interaction
   * Currently only logs the data that would be published
   */
  async publishToOracle(params: PublishParams): Promise<PublishResult> {
    return this.retry(async () => {
      // Log the data that would be published
      console.log("[PUBLISHER] Would publish to Oracle contract:");
      console.log(`  Contract ID: ${this.contractId}`);
      console.log(`  RPC URL: ${this.rpcUrl}`);
      console.log(`  Network: ${this.networkPassphrase}`);
      console.log(`  Asset ID: ${params.assetId}`);
      console.log(`  Price: ${params.price} (${params.price / 1e7} raw)`);
      console.log(
        `  Timestamp: ${params.timestamp} (${new Date(
          params.timestamp * 1000
        ).toISOString()})`
      );
      console.log(`  Commit: ${params.commit}`);
      console.log(`  Signer: ${this.publicKey}`);

      // Simulate transaction hash
      const mockTxHash = "0".repeat(64); // Mock 64-char hex hash

      // TODO: Uncomment and implement actual Soroban transaction
      /*
      const contract = new Contract(this.contractId);
      const sourceAccount = await this.server.getAccount(
        this.keypair.publicKey()
      );

      // Build contract method call arguments as ScVal
      const methodArgs = [
        this.stringToScVal(params.assetId),
        this.numberToScVal(params.price),
        this.numberToScVal(params.timestamp),
        this.stringToScVal(params.commit),
      ];

      // Build transaction with contract invocation
      const transactionBuilder = new TransactionBuilder(sourceAccount, {
        fee: "100", // Base fee
        networkPassphrase: this.networkPassphrase,
      });

      const operation = contract.call("update_price", ...methodArgs);
      transactionBuilder.addOperation(operation);
      transactionBuilder.setTimeout(30);

      // Build the transaction
      let transaction = transactionBuilder.build();

      // Simulate transaction to get resource estimates
      const simulateResult = await this.server.simulateTransaction(transaction);

      if (SorobanRpc.Api.isSimulationError(simulateResult)) {
        throw new Error(`Simulation error: ${JSON.stringify(simulateResult)}`);
      }

      // Assemble transaction (add simulation results)
      // Note: assembleTransaction helper may vary by SDK version
      // If this fails, you may need to manually set resources using:
      // transaction.setSorobanData(simulateResult.transactionData.build())
      let assembledTransaction: any;
      if (typeof SorobanRpc.assembleTransaction === "function") {
        assembledTransaction = SorobanRpc.assembleTransaction(
          transaction,
          simulateResult
        ).build();
      } else {
        // Fallback: manually set resources
        transaction.setSorobanData(simulateResult.transactionData.build());
        assembledTransaction = transaction;
      }

      // Sign transaction
      assembledTransaction.sign(this.keypair);

      // Send transaction
      const sendResult = await this.server.sendTransaction(
        assembledTransaction
      );

      if (sendResult.status === "ERROR") {
        throw new Error(
          `Transaction send error: ${JSON.stringify(sendResult)}`
        );
      }

      // Wait for transaction to be confirmed (poll)
      let getTransactionResult = await this.server.getTransaction(
        sendResult.hash
      );
      const pollLimit = 10;
      let pollCount = 0;

      while (
        getTransactionResult.status ===
          SorobanRpc.Api.GetTransactionStatus.NOT_FOUND &&
        pollCount < pollLimit
      ) {
        await new Promise((resolve) => setTimeout(resolve, 1000));
        getTransactionResult = await this.server.getTransaction(
          sendResult.hash
        );
        pollCount++;
      }

      if (
        getTransactionResult.status ===
        SorobanRpc.Api.GetTransactionStatus.FAILED
      ) {
        const resultXdr = getTransactionResult.resultXdr;
        throw new Error(`Transaction failed: ${resultXdr}`);
      }

      if (
        getTransactionResult.status ===
        SorobanRpc.Api.GetTransactionStatus.NOT_FOUND
      ) {
        throw new Error("Transaction not found after polling");
      }

      return {
        txHash: sendResult.hash,
        success: true,
      };
      */

      console.log(`[PUBLISHER] Mock transaction hash: ${mockTxHash}`);
      console.log("[PUBLISHER] (No actual transaction sent - logging only)");

      return {
        txHash: mockTxHash,
        success: true,
      };
    });
  }
}
