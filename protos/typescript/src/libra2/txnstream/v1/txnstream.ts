/* eslint-disable */
import {
  ChannelCredentials,
  Client,
  ClientReadableStream,
  handleServerStreamingCall,
  makeGenericClientConstructor,
  Metadata,
} from "@grpc/grpc-js";
import type {
  CallOptions,
  ClientOptions,
  ClientUnaryCall,
  handleUnaryCall,
  ServiceError,
  UntypedServiceImplementation,
} from "@grpc/grpc-js";
import Long from "long";
import _m0 from "protobufjs/minimal";

/** protos/proto/libra2/txnstream/v1/txnstream.proto */

export interface Empty {
}

export interface ChainIdResponse {
  chainId?: number | undefined;
}

export interface ServerInfoResponse {
  version?: string | undefined;
  buildTimestamp?: bigint | undefined;
}

export interface TransactionsRequest {
  startVersion?: bigint | undefined;
  includeEvents?:
    | boolean
    | undefined;
  /** server clamps to sane max (e.g., 1000) */
  batchSize?:
    | number
    | undefined;
  /** long-poll; 0 = immediate */
  maxWaitMs?: number | undefined;
  excludeLedgerChanges?: boolean | undefined;
}

export interface TransactionOutput {
  version?:
    | bigint
    | undefined;
  /** bcs-encoded transaction */
  txn?:
    | Uint8Array
    | undefined;
  /** bcs-encoded events */
  events?:
    | Uint8Array[]
    | undefined;
  /** bcs-encoded TransactionInfo */
  info?: Uint8Array | undefined;
}

export interface TransactionsResponse {
  batch?: TransactionOutput[] | undefined;
}

function createBaseEmpty(): Empty {
  return {};
}

export const Empty = {
  encode(_: Empty, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): Empty {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseEmpty();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  // encodeTransform encodes a source of message objects.
  // Transform<Empty, Uint8Array>
  async *encodeTransform(
    source: AsyncIterable<Empty | Empty[]> | Iterable<Empty | Empty[]>,
  ): AsyncIterable<Uint8Array> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [Empty.encode(p).finish()];
        }
      } else {
        yield* [Empty.encode(pkt as any).finish()];
      }
    }
  },

  // decodeTransform decodes a source of encoded messages.
  // Transform<Uint8Array, Empty>
  async *decodeTransform(
    source: AsyncIterable<Uint8Array | Uint8Array[]> | Iterable<Uint8Array | Uint8Array[]>,
  ): AsyncIterable<Empty> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [Empty.decode(p)];
        }
      } else {
        yield* [Empty.decode(pkt as any)];
      }
    }
  },

  fromJSON(_: any): Empty {
    return {};
  },

  toJSON(_: Empty): unknown {
    const obj: any = {};
    return obj;
  },

  create(base?: DeepPartial<Empty>): Empty {
    return Empty.fromPartial(base ?? {});
  },
  fromPartial(_: DeepPartial<Empty>): Empty {
    const message = createBaseEmpty();
    return message;
  },
};

function createBaseChainIdResponse(): ChainIdResponse {
  return { chainId: 0 };
}

export const ChainIdResponse = {
  encode(message: ChainIdResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.chainId !== undefined && message.chainId !== 0) {
      writer.uint32(8).uint32(message.chainId);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ChainIdResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseChainIdResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.chainId = reader.uint32();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  // encodeTransform encodes a source of message objects.
  // Transform<ChainIdResponse, Uint8Array>
  async *encodeTransform(
    source: AsyncIterable<ChainIdResponse | ChainIdResponse[]> | Iterable<ChainIdResponse | ChainIdResponse[]>,
  ): AsyncIterable<Uint8Array> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [ChainIdResponse.encode(p).finish()];
        }
      } else {
        yield* [ChainIdResponse.encode(pkt as any).finish()];
      }
    }
  },

  // decodeTransform decodes a source of encoded messages.
  // Transform<Uint8Array, ChainIdResponse>
  async *decodeTransform(
    source: AsyncIterable<Uint8Array | Uint8Array[]> | Iterable<Uint8Array | Uint8Array[]>,
  ): AsyncIterable<ChainIdResponse> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [ChainIdResponse.decode(p)];
        }
      } else {
        yield* [ChainIdResponse.decode(pkt as any)];
      }
    }
  },

  fromJSON(object: any): ChainIdResponse {
    return { chainId: isSet(object.chainId) ? globalThis.Number(object.chainId) : 0 };
  },

  toJSON(message: ChainIdResponse): unknown {
    const obj: any = {};
    if (message.chainId !== undefined && message.chainId !== 0) {
      obj.chainId = Math.round(message.chainId);
    }
    return obj;
  },

  create(base?: DeepPartial<ChainIdResponse>): ChainIdResponse {
    return ChainIdResponse.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<ChainIdResponse>): ChainIdResponse {
    const message = createBaseChainIdResponse();
    message.chainId = object.chainId ?? 0;
    return message;
  },
};

function createBaseServerInfoResponse(): ServerInfoResponse {
  return { version: "", buildTimestamp: BigInt("0") };
}

export const ServerInfoResponse = {
  encode(message: ServerInfoResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.version !== undefined && message.version !== "") {
      writer.uint32(10).string(message.version);
    }
    if (message.buildTimestamp !== undefined && message.buildTimestamp !== BigInt("0")) {
      if (BigInt.asUintN(64, message.buildTimestamp) !== message.buildTimestamp) {
        throw new globalThis.Error("value provided for field message.buildTimestamp of type uint64 too large");
      }
      writer.uint32(16).uint64(message.buildTimestamp.toString());
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): ServerInfoResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseServerInfoResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.version = reader.string();
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.buildTimestamp = longToBigint(reader.uint64() as Long);
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  // encodeTransform encodes a source of message objects.
  // Transform<ServerInfoResponse, Uint8Array>
  async *encodeTransform(
    source:
      | AsyncIterable<ServerInfoResponse | ServerInfoResponse[]>
      | Iterable<ServerInfoResponse | ServerInfoResponse[]>,
  ): AsyncIterable<Uint8Array> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [ServerInfoResponse.encode(p).finish()];
        }
      } else {
        yield* [ServerInfoResponse.encode(pkt as any).finish()];
      }
    }
  },

  // decodeTransform decodes a source of encoded messages.
  // Transform<Uint8Array, ServerInfoResponse>
  async *decodeTransform(
    source: AsyncIterable<Uint8Array | Uint8Array[]> | Iterable<Uint8Array | Uint8Array[]>,
  ): AsyncIterable<ServerInfoResponse> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [ServerInfoResponse.decode(p)];
        }
      } else {
        yield* [ServerInfoResponse.decode(pkt as any)];
      }
    }
  },

  fromJSON(object: any): ServerInfoResponse {
    return {
      version: isSet(object.version) ? globalThis.String(object.version) : "",
      buildTimestamp: isSet(object.buildTimestamp) ? BigInt(object.buildTimestamp) : BigInt("0"),
    };
  },

  toJSON(message: ServerInfoResponse): unknown {
    const obj: any = {};
    if (message.version !== undefined && message.version !== "") {
      obj.version = message.version;
    }
    if (message.buildTimestamp !== undefined && message.buildTimestamp !== BigInt("0")) {
      obj.buildTimestamp = message.buildTimestamp.toString();
    }
    return obj;
  },

  create(base?: DeepPartial<ServerInfoResponse>): ServerInfoResponse {
    return ServerInfoResponse.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<ServerInfoResponse>): ServerInfoResponse {
    const message = createBaseServerInfoResponse();
    message.version = object.version ?? "";
    message.buildTimestamp = object.buildTimestamp ?? BigInt("0");
    return message;
  },
};

function createBaseTransactionsRequest(): TransactionsRequest {
  return { startVersion: BigInt("0"), includeEvents: false, batchSize: 0, maxWaitMs: 0, excludeLedgerChanges: false };
}

export const TransactionsRequest = {
  encode(message: TransactionsRequest, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.startVersion !== undefined && message.startVersion !== BigInt("0")) {
      if (BigInt.asUintN(64, message.startVersion) !== message.startVersion) {
        throw new globalThis.Error("value provided for field message.startVersion of type uint64 too large");
      }
      writer.uint32(8).uint64(message.startVersion.toString());
    }
    if (message.includeEvents === true) {
      writer.uint32(16).bool(message.includeEvents);
    }
    if (message.batchSize !== undefined && message.batchSize !== 0) {
      writer.uint32(24).uint32(message.batchSize);
    }
    if (message.maxWaitMs !== undefined && message.maxWaitMs !== 0) {
      writer.uint32(32).uint32(message.maxWaitMs);
    }
    if (message.excludeLedgerChanges === true) {
      writer.uint32(40).bool(message.excludeLedgerChanges);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TransactionsRequest {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTransactionsRequest();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.startVersion = longToBigint(reader.uint64() as Long);
          continue;
        case 2:
          if (tag !== 16) {
            break;
          }

          message.includeEvents = reader.bool();
          continue;
        case 3:
          if (tag !== 24) {
            break;
          }

          message.batchSize = reader.uint32();
          continue;
        case 4:
          if (tag !== 32) {
            break;
          }

          message.maxWaitMs = reader.uint32();
          continue;
        case 5:
          if (tag !== 40) {
            break;
          }

          message.excludeLedgerChanges = reader.bool();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  // encodeTransform encodes a source of message objects.
  // Transform<TransactionsRequest, Uint8Array>
  async *encodeTransform(
    source:
      | AsyncIterable<TransactionsRequest | TransactionsRequest[]>
      | Iterable<TransactionsRequest | TransactionsRequest[]>,
  ): AsyncIterable<Uint8Array> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [TransactionsRequest.encode(p).finish()];
        }
      } else {
        yield* [TransactionsRequest.encode(pkt as any).finish()];
      }
    }
  },

  // decodeTransform decodes a source of encoded messages.
  // Transform<Uint8Array, TransactionsRequest>
  async *decodeTransform(
    source: AsyncIterable<Uint8Array | Uint8Array[]> | Iterable<Uint8Array | Uint8Array[]>,
  ): AsyncIterable<TransactionsRequest> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [TransactionsRequest.decode(p)];
        }
      } else {
        yield* [TransactionsRequest.decode(pkt as any)];
      }
    }
  },

  fromJSON(object: any): TransactionsRequest {
    return {
      startVersion: isSet(object.startVersion) ? BigInt(object.startVersion) : BigInt("0"),
      includeEvents: isSet(object.includeEvents) ? globalThis.Boolean(object.includeEvents) : false,
      batchSize: isSet(object.batchSize) ? globalThis.Number(object.batchSize) : 0,
      maxWaitMs: isSet(object.maxWaitMs) ? globalThis.Number(object.maxWaitMs) : 0,
      excludeLedgerChanges: isSet(object.excludeLedgerChanges)
        ? globalThis.Boolean(object.excludeLedgerChanges)
        : false,
    };
  },

  toJSON(message: TransactionsRequest): unknown {
    const obj: any = {};
    if (message.startVersion !== undefined && message.startVersion !== BigInt("0")) {
      obj.startVersion = message.startVersion.toString();
    }
    if (message.includeEvents === true) {
      obj.includeEvents = message.includeEvents;
    }
    if (message.batchSize !== undefined && message.batchSize !== 0) {
      obj.batchSize = Math.round(message.batchSize);
    }
    if (message.maxWaitMs !== undefined && message.maxWaitMs !== 0) {
      obj.maxWaitMs = Math.round(message.maxWaitMs);
    }
    if (message.excludeLedgerChanges === true) {
      obj.excludeLedgerChanges = message.excludeLedgerChanges;
    }
    return obj;
  },

  create(base?: DeepPartial<TransactionsRequest>): TransactionsRequest {
    return TransactionsRequest.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<TransactionsRequest>): TransactionsRequest {
    const message = createBaseTransactionsRequest();
    message.startVersion = object.startVersion ?? BigInt("0");
    message.includeEvents = object.includeEvents ?? false;
    message.batchSize = object.batchSize ?? 0;
    message.maxWaitMs = object.maxWaitMs ?? 0;
    message.excludeLedgerChanges = object.excludeLedgerChanges ?? false;
    return message;
  },
};

function createBaseTransactionOutput(): TransactionOutput {
  return { version: BigInt("0"), txn: new Uint8Array(0), events: [], info: new Uint8Array(0) };
}

export const TransactionOutput = {
  encode(message: TransactionOutput, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.version !== undefined && message.version !== BigInt("0")) {
      if (BigInt.asUintN(64, message.version) !== message.version) {
        throw new globalThis.Error("value provided for field message.version of type uint64 too large");
      }
      writer.uint32(8).uint64(message.version.toString());
    }
    if (message.txn !== undefined && message.txn.length !== 0) {
      writer.uint32(18).bytes(message.txn);
    }
    if (message.events !== undefined && message.events.length !== 0) {
      for (const v of message.events) {
        writer.uint32(26).bytes(v!);
      }
    }
    if (message.info !== undefined && message.info.length !== 0) {
      writer.uint32(34).bytes(message.info);
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TransactionOutput {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTransactionOutput();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 8) {
            break;
          }

          message.version = longToBigint(reader.uint64() as Long);
          continue;
        case 2:
          if (tag !== 18) {
            break;
          }

          message.txn = reader.bytes();
          continue;
        case 3:
          if (tag !== 26) {
            break;
          }

          message.events!.push(reader.bytes());
          continue;
        case 4:
          if (tag !== 34) {
            break;
          }

          message.info = reader.bytes();
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  // encodeTransform encodes a source of message objects.
  // Transform<TransactionOutput, Uint8Array>
  async *encodeTransform(
    source: AsyncIterable<TransactionOutput | TransactionOutput[]> | Iterable<TransactionOutput | TransactionOutput[]>,
  ): AsyncIterable<Uint8Array> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [TransactionOutput.encode(p).finish()];
        }
      } else {
        yield* [TransactionOutput.encode(pkt as any).finish()];
      }
    }
  },

  // decodeTransform decodes a source of encoded messages.
  // Transform<Uint8Array, TransactionOutput>
  async *decodeTransform(
    source: AsyncIterable<Uint8Array | Uint8Array[]> | Iterable<Uint8Array | Uint8Array[]>,
  ): AsyncIterable<TransactionOutput> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [TransactionOutput.decode(p)];
        }
      } else {
        yield* [TransactionOutput.decode(pkt as any)];
      }
    }
  },

  fromJSON(object: any): TransactionOutput {
    return {
      version: isSet(object.version) ? BigInt(object.version) : BigInt("0"),
      txn: isSet(object.txn) ? bytesFromBase64(object.txn) : new Uint8Array(0),
      events: globalThis.Array.isArray(object?.events) ? object.events.map((e: any) => bytesFromBase64(e)) : [],
      info: isSet(object.info) ? bytesFromBase64(object.info) : new Uint8Array(0),
    };
  },

  toJSON(message: TransactionOutput): unknown {
    const obj: any = {};
    if (message.version !== undefined && message.version !== BigInt("0")) {
      obj.version = message.version.toString();
    }
    if (message.txn !== undefined && message.txn.length !== 0) {
      obj.txn = base64FromBytes(message.txn);
    }
    if (message.events?.length) {
      obj.events = message.events.map((e) => base64FromBytes(e));
    }
    if (message.info !== undefined && message.info.length !== 0) {
      obj.info = base64FromBytes(message.info);
    }
    return obj;
  },

  create(base?: DeepPartial<TransactionOutput>): TransactionOutput {
    return TransactionOutput.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<TransactionOutput>): TransactionOutput {
    const message = createBaseTransactionOutput();
    message.version = object.version ?? BigInt("0");
    message.txn = object.txn ?? new Uint8Array(0);
    message.events = object.events?.map((e) => e) || [];
    message.info = object.info ?? new Uint8Array(0);
    return message;
  },
};

function createBaseTransactionsResponse(): TransactionsResponse {
  return { batch: [] };
}

export const TransactionsResponse = {
  encode(message: TransactionsResponse, writer: _m0.Writer = _m0.Writer.create()): _m0.Writer {
    if (message.batch !== undefined && message.batch.length !== 0) {
      for (const v of message.batch) {
        TransactionOutput.encode(v!, writer.uint32(10).fork()).ldelim();
      }
    }
    return writer;
  },

  decode(input: _m0.Reader | Uint8Array, length?: number): TransactionsResponse {
    const reader = input instanceof _m0.Reader ? input : _m0.Reader.create(input);
    let end = length === undefined ? reader.len : reader.pos + length;
    const message = createBaseTransactionsResponse();
    while (reader.pos < end) {
      const tag = reader.uint32();
      switch (tag >>> 3) {
        case 1:
          if (tag !== 10) {
            break;
          }

          message.batch!.push(TransactionOutput.decode(reader, reader.uint32()));
          continue;
      }
      if ((tag & 7) === 4 || tag === 0) {
        break;
      }
      reader.skipType(tag & 7);
    }
    return message;
  },

  // encodeTransform encodes a source of message objects.
  // Transform<TransactionsResponse, Uint8Array>
  async *encodeTransform(
    source:
      | AsyncIterable<TransactionsResponse | TransactionsResponse[]>
      | Iterable<TransactionsResponse | TransactionsResponse[]>,
  ): AsyncIterable<Uint8Array> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [TransactionsResponse.encode(p).finish()];
        }
      } else {
        yield* [TransactionsResponse.encode(pkt as any).finish()];
      }
    }
  },

  // decodeTransform decodes a source of encoded messages.
  // Transform<Uint8Array, TransactionsResponse>
  async *decodeTransform(
    source: AsyncIterable<Uint8Array | Uint8Array[]> | Iterable<Uint8Array | Uint8Array[]>,
  ): AsyncIterable<TransactionsResponse> {
    for await (const pkt of source) {
      if (globalThis.Array.isArray(pkt)) {
        for (const p of (pkt as any)) {
          yield* [TransactionsResponse.decode(p)];
        }
      } else {
        yield* [TransactionsResponse.decode(pkt as any)];
      }
    }
  },

  fromJSON(object: any): TransactionsResponse {
    return {
      batch: globalThis.Array.isArray(object?.batch) ? object.batch.map((e: any) => TransactionOutput.fromJSON(e)) : [],
    };
  },

  toJSON(message: TransactionsResponse): unknown {
    const obj: any = {};
    if (message.batch?.length) {
      obj.batch = message.batch.map((e) => TransactionOutput.toJSON(e));
    }
    return obj;
  },

  create(base?: DeepPartial<TransactionsResponse>): TransactionsResponse {
    return TransactionsResponse.fromPartial(base ?? {});
  },
  fromPartial(object: DeepPartial<TransactionsResponse>): TransactionsResponse {
    const message = createBaseTransactionsResponse();
    message.batch = object.batch?.map((e) => TransactionOutput.fromPartial(e)) || [];
    return message;
  },
};

export type TxnStreamService = typeof TxnStreamService;
export const TxnStreamService = {
  getChainId: {
    path: "/libra2.txnstream.v1.TxnStream/GetChainId",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    requestDeserialize: (value: Buffer) => Empty.decode(value),
    responseSerialize: (value: ChainIdResponse) => Buffer.from(ChainIdResponse.encode(value).finish()),
    responseDeserialize: (value: Buffer) => ChainIdResponse.decode(value),
  },
  getServerInfo: {
    path: "/libra2.txnstream.v1.TxnStream/GetServerInfo",
    requestStream: false,
    responseStream: false,
    requestSerialize: (value: Empty) => Buffer.from(Empty.encode(value).finish()),
    requestDeserialize: (value: Buffer) => Empty.decode(value),
    responseSerialize: (value: ServerInfoResponse) => Buffer.from(ServerInfoResponse.encode(value).finish()),
    responseDeserialize: (value: Buffer) => ServerInfoResponse.decode(value),
  },
  streamTransactions: {
    path: "/libra2.txnstream.v1.TxnStream/StreamTransactions",
    requestStream: false,
    responseStream: true,
    requestSerialize: (value: TransactionsRequest) => Buffer.from(TransactionsRequest.encode(value).finish()),
    requestDeserialize: (value: Buffer) => TransactionsRequest.decode(value),
    responseSerialize: (value: TransactionsResponse) => Buffer.from(TransactionsResponse.encode(value).finish()),
    responseDeserialize: (value: Buffer) => TransactionsResponse.decode(value),
  },
} as const;

export interface TxnStreamServer extends UntypedServiceImplementation {
  getChainId: handleUnaryCall<Empty, ChainIdResponse>;
  getServerInfo: handleUnaryCall<Empty, ServerInfoResponse>;
  streamTransactions: handleServerStreamingCall<TransactionsRequest, TransactionsResponse>;
}

export interface TxnStreamClient extends Client {
  getChainId(
    request: Empty,
    callback: (error: ServiceError | null, response: ChainIdResponse) => void,
  ): ClientUnaryCall;
  getChainId(
    request: Empty,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: ChainIdResponse) => void,
  ): ClientUnaryCall;
  getChainId(
    request: Empty,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: ChainIdResponse) => void,
  ): ClientUnaryCall;
  getServerInfo(
    request: Empty,
    callback: (error: ServiceError | null, response: ServerInfoResponse) => void,
  ): ClientUnaryCall;
  getServerInfo(
    request: Empty,
    metadata: Metadata,
    callback: (error: ServiceError | null, response: ServerInfoResponse) => void,
  ): ClientUnaryCall;
  getServerInfo(
    request: Empty,
    metadata: Metadata,
    options: Partial<CallOptions>,
    callback: (error: ServiceError | null, response: ServerInfoResponse) => void,
  ): ClientUnaryCall;
  streamTransactions(
    request: TransactionsRequest,
    options?: Partial<CallOptions>,
  ): ClientReadableStream<TransactionsResponse>;
  streamTransactions(
    request: TransactionsRequest,
    metadata?: Metadata,
    options?: Partial<CallOptions>,
  ): ClientReadableStream<TransactionsResponse>;
}

export const TxnStreamClient = makeGenericClientConstructor(
  TxnStreamService,
  "libra2.txnstream.v1.TxnStream",
) as unknown as {
  new (address: string, credentials: ChannelCredentials, options?: Partial<ClientOptions>): TxnStreamClient;
  service: typeof TxnStreamService;
  serviceName: string;
};

function bytesFromBase64(b64: string): Uint8Array {
  if ((globalThis as any).Buffer) {
    return Uint8Array.from(globalThis.Buffer.from(b64, "base64"));
  } else {
    const bin = globalThis.atob(b64);
    const arr = new Uint8Array(bin.length);
    for (let i = 0; i < bin.length; ++i) {
      arr[i] = bin.charCodeAt(i);
    }
    return arr;
  }
}

function base64FromBytes(arr: Uint8Array): string {
  if ((globalThis as any).Buffer) {
    return globalThis.Buffer.from(arr).toString("base64");
  } else {
    const bin: string[] = [];
    arr.forEach((byte) => {
      bin.push(globalThis.String.fromCharCode(byte));
    });
    return globalThis.btoa(bin.join(""));
  }
}

type Builtin = Date | Function | Uint8Array | string | number | boolean | bigint | undefined;

type DeepPartial<T> = T extends Builtin ? T
  : T extends globalThis.Array<infer U> ? globalThis.Array<DeepPartial<U>>
  : T extends ReadonlyArray<infer U> ? ReadonlyArray<DeepPartial<U>>
  : T extends {} ? { [K in keyof T]?: DeepPartial<T[K]> }
  : Partial<T>;

function longToBigint(long: Long) {
  return BigInt(long.toString());
}

if (_m0.util.Long !== Long) {
  _m0.util.Long = Long as any;
  _m0.configure();
}

function isSet(value: any): boolean {
  return value !== null && value !== undefined;
}
