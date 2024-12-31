// cSpell:disable

#[cfg(test)]
mod tests {
    use crate::candid::parse_service_candid;

    #[allow(unused)]
    fn print_candid(filename: &str, candid: &str) {
        // 输出到对应的文件
        use std::io::Write;
        std::fs::remove_file(filename);
        std::fs::File::create(filename)
            .expect("create failed")
            .write_all(candid.as_bytes())
            .expect("write candid failed");
    }

    fn test_single_candid(candid: &str, name: &str) {
        let wrapped = parse_service_candid(candid).unwrap();

        // println!("wrapped1: {:#?}", wrapped1);

        print_candid(
            &format!("./tmp/{}_debug.tmp", name),
            &format!("{:#?}", wrapped),
        );
        print_candid(
            &format!("./tmp/{}_restore.tmp", name),
            &format!("{}", wrapped.to_text()),
        );

        println!("\n ======= {} done =======\n", name);
    }

    #[test]
    fn test_parse_candid() {
        std::fs::create_dir_all("./tmp").unwrap();

        let candid1 = r##"type CanisterInitialArg = record {
      permission_host : opt principal;
      record_collector : opt principal;
      schedule : opt nat64;
    };
    type CanisterStatusResponse = record {
      status : CanisterStatusType;
      memory_size : nat;
      cycles : nat;
      settings : DefiniteCanisterSettings;
      idle_cycles_burned_per_day : nat;
      module_hash : opt vec nat8;
    };
    type CanisterStatusType = variant { stopped; stopping; running };
    type DefiniteCanisterSettings = record {
      freezing_threshold : nat;
      controllers : vec principal;
      memory_allocation : nat;
      compute_allocation : nat;
    };
    type MaintainingReason = record { created : nat64; message : text };
    type MigratedRecords = record {
      records : vec Record;
      topics : vec text;
      updated : vec record { nat64; nat64; text };
      next_id : nat64;
    };
    type Page = record { page : nat32; size : nat32 };
    type PageData = record {
      total : nat32;
      data : vec Record;
      page : nat32;
      size : nat32;
    };
    type Permission = variant { Permitted : text; Forbidden : text };
    type PermissionReplacedArg = record {
      permissions : vec text;
      user_permissions : vec record { principal; vec text };
      role_permissions : vec record { text; vec text };
      user_roles : vec record { principal; vec text };
    };
    type PermissionUpdatedArg = variant {
      UpdateRolePermission : record { text; opt vec text };
      UpdateUserPermission : record { principal; opt vec text };
      UpdateUserRole : record { principal; opt vec text };
    };
    type Record = record {
      id : nat64;
      result : text;
      created : nat64;
      topic : text;
      content : text;
      done : nat64;
      level : RecordLevel;
      caller : principal;
    };
    type RecordLevel = variant { Error; Info; Warn; Debug; Trace };
    type RecordSearch = record {
      id : opt record { opt nat64; opt nat64 };
      created : opt record { opt nat64; opt nat64 };
      topic : opt vec text;
      content : opt text;
      level : opt vec RecordLevel;
      caller : opt vec principal;
    };
    type TestType = record { child : opt TestType };
    type WalletReceiveResult = record { accepted : nat64 };
    service : (opt CanisterInitialArg) -> {
      __get_candid_interface_tmp_hack : () -> (text) query;
      business_test : (TestType) -> () query;
      business_test_template_query : () -> (text);
      business_test_template_set : (text) -> ();
      canister_status : () -> (CanisterStatusResponse);
      maintaining_query : () -> (bool) query;
      maintaining_query_reason : () -> (opt MaintainingReason) query;
      maintaining_replace : (opt text) -> ();
      permission_all : () -> (vec Permission) query;
      permission_assigned_by_user : (principal) -> (opt vec Permission) query;
      permission_assigned_query : () -> (opt vec Permission) query;
      permission_find_by_user : (principal) -> (vec text) query;
      permission_host_find : () -> (opt principal) query;
      permission_host_replace : (opt principal) -> ();
      permission_query : () -> (vec text) query;
      permission_replace : (PermissionReplacedArg) -> ();
      permission_roles_all : () -> (vec record { text; vec Permission }) query;
      permission_roles_by_user : (principal) -> (opt vec text) query;
      permission_roles_query : () -> (opt vec text) query;
      permission_update : (vec PermissionUpdatedArg) -> ();
      record_collector_find : () -> (opt principal) query;
      record_collector_update : (opt principal) -> ();
      record_find_all : (opt RecordSearch) -> (vec Record) query;
      record_find_by_page : (opt RecordSearch, Page) -> (PageData) query;
      record_migrate : (nat32) -> (MigratedRecords);
      record_topics : () -> (vec text) query;
      schedule_find : () -> (opt nat64) query;
      schedule_replace : (opt nat64) -> ();
      schedule_trigger : () -> ();
      version : () -> (nat32) query;
      wallet_balance : () -> (nat) query;
      wallet_receive : () -> (WalletReceiveResult);
      whoami : () -> (principal) query;
    }"##;

        test_single_candid(candid1, "candid1");

        let candid2 = r##"type CanisterStatusResponse = record {
      status : CanisterStatusType;
      memory_size : nat;
      cycles : nat;
      settings : DefiniteCanisterSettings;
      idle_cycles_burned_per_day : nat;
      module_hash : opt vec nat8;
    };
    type CanisterStatusType = variant { stopped; stopping; running };
    type CustomHttpRequest = record {
      url : text;
      method : text;
      body : vec nat8;
      headers : vec record { text; text };
    };
    type CustomHttpResponse = record {
      body : vec nat8;
      headers : vec record { text; text };
      status_code : nat16;
    };
    type DefiniteCanisterSettings = record {
      freezing_threshold : nat;
      controllers : vec principal;
      memory_allocation : nat;
      compute_allocation : nat;
    };
    type ExtAllowanceArgs = record {
      token : text;
      owner : ExtUser;
      spender : principal;
    };
    type ExtApproveArgs = record {
      token : text;
      subaccount : opt vec nat8;
      allowance : nat;
      spender : principal;
    };
    type ExtBalanceArgs = record { token : text; user : ExtUser };
    type ExtBatchError = variant { Error : text };
    type ExtCommonError = variant { InvalidToken : text; Other : text };
    type ExtListing = record {
      locked : opt int;
      seller : principal;
      price : nat64;
    };
    type ExtMintArgs = record { to : ExtUser; metadata : opt vec nat8 };
    type ExtTokenMetadata = variant {
      fungible : record {
        decimals : nat8;
        metadata : opt vec nat8;
        name : text;
        symbol : text;
      };
      nonfungible : record { metadata : opt vec nat8 };
    };
    type ExtTransferArgs = record {
      to : ExtUser;
      token : text;
      notify : bool;
      from : ExtUser;
      memo : vec nat8;
      subaccount : opt vec nat8;
      amount : nat;
    };
    type ExtTransferError = variant {
      CannotNotify : text;
      InsufficientBalance;
      InvalidToken : text;
      Rejected;
      Unauthorized : text;
      Other : text;
    };
    type ExtUser = variant { "principal" : principal; address : text };
    type InnerData = record {
      data : vec nat8;
      headers : vec record { text; text };
    };
    type LimitDuration = record { end : nat64; start : nat64 };
    type MediaData = variant { Inner : InnerData; Outer : OuterData };
    type MotokoResult = variant { ok : nat; err : ExtCommonError };
    type MotokoResult_1 = variant { ok : vec MotokoResult; err : ExtBatchError };
    type MotokoResult_2 = variant { ok : nat; err : ExtTransferError };
    type MotokoResult_3 = variant { ok : text; err : ExtCommonError };
    type MotokoResult_4 = variant { ok : ExtTokenMetadata; err : ExtCommonError };
    type MotokoResult_5 = variant { ok : vec nat32; err : ExtCommonError };
    type MotokoResult_6 = variant {
      ok : vec record { nat32; opt ExtListing; opt vec nat8 };
      err : ExtCommonError;
    };
    type MotokoResult_7 = variant { ok : vec MotokoResult_2; err : ExtBatchError };
    type NFTOwnable = variant {
      Data : vec nat8;
      List : vec NFTOwnable;
      None;
      Text : text;
      Media : MediaData;
    };
    type NftTicketStatus = variant {
      Anonymous : record { nat64; NFTOwnable };
      NoBody : nat64;
      InvalidToken;
      Owner : record { nat64; NFTOwnable };
      Forbidden : nat64;
    };
    type NftView = record { owner : text; name : text; approved : opt text };
    type OuterData = record { url : text; headers : vec record { text; text } };
    type WalletReceiveResult = record { accepted : nat64 };
    service : {
      __get_candid_interface_tmp_hack : () -> (text) query;
      allowance : (ExtAllowanceArgs) -> (MotokoResult) query;
      approve : (ExtApproveArgs) -> (bool);
      approveAll : (vec ExtApproveArgs) -> (vec nat32);
      balance : (ExtBalanceArgs) -> (MotokoResult) query;
      balance_batch : (vec ExtBalanceArgs) -> (MotokoResult_1) query;
      batchTransfer : (vec ExtTransferArgs) -> (vec MotokoResult_2);
      bearer : (text) -> (MotokoResult_3) query;
      calcTokenIdentifier : (nat32) -> (text) query;
      canister_status : () -> (CanisterStatusResponse);
      extensions : () -> (vec text) query;
      getAllowances : () -> (vec record { nat32; principal }) query;
      getMetadata : () -> (vec record { nat32; ExtTokenMetadata }) query;
      getMinter : () -> (principal) query;
      getProperties : () -> (vec record { text; vec record { text; nat } }) query;
      getRegistry : () -> (vec record { nat32; text }) query;
      getScore : () -> (vec record { nat32; float64 }) query;
      getTokens : () -> (vec record { nat32; ExtTokenMetadata }) query;
      getTokensByIds : (vec nat32) -> (
          vec record { nat32; ExtTokenMetadata },
        ) query;
      http_request : (CustomHttpRequest) -> (CustomHttpResponse) query;
      maintainable_is_maintaining : () -> (bool) query;
      maintainable_set_maintaining : (bool) -> ();
      metadata : (text) -> (MotokoResult_4) query;
      mintNFT : (ExtMintArgs) -> ();
      nft_get_all_tokens : () -> (vec NftView) query;
      nft_get_metadata : (text, nat32) -> (opt MediaData) query;
      nft_get_rarity : (text) -> (text) query;
      nft_info_get_name : () -> (text) query;
      nft_info_get_symbol : () -> (text) query;
      nft_info_set_logo : (opt MediaData) -> ();
      nft_info_set_maintaining : (opt MediaData) -> ();
      nft_info_set_name : (text) -> ();
      nft_info_set_symbol : (text) -> ();
      nft_info_set_thumbnail : (opt MediaData) -> ();
      nft_limit_minter_get : () -> (vec LimitDuration) query;
      nft_limit_minter_set : (vec LimitDuration) -> ();
      nft_mint_batch : (ExtMintArgs, opt principal, nat32, nat32) -> ();
      nft_set_content : (vec record { text; opt vec nat8 }) -> ();
      nft_set_content_by_token_index : (vec record { nat32; opt vec nat8 }) -> ();
      nft_set_content_by_url_and_thumbnail : (text, text) -> ();
      nft_set_metadata : (vec record { text; nat32; opt MediaData }) -> ();
      nft_set_metadata_all : (vec record { nat32; opt MediaData }) -> ();
      nft_set_metadata_by_token_index : (
          vec record { nat32; nat32; opt MediaData },
        ) -> ();
      nft_set_ownable : (text, NFTOwnable) -> ();
      nft_set_ownable_all : (NFTOwnable) -> ();
      nft_set_ownable_by_token_index : (nat32, NFTOwnable) -> ();
      nft_set_ownable_by_token_index_batch : (
          vec record { nat32; NFTOwnable },
        ) -> ();
      nft_set_rarity : (vec record { text; text }) -> ();
      nft_set_rarity_all : (text) -> ();
      nft_set_rarity_by_token_index : (vec record { nat32; text }) -> ();
      nft_set_thumbnail : (text, opt MediaData) -> ();
      nft_set_thumbnail_all : (opt MediaData) -> ();
      nft_set_thumbnail_by_token_index : (nat32, opt MediaData) -> ();
      nft_set_yumi_traits : (vec record { text; text }) -> (bool);
      nft_ticket : (text) -> (NftTicketStatus) query;
      nft_ticket_get_activity : () -> (nat64, nat64) query;
      nft_ticket_get_transfer_forbidden : () -> (vec LimitDuration) query;
      nft_ticket_set_activity : (nat64, nat64) -> ();
      nft_ticket_set_transfer_forbidden : (vec LimitDuration) -> ();
      permission_get_admins : () -> (vec principal) query;
      permission_get_minters : () -> (vec principal) query;
      permission_is_admin : (principal) -> (bool) query;
      permission_is_minter : (principal) -> (bool) query;
      permission_remove_admin : (principal) -> ();
      permission_remove_minter : (principal) -> ();
      permission_set_admin : (principal) -> ();
      permission_set_minter : (principal) -> ();
      supply : (text) -> (MotokoResult) query;
      toAddress : (text, nat) -> (text) query;
      tokens : (text) -> (MotokoResult_5) query;
      tokens_ext : (text) -> (MotokoResult_6) query;
      transfer : (ExtTransferArgs) -> (MotokoResult_2);
      transfer_batch : (vec ExtTransferArgs) -> (MotokoResult_7);
      upload_data_by_slice : (vec nat8) -> ();
      upload_data_by_slice_query : (nat32, nat32) -> (vec nat8) query;
      wallet_balance : () -> (nat) query;
      wallet_receive : () -> (WalletReceiveResult);
      whoami : () -> (principal) query;
    }"##;

        test_single_candid(candid2, "candid2");

        let candid3 = r##"type definite_canister_settings =
    record {
      compute_allocation: nat;
      controllers: opt vec principal;
      freezing_threshold: nat;
      memory_allocation: nat;
    };
   type canister_status =
    record {
      cycles: nat;
      memory_size: nat;
      module_hash: opt vec nat8;
      settings: definite_canister_settings;
      status: variant {
                running;
                stopped;
                stopping;
              };
    };
   type canister_id = principal;
   type WithdrawResponse =
    record {
      index: nat;
      timestamp: int;
      token_id: text;
      txn_type:
       variant {
         auction_bid:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            sale_id: text;
            token: TokenSpec;
          };
         burn;
         canister_managers_updated:
          record {
            extensible: CandyValue;
            managers: vec principal;
          };
         canister_network_updated:
          record {
            extensible: CandyValue;
            network: principal;
          };
         canister_owner_updated:
          record {
            extensible: CandyValue;
            owner: principal;
          };
         data;
         deposit_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            token: TokenSpec;
            trx_id: TransactionID;
          };
         escrow_deposit:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
         escrow_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
         extensible: CandyValue;
         mint:
          record {
            extensible: CandyValue;
            from: Account__1;
            sale: opt record {
                        amount: nat;
                        token: TokenSpec;
                      };
            to: Account__1;
          };
         owner_transfer:
          record {
            extensible: CandyValue;
            from: Account__1;
            to: Account__1;
          };
         royalty_paid:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            reciever: Account__1;
            sale_id: opt text;
            seller: Account__1;
            tag: text;
            token: TokenSpec;
          };
         sale_ended:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            sale_id: opt text;
            seller: Account__1;
            token: TokenSpec;
          };
         sale_opened:
          record {
            extensible: CandyValue;
            pricing: PricingConfig;
            sale_id: text;
          };
         sale_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
       };
    };
   type WithdrawRequest =
    variant {
      deposit: DepositWithdrawDescription;
      escrow: WithdrawDescription;
      reject: RejectDescription;
      sale: WithdrawDescription;
    };
   type WithdrawDescription =
    record {
      amount: nat;
      buyer: Account;
      seller: Account;
      token: TokenSpec__1;
      token_id: text;
      withdraw_to: Account;
    };
   type User =
    variant {
      address: AccountIdentifier;
      "principal": principal;
    };
   type UpdateRequest =
    record {
      id: text;
      update: vec Update;
    };
   type UpdateMode =
    variant {
      Lock: CandyValue;
      Next: vec Update;
      Set: CandyValue;
    };
   type UpdateCallsAggregatedData = vec nat64;
   type Update =
    record {
      mode: UpdateMode;
      name: text;
    };
   type TransferResponse =
    variant {
      err:
       variant {
         CannotNotify: AccountIdentifier;
         InsufficientBalance;
         InvalidToken: TokenIdentifier;
         Other: text;
         Rejected;
         Unauthorized: AccountIdentifier;
       };
      ok: Balance;
    };
   type TransferRequest =
    record {
      amount: Balance;
      from: User;
      memo: Memo;
      notify: bool;
      subaccount: opt SubAccount;
      to: User;
      token: TokenIdentifier;
    };
   type TransactionRecord =
    record {
      index: nat;
      timestamp: int;
      token_id: text;
      txn_type:
       variant {
         auction_bid:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            sale_id: text;
            token: TokenSpec;
          };
         burn;
         canister_managers_updated:
          record {
            extensible: CandyValue;
            managers: vec principal;
          };
         canister_network_updated:
          record {
            extensible: CandyValue;
            network: principal;
          };
         canister_owner_updated:
          record {
            extensible: CandyValue;
            owner: principal;
          };
         data;
         deposit_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            token: TokenSpec;
            trx_id: TransactionID;
          };
         escrow_deposit:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
         escrow_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
         extensible: CandyValue;
         mint:
          record {
            extensible: CandyValue;
            from: Account__1;
            sale: opt record {
                        amount: nat;
                        token: TokenSpec;
                      };
            to: Account__1;
          };
         owner_transfer:
          record {
            extensible: CandyValue;
            from: Account__1;
            to: Account__1;
          };
         royalty_paid:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            reciever: Account__1;
            sale_id: opt text;
            seller: Account__1;
            tag: text;
            token: TokenSpec;
          };
         sale_ended:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            sale_id: opt text;
            seller: Account__1;
            token: TokenSpec;
          };
         sale_opened:
          record {
            extensible: CandyValue;
            pricing: PricingConfig;
            sale_id: text;
          };
         sale_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
       };
    };
   type TransactionID__1 =
    variant {
      extensible: CandyValue;
      "nat": nat;
      "text": text;
    };
   type TransactionID =
    variant {
      extensible: CandyValue;
      "nat": nat;
      "text": text;
    };
   type TokenSpec__1 =
    variant {
      extensible: CandyValue;
      ic: ICTokenSpec;
    };
   type TokenSpec =
    variant {
      extensible: CandyValue;
      ic: ICTokenSpec;
    };
   type TokenIdentifier = text;
   type SubAccountInfo =
    record {
      account: record {
                 "principal": principal;
                 sub_account: blob;
               };
      account_id: blob;
      account_id_text: text;
      "principal": principal;
    };
   type SubAccount = vec nat8;
   type StreamingStrategy = variant {
                              Callback:
                               record {
                                 callback: func () -> ();
                                 token: StreamingCallbackToken;
                               };};
   type StreamingCallbackToken =
    record {
      content_encoding: text;
      index: nat;
      key: text;
    };
   type StreamingCallbackResponse =
    record {
      body: blob;
      token: opt StreamingCallbackToken;
    };
   type StorageMetrics =
    record {
      allocated_storage: nat;
      allocations: vec AllocationRecordStable;
      available_space: nat;
    };
   type StateSize =
    record {
      allocations: nat;
      buckets: nat;
      escrow_balances: nat;
      nft_ledgers: nat;
      nft_sales: nat;
      offers: nat;
      sales_balances: nat;
    };
   type StakeRecord =
    record {
      amount: nat;
      staker: Account;
      token_id: text;
    };
   type StageLibraryResponse = record {canister: principal;};
   type StageChunkArg =
    record {
      chunk: nat;
      content: blob;
      filedata: CandyValue;
      library_id: text;
      token_id: text;
    };
   type StableSalesBalances =
    vec record {
          Account;
          Account;
          text;
          EscrowRecord;
        };
   type StableOffers =
    vec record {
          Account;
          Account;
          int;
        };
   type StableNftLedger =
    vec record {
          text;
          TransactionRecord;
        };
   type StableEscrowBalances =
    vec record {
          Account;
          Account;
          text;
          EscrowRecord;
        };
   type StableCollectionData =
    record {
      active_bucket: opt principal;
      allocated_storage: nat;
      available_space: nat;
      logo: opt text;
      managers: vec principal;
      metadata: opt CandyValue;
      name: opt text;
      network: opt principal;
      owner: principal;
      symbol: opt text;
    };
   type StableBucketData =
    record {
      allocated_space: nat;
      allocations: vec record {
                         record {
                           text;
                           text;
                         };
                         int;
                       };
      available_space: nat;
      b_gateway: bool;
      date_added: int;
      "principal": principal;
      version: record {
                 nat;
                 nat;
                 nat;
               };
    };
   type ShareWalletRequest =
    record {
      from: Account;
      to: Account;
      token_id: text;
    };
   type SalesConfig =
    record {
      broker_id: opt principal;
      escrow_receipt: opt EscrowReceipt;
      pricing: PricingConfig__1;
    };
   type SaleStatusStable =
    record {
      broker_id: opt principal;
      original_broker_id: opt principal;
      sale_id: text;
      sale_type: variant {auction: AuctionStateStable;};
      token_id: text;
    };
   type SaleInfoResponse =
    variant {
      active:
       record {
         count: nat;
         eof: bool;
         records: vec record {
                        text;
                        opt SaleStatusStable;
                      };
       };
      deposit_info: SubAccountInfo;
      history: record {
                 count: nat;
                 eof: bool;
                 records: vec opt SaleStatusStable;
               };
      status: opt SaleStatusStable;
    };
   type SaleInfoRequest =
    variant {
      active: opt record {
                    nat;
                    nat;
                  };
      deposit_info: opt Account;
      history: opt record {
                     nat;
                     nat;
                   };
      status: text;
    };
   type Result__1 =
    variant {
      Err: NftError;
      Ok: nat;
    };
   type Result_9 =
    variant {
      err: CommonError;
      ok: Metadata;
    };
   type Result_8 =
    variant {
      err: OrigynError;
      ok: NFTInfoStable;
    };
   type Result_7 =
    variant {
      err: OrigynError;
      ok: SaleInfoResponse;
    };
   type Result_6 =
    variant {
      err: OrigynError;
      ok: ManageSaleResponse;
    };
   type Result_5 =
    variant {
      err: OrigynError;
      ok: OwnerTransferResponse;
    };
   type Result_4 =
    variant {
      err: OrigynError;
      ok: StageLibraryResponse;
    };
   type Result_3 =
    variant {
      err: OrigynError;
      ok: text;
    };
   type Result_2 =
    variant {
      err: OrigynError;
      ok: StorageMetrics;
    };
   type Result_19 =
    variant {
      err: OrigynError;
      ok: BalanceResponse;
    };
   type Result_18 =
    variant {
      err: CommonError;
      ok: AccountIdentifier;
    };
   type Result_17 =
    variant {
      err: OrigynError;
      ok: Account;
    };
   type Result_16 =
    variant {
      err: OrigynError;
      ok: ChunkContent;
    };
   type Result_15 =
    variant {
      err: OrigynError;
      ok: CollectionInfo;
    };
   type Result_14 =
    variant {
      err: OrigynError;
      ok: bool;
    };
   type Result_13 =
    variant {
      err: OrigynError;
      ok: GovernanceResponse;
    };
   type Result_12 =
    variant {
      err: OrigynError;
      ok: vec TransactionRecord;
    };
   type Result_11 =
    variant {
      err: OrigynError;
      ok: ManageStorageResponse;
    };
   type Result_10 =
    variant {
      err: OrigynError;
      ok: MarketTransferRequestReponse;
    };
   type Result_1 =
    variant {
      err: CommonError;
      ok: vec EXTTokensResult;
    };
   type Result =
    variant {
      err: OrigynError;
      ok: NFTUpdateResponse;
    };
   type RejectDescription =
    record {
      buyer: Account;
      seller: Account;
      token: TokenSpec__1;
      token_id: text;
    };
   type Property =
    record {
      immutable: bool;
      name: text;
      value: CandyValue;
    };
   type Principal = principal;
   type PricingConfig__1 =
    variant {
      auction: AuctionConfig;
      dutch: record {
               decay_per_hour: float64;
               reserve: opt nat;
               start_price: nat;
             };
      extensible: variant {candyClass;};
      flat: record {
              amount: nat;
              token: TokenSpec;
            };
      instant;
    };
   type PricingConfig =
    variant {
      auction: AuctionConfig;
      dutch: record {
               decay_per_hour: float64;
               reserve: opt nat;
               start_price: nat;
             };
      extensible: variant {candyClass;};
      flat: record {
              amount: nat;
              token: TokenSpec;
            };
      instant;
    };
   type OwnerTransferResponse =
    record {
      assets: vec CandyValue;
      transaction: TransactionRecord;
    };
   type OwnerOfResponse =
    variant {
      Err: NftError;
      Ok: opt principal;
    };
   type OrigynError =
    record {
      error: Errors;
      flag_point: text;
      number: nat32;
      "text": text;
    };
   type NumericEntity =
    record {
      avg: nat64;
      first: nat64;
      last: nat64;
      max: nat64;
      min: nat64;
    };
   type NftError =
    variant {
      ExistedNFT;
      OperatorNotFound;
      Other: text;
      OwnerNotFound;
      SelfApprove;
      SelfTransfer;
      TokenNotFound;
      TxNotFound;
      UnauthorizedOperator;
      UnauthorizedOwner;
    };
   type Nanos = nat64;
   type NFTUpdateResponse = bool;
   type NFTUpdateRequest =
    variant {
      replace: record {
                 data: CandyValue;
                 token_id: text;
               };
      update: record {
                app_id: text;
                token_id: text;
                update: UpdateRequest;
              };
    };
   type NFTInfoStable =
    record {
      current_sale: opt SaleStatusStable;
      metadata: CandyValue;
    };
   type NFTBackupChunk =
    record {
      allocations: vec record {
                         record {
                           text;
                           text;
                         };
                         AllocationRecordStable;
                       };
      buckets: vec record {
                     principal;
                     StableBucketData;
                   };
      canister: principal;
      collection_data: StableCollectionData;
      escrow_balances: StableEscrowBalances;
      nft_ledgers: StableNftLedger;
      nft_sales: vec record {
                       text;
                       SaleStatusStable;
                     };
      offers: StableOffers;
      sales_balances: StableSalesBalances;
    };
   type MetricsGranularity =
    variant {
      daily;
      hourly;
    };
   type Metadata =
    variant {
      fungible:
       record {
         decimals: nat8;
         metadata: opt blob;
         name: text;
         symbol: text;
       };
      nonfungible: record {metadata: opt blob;};
    };
   type Memo = blob;
   type MarketTransferRequestReponse =
    record {
      index: nat;
      timestamp: int;
      token_id: text;
      txn_type:
       variant {
         auction_bid:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            sale_id: text;
            token: TokenSpec;
          };
         burn;
         canister_managers_updated:
          record {
            extensible: CandyValue;
            managers: vec principal;
          };
         canister_network_updated:
          record {
            extensible: CandyValue;
            network: principal;
          };
         canister_owner_updated:
          record {
            extensible: CandyValue;
            owner: principal;
          };
         data;
         deposit_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            token: TokenSpec;
            trx_id: TransactionID;
          };
         escrow_deposit:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
         escrow_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
         extensible: CandyValue;
         mint:
          record {
            extensible: CandyValue;
            from: Account__1;
            sale: opt record {
                        amount: nat;
                        token: TokenSpec;
                      };
            to: Account__1;
          };
         owner_transfer:
          record {
            extensible: CandyValue;
            from: Account__1;
            to: Account__1;
          };
         royalty_paid:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            reciever: Account__1;
            sale_id: opt text;
            seller: Account__1;
            tag: text;
            token: TokenSpec;
          };
         sale_ended:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            sale_id: opt text;
            seller: Account__1;
            token: TokenSpec;
          };
         sale_opened:
          record {
            extensible: CandyValue;
            pricing: PricingConfig;
            sale_id: text;
          };
         sale_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
       };
    };
   type MarketTransferRequest =
    record {
      sales_config: SalesConfig;
      token_id: text;
    };
   type ManageStorageResponse = variant {
                                  add_storage_canisters: record {
                                                           nat;
                                                           nat;
                                                         };};
   type ManageStorageRequest = variant {
                                 add_storage_canisters:
                                  vec
                                   record {
                                     principal;
                                     nat;
                                     record {
                                       nat;
                                       nat;
                                       nat;
                                     };
                                   };};
   type ManageSaleResponse =
    variant {
      bid: BidResponse;
      distribute_sale: DistributeSaleResponse;
      end_sale: EndSaleResponse;
      escrow_deposit: EscrowResponse;
      open_sale: bool;
      refresh_offers: vec EscrowRecord;
      withdraw: WithdrawResponse;
    };
   type ManageSaleRequest =
    variant {
      bid: BidRequest;
      distribute_sale: DistributeSaleRequest;
      end_sale: text;
      escrow_deposit: EscrowRequest;
      open_sale: text;
      refresh_offers: opt Account;
      withdraw: WithdrawRequest;
    };
   type ManageCollectionCommand =
    variant {
      UpdateLogo: opt text;
      UpdateManagers: vec principal;
      UpdateMetadata: record {
                        text;
                        opt CandyValue;
                        bool;
                      };
      UpdateName: opt text;
      UpdateNetwork: opt principal;
      UpdateOwner: principal;
      UpdateSymbol: opt text;
    };
   type LogMessagesData =
    record {
      caller: Caller;
      data: Data;
      message: text;
      timeNanos: Nanos;
    };
   type LogEntry =
    record {
      caller: opt principal;
      data: CandyValue;
      event: text;
      timestamp: int;
    };
   type InitArgs =
    record {
      owner: Principal;
      storage_space: opt nat;
    };
   type ICTokenSpec =
    record {
      canister: principal;
      decimals: nat;
      fee: nat;
      standard: variant {
                  DIP20;
                  EXTFungible;
                  ICRC1;
                  Ledger;
                };
      symbol: text;
    };
   type HttpRequest =
    record {
      body: blob;
      headers: vec HeaderField;
      method: text;
      url: text;
    };
   type HourlyMetricsData =
    record {
      canisterCycles: CanisterCyclesAggregatedData;
      canisterHeapMemorySize: CanisterHeapMemoryAggregatedData;
      canisterMemorySize: CanisterMemoryAggregatedData;
      timeMillis: int;
      updateCalls: UpdateCallsAggregatedData;
    };
   type HeaderField__1 =
    record {
      text;
      text;
    };
   type HeaderField =
    record {
      text;
      text;
    };
   type HTTPResponse =
    record {
      body: blob;
      headers: vec HeaderField__1;
      status_code: nat16;
      streaming_strategy: opt StreamingStrategy;
    };
   type GovernanceResponse = variant {clear_shared_wallets: bool;};
   type GovernanceRequest = variant {clear_shared_wallets: text;};
   type GetMetricsParameters =
    record {
      dateFromMillis: nat;
      dateToMillis: nat;
      granularity: MetricsGranularity;
    };
   type GetLogMessagesParameters =
    record {
      count: nat32;
      filter: opt GetLogMessagesFilter;
      fromTimeNanos: opt Nanos;
    };
   type GetLogMessagesFilter =
    record {
      analyzeCount: nat32;
      messageContains: opt text;
      messageRegex: opt text;
    };
   type GetLatestLogMessagesParameters =
    record {
      count: nat32;
      filter: opt GetLogMessagesFilter;
      upToTimeNanos: opt Nanos;
    };
   type EscrowResponse =
    record {
      balance: nat;
      receipt: EscrowReceipt;
      transaction: TransactionRecord;
    };
   type EscrowRequest =
    record {
      deposit: DepositDetail;
      lock_to_date: opt int;
      token_id: text;
    };
   type EscrowRecord =
    record {
      account_hash: opt blob;
      amount: nat;
      buyer: Account__1;
      lock_to_date: opt int;
      sale_id: opt text;
      seller: Account__1;
      token: TokenSpec;
      token_id: text;
    };
   type EscrowReceipt =
    record {
      amount: nat;
      buyer: Account__1;
      seller: Account__1;
      token: TokenSpec;
      token_id: text;
    };
   type Errors =
    variant {
      app_id_not_found;
      asset_mismatch;
      attempt_to_stage_system_data;
      auction_ended;
      auction_not_started;
      bid_too_low;
      cannot_find_status_in_metadata;
      cannot_restage_minted_token;
      content_not_deserializable;
      content_not_found;
      deposit_burned;
      escrow_cannot_be_removed;
      escrow_owner_not_the_owner;
      escrow_withdraw_payment_failed;
      existing_sale_found;
      id_not_found_in_metadata;
      improper_interface;
      item_already_minted;
      item_not_owned;
      library_not_found;
      malformed_metadata;
      no_escrow_found;
      not_enough_storage;
      nyi;
      out_of_range;
      owner_not_found;
      property_not_found;
      receipt_data_mismatch;
      sale_id_does_not_match;
      sale_not_found;
      sale_not_over;
      sales_withdraw_payment_failed;
      storage_configuration_error;
      token_id_mismatch;
      token_non_transferable;
      token_not_found;
      unauthorized_access;
      unreachable;
      update_class_error;
      validate_deposit_failed;
      validate_deposit_wrong_amount;
      validate_deposit_wrong_buyer;
      validate_trx_wrong_host;
      withdraw_too_large;
    };
   type EndSaleResponse =
    record {
      index: nat;
      timestamp: int;
      token_id: text;
      txn_type:
       variant {
         auction_bid:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            sale_id: text;
            token: TokenSpec;
          };
         burn;
         canister_managers_updated:
          record {
            extensible: CandyValue;
            managers: vec principal;
          };
         canister_network_updated:
          record {
            extensible: CandyValue;
            network: principal;
          };
         canister_owner_updated:
          record {
            extensible: CandyValue;
            owner: principal;
          };
         data;
         deposit_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            token: TokenSpec;
            trx_id: TransactionID;
          };
         escrow_deposit:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
         escrow_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
         extensible: CandyValue;
         mint:
          record {
            extensible: CandyValue;
            from: Account__1;
            sale: opt record {
                        amount: nat;
                        token: TokenSpec;
                      };
            to: Account__1;
          };
         owner_transfer:
          record {
            extensible: CandyValue;
            from: Account__1;
            to: Account__1;
          };
         royalty_paid:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            reciever: Account__1;
            sale_id: opt text;
            seller: Account__1;
            tag: text;
            token: TokenSpec;
          };
         sale_ended:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            sale_id: opt text;
            seller: Account__1;
            token: TokenSpec;
          };
         sale_opened:
          record {
            extensible: CandyValue;
            pricing: PricingConfig;
            sale_id: text;
          };
         sale_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
       };
    };
   type EXTTokensResult =
    record {
      nat32;
      opt record {
            locked: opt int;
            price: nat64;
            seller: principal;
          };
      opt vec nat8;
    };
   type DistributeSaleResponse = vec Result_6;
   type DistributeSaleRequest = record {seller: opt Account;};
   type DepositWithdrawDescription =
    record {
      amount: nat;
      buyer: Account;
      token: TokenSpec__1;
      withdraw_to: Account;
    };
   type DepositDetail =
    record {
      amount: nat;
      buyer: Account;
      sale_id: opt text;
      seller: Account;
      token: TokenSpec__1;
      trx_id: opt TransactionID__1;
    };
   type Data =
    variant {
      Array: variant {
               frozen: vec CandyValue;
               thawed: vec CandyValue;
             };
      Blob: blob;
      Bool: bool;
      Bytes: variant {
               frozen: vec nat8;
               thawed: vec nat8;
             };
      Class: vec Property;
      Empty;
      Float: float64;
      Floats: variant {
                frozen: vec float64;
                thawed: vec float64;
              };
      Int: int;
      Int16: int16;
      Int32: int32;
      Int64: int64;
      Int8: int8;
      Nat: nat;
      Nat16: nat16;
      Nat32: nat32;
      Nat64: nat64;
      Nat8: nat8;
      Nats: variant {
              frozen: vec nat;
              thawed: vec nat;
            };
      Option: opt CandyValue;
      Principal: principal;
      Text: text;
    };
   type DailyMetricsData =
    record {
      canisterCycles: NumericEntity;
      canisterHeapMemorySize: NumericEntity;
      canisterMemorySize: NumericEntity;
      timeMillis: int;
      updateCalls: nat64;
    };
   type CommonError =
    variant {
      InvalidToken: TokenIdentifier;
      Other: text;
    };
   type CollectionInfo =
    record {
      allocated_storage: opt nat;
      available_space: opt nat;
      fields: opt vec record {
                        text;
                        opt nat;
                        opt nat;
                      };
      logo: opt text;
      managers: opt vec principal;
      metadata: opt CandyValue;
      multi_canister: opt vec principal;
      multi_canister_count: opt nat;
      name: opt text;
      network: opt principal;
      owner: opt principal;
      symbol: opt text;
      token_ids: opt vec text;
      token_ids_count: opt nat;
      total_supply: opt nat;
    };
   type ChunkRequest =
    record {
      chunk: opt nat;
      library_id: text;
      token_id: text;
    };
   type ChunkContent =
    variant {
      chunk:
       record {
         content: blob;
         current_chunk: opt nat;
         storage_allocation: AllocationRecordStable;
         total_chunks: nat;
       };
      remote: record {
                args: ChunkRequest;
                canister: principal;
              };
    };
   type CanisterMetricsData =
    variant {
      daily: vec DailyMetricsData;
      hourly: vec HourlyMetricsData;
    };
   type CanisterMetrics = record {data: CanisterMetricsData;};
   type CanisterMemoryAggregatedData = vec nat64;
   type CanisterLogResponse =
    variant {
      messages: CanisterLogMessages;
      messagesInfo: CanisterLogMessagesInfo;
    };
   type CanisterLogRequest =
    variant {
      getLatestMessages: GetLatestLogMessagesParameters;
      getMessages: GetLogMessagesParameters;
      getMessagesInfo;
    };
   type CanisterLogMessagesInfo =
    record {
      count: nat32;
      features: vec opt CanisterLogFeature;
      firstTimeNanos: opt Nanos;
      lastTimeNanos: opt Nanos;
    };
   type CanisterLogMessages =
    record {
      data: vec LogMessagesData;
      lastAnalyzedMessageTimeNanos: opt Nanos;
    };
   type CanisterLogFeature =
    variant {
      filterMessageByContains;
      filterMessageByRegex;
    };
   type CanisterHeapMemoryAggregatedData = vec nat64;
   type CanisterCyclesAggregatedData = vec nat64;
   type CandyValue =
    variant {
      Array: variant {
               frozen: vec CandyValue;
               thawed: vec CandyValue;
             };
      Blob: blob;
      Bool: bool;
      Bytes: variant {
               frozen: vec nat8;
               thawed: vec nat8;
             };
      Class: vec Property;
      Empty;
      Float: float64;
      Floats: variant {
                frozen: vec float64;
                thawed: vec float64;
              };
      Int: int;
      Int16: int16;
      Int32: int32;
      Int64: int64;
      Int8: int8;
      Nat: nat;
      Nat16: nat16;
      Nat32: nat32;
      Nat64: nat64;
      Nat8: nat8;
      Nats: variant {
              frozen: vec nat;
              thawed: vec nat;
            };
      Option: opt CandyValue;
      Principal: principal;
      Text: text;
    };
   type Caller = opt principal;
   type BidResponse =
    record {
      index: nat;
      timestamp: int;
      token_id: text;
      txn_type:
       variant {
         auction_bid:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            sale_id: text;
            token: TokenSpec;
          };
         burn;
         canister_managers_updated:
          record {
            extensible: CandyValue;
            managers: vec principal;
          };
         canister_network_updated:
          record {
            extensible: CandyValue;
            network: principal;
          };
         canister_owner_updated:
          record {
            extensible: CandyValue;
            owner: principal;
          };
         data;
         deposit_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            token: TokenSpec;
            trx_id: TransactionID;
          };
         escrow_deposit:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
         escrow_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
         extensible: CandyValue;
         mint:
          record {
            extensible: CandyValue;
            from: Account__1;
            sale: opt record {
                        amount: nat;
                        token: TokenSpec;
                      };
            to: Account__1;
          };
         owner_transfer:
          record {
            extensible: CandyValue;
            from: Account__1;
            to: Account__1;
          };
         royalty_paid:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            reciever: Account__1;
            sale_id: opt text;
            seller: Account__1;
            tag: text;
            token: TokenSpec;
          };
         sale_ended:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            sale_id: opt text;
            seller: Account__1;
            token: TokenSpec;
          };
         sale_opened:
          record {
            extensible: CandyValue;
            pricing: PricingConfig;
            sale_id: text;
          };
         sale_withdraw:
          record {
            amount: nat;
            buyer: Account__1;
            extensible: CandyValue;
            fee: nat;
            seller: Account__1;
            token: TokenSpec;
            token_id: text;
            trx_id: TransactionID;
          };
       };
    };
   type BidRequest =
    record {
      broker_id: opt principal;
      escrow_receipt: EscrowReceipt;
      sale_id: text;
    };
   type BalanceResponse__1 =
    variant {
      err: CommonError;
      ok: Balance;
    };
   type BalanceResponse =
    record {
      escrow: vec EscrowRecord;
      multi_canister: opt vec principal;
      nfts: vec text;
      offers: vec EscrowRecord;
      sales: vec EscrowRecord;
      stake: vec StakeRecord;
    };
   type BalanceRequest =
    record {
      token: TokenIdentifier;
      user: User;
    };
   type Balance = nat;
   type AuctionStateStable =
    record {
      allow_list: opt vec record {
                            principal;
                            bool;
                          };
      config: PricingConfig__1;
      current_bid_amount: nat;
      current_broker_id: opt principal;
      current_escrow: opt EscrowReceipt;
      end_date: int;
      min_next_bid: nat;
      participants: vec record {
                          principal;
                          int;
                        };
      status: variant {
                closed;
                not_started;
                open;
              };
      wait_for_quiet_count: opt nat;
      winner: opt Account;
    };
   type AuctionConfig =
    record {
      allow_list: opt vec principal;
      buy_now: opt nat;
      ending:
       variant {
         date: int;
         waitForQuiet:
          record {
            date: int;
            extention: nat64;
            fade: float64;
            max: nat;
          };
       };
      min_increase: variant {
                      amount: nat;
                      percentage: float64;
                    };
      reserve: opt nat;
      start_date: int;
      start_price: nat;
      token: TokenSpec;
    };
   type AllocationRecordStable =
    record {
      allocated_space: nat;
      available_space: nat;
      canister: principal;
      chunks: vec nat;
      library_id: text;
      token_id: text;
    };
   type Account__1 =
    variant {
      account: record {
                 owner: principal;
                 sub_account: opt blob;
               };
      account_id: text;
      extensible: CandyValue;
      "principal": principal;
    };
   type AccountIdentifier = text;
   type Account =
    variant {
      account: record {
                 owner: principal;
                 sub_account: opt blob;
               };
      account_id: text;
      extensible: CandyValue;
      "principal": principal;
    };
   service : {
     __advance_time: (int) -> (int);
     __set_time_mode: (variant {
                         standard;
                         test;
                       }) -> (bool);
     __supports: () -> (vec record {
                              text;
                              text;
                            }) query;
     back_up: (nat) ->
      (variant {
         data: NFTBackupChunk;
         eof: NFTBackupChunk;
       }) query;
     balance: (BalanceRequest) -> (BalanceResponse__1) query;
     balanceEXT: (BalanceRequest) -> (BalanceResponse__1) query;
     balanceOfDip721: (principal) -> (nat) query;
     balance_of_nft_origyn: (Account) -> (Result_19) query;
     balance_of_secure_nft_origyn: (Account) -> (Result_19);
     bearer: (TokenIdentifier) -> (Result_18) query;
     bearerEXT: (TokenIdentifier) -> (Result_18) query;
     bearer_batch_nft_origyn: (vec text) -> (vec Result_17) query;
     bearer_batch_secure_nft_origyn: (vec text) -> (vec Result_17);
     bearer_nft_origyn: (text) -> (Result_17) query;
     bearer_secure_nft_origyn: (text) -> (Result_17);
     canister_status: (record {canister_id: canister_id;}) -> (canister_status);
     chunk_nft_origyn: (ChunkRequest) -> (Result_16) query;
     chunk_secure_nft_origyn: (ChunkRequest) -> (Result_16);
     collectCanisterMetrics: () -> () query;
     collection_nft_origyn: (opt vec record {
                                       text;
                                       opt nat;
                                       opt nat;
                                     }) -> (Result_15) query;
     collection_secure_nft_origyn: (opt vec record {
                                              text;
                                              opt nat;
                                              opt nat;
                                            }) -> (Result_15);
     collection_update_batch_nft_origyn: (vec ManageCollectionCommand) ->
      (vec Result_14);
     collection_update_nft_origyn: (ManageCollectionCommand) -> (Result_14);
     current_log: () -> (vec LogEntry) query;
     cycles: () -> (nat) query;
     getCanisterLog: (opt CanisterLogRequest) -> (opt CanisterLogResponse) query;
     getCanisterMetrics: (GetMetricsParameters) -> (opt CanisterMetrics) query;
     getEXTTokenIdentifier: (text) -> (text) query;
     get_access_key: () -> (Result_3) query;
     get_halt: () -> (bool) query;
     get_nat_as_token_id_origyn: (nat) -> (text) query;
     get_token_id_as_nat_origyn: (text) -> (nat) query;
     governance_nft_origyn: (GovernanceRequest) -> (Result_13);
     harvest_log: (nat) -> (vec vec LogEntry);
     history_batch_nft_origyn: (vec record {
                                      text;
                                      opt nat;
                                      opt nat;
                                    }) -> (vec Result_12) query;
     history_batch_secure_nft_origyn: (vec record {
                                             text;
                                             opt nat;
                                             opt nat;
                                           }) -> (vec Result_12);
     history_nft_origyn: (text, opt nat, opt nat) -> (Result_12) query;
     history_secure_nft_origyn: (text, opt nat, opt nat) -> (Result_12);
     http_access_key: () -> (Result_3);
     http_request: (HttpRequest) -> (HTTPResponse) query;
     http_request_streaming_callback: (StreamingCallbackToken) ->
      (StreamingCallbackResponse) query;
     log_history_page: (nat) -> (vec LogEntry) query;
     log_history_page_chunk: (nat, nat, nat) -> (vec LogEntry) query;
     log_history_size: () -> (nat) query;
     manage_storage_nft_origyn: (ManageStorageRequest) -> (Result_11);
     market_transfer_batch_nft_origyn: (vec MarketTransferRequest) ->
      (vec Result_10);
     market_transfer_nft_origyn: (MarketTransferRequest) -> (Result_10);
     metadata: (TokenIdentifier) -> (Result_9) query;
     mint_batch_nft_origyn: (vec record {
                                   text;
                                   Account;
                                 }) -> (vec Result_3);
     mint_nft_origyn: (text, Account) -> (Result_3);
     nftStreamingCallback: (StreamingCallbackToken) ->
      (StreamingCallbackResponse) query;
     nft_batch_origyn: (vec text) -> (vec Result_8) query;
     nft_batch_secure_origyn: (vec text) -> (vec Result_8);
     nft_origyn: (text) -> (Result_8) query;
     nft_secure_origyn: (text) -> (Result_8);
     nuke_log: () -> ();
     ownerOf: (nat) -> (OwnerOfResponse) query;
     ownerOfDIP721: (nat) -> (OwnerOfResponse) query;
     sale_batch_nft_origyn: (vec ManageSaleRequest) -> (vec Result_6);
     sale_info_batch_nft_origyn: (vec SaleInfoRequest) -> (vec Result_7) query;
     sale_info_batch_secure_nft_origyn: (vec SaleInfoRequest) -> (vec Result_7);
     sale_info_nft_origyn: (SaleInfoRequest) -> (Result_7) query;
     sale_info_secure_nft_origyn: (SaleInfoRequest) -> (Result_7);
     sale_nft_origyn: (ManageSaleRequest) -> (Result_6);
     set_data_harvester: (nat) -> ();
     set_halt: (bool) -> ();
     set_log_harvester_id: (principal) -> ();
     share_wallet_nft_origyn: (ShareWalletRequest) -> (Result_5);
     stage_batch_nft_origyn: (vec record {metadata: CandyValue;}) ->
      (vec Result_3);
     stage_library_batch_nft_origyn: (vec StageChunkArg) -> (vec Result_4);
     stage_library_nft_origyn: (StageChunkArg) -> (Result_4);
     stage_nft_origyn: (record {metadata: CandyValue;}) -> (Result_3);
     state_size: () -> (StateSize) query;
     storage_info_nft_origyn: () -> (Result_2) query;
     storage_info_secure_nft_origyn: () -> (Result_2);
     tokens_ext: (text) -> (Result_1) query;
     transfer: (TransferRequest) -> (TransferResponse);
     transferDip721: (principal, nat) -> (Result__1);
     transferEXT: (TransferRequest) -> (TransferResponse);
     transferFrom: (principal, principal, nat) -> (Result__1);
     transferFromDip721: (principal, principal, nat) -> (Result__1);
     update_app_nft_origyn: (NFTUpdateRequest) -> (Result);
     wallet_receive: () -> (nat);
     whoami: () -> (principal) query;
   }
   "##;

        test_single_candid(candid3, "candid3");

        let candid4 = r##"type Tokens = nat;

        type InitArg = record {
            ledger_id: principal;
        };

        type UpgradeArg = record {
            ledger_id: opt principal;
        };

        type IndexArg = variant {
            Init: InitArg;
            Upgrade: UpgradeArg;
        };

        type GetBlocksRequest = record {
            start : nat;
            length : nat;
        };

        type Value = variant {
            Blob : blob;
            Text : text;
            Nat : nat;
            Nat64: nat64;
            Int : int;
            Array : vec Value;
            Map : Map;
        };

        type Map = vec record { text; Value };

        type Block = Value;

        type GetBlocksResponse = record {
            chain_length: nat64;
            blocks: vec Block;
        };

        type BlockIndex = nat;

        type SubAccount = blob;

        type Account = record { owner : principal; subaccount : opt SubAccount };

        type Transaction = record {
          burn : opt Burn;
          kind : text;
          mint : opt Mint;
          approve : opt Approve;
          timestamp : nat64;
          transfer : opt Transfer;
        };

        type Approve = record {
          fee : opt nat;
          from : Account;
          memo : opt vec nat8;
          created_at_time : opt nat64;
          amount : nat;
          expected_allowance : opt nat;
          expires_at : opt nat64;
          spender : Account;
        };

        type Burn = record {
          from : Account;
          memo : opt vec nat8;
          created_at_time : opt nat64;
          amount : nat;
          spender : opt Account;
        };

        type Mint = record {
          to : Account;
          memo : opt vec nat8;
          created_at_time : opt nat64;
          amount : nat;
        };

        type Transfer = record {
          to : Account;
          fee : opt nat;
          from : Account;
          memo : opt vec nat8;
          created_at_time : opt nat64;
          amount : nat;
          spender : opt Account;
        };

        type GetAccountTransactionsArgs = record {
            account : Account;
            // The txid of the last transaction seen by the client.
            // If None then the results will start from the most recent
            // txid.
            start : opt BlockIndex;
            // Maximum number of transactions to fetch.
            max_results : nat;
        };

        type TransactionWithId = record {
          id : BlockIndex;
          transaction : Transaction;
        };

        type GetTransactions = record {
          balance : Tokens;
          transactions : vec TransactionWithId;
          // The txid of the oldest transaction the account has
          oldest_tx_id : opt BlockIndex;
        };

        type GetTransactionsErr = record {
          message : text;
        };

        type GetTransactionsResult = variant {
          Ok : GetTransactions;
          Err : GetTransactionsErr;
        };

        type ListSubaccountsArgs = record {
            owner: principal;
            start: opt SubAccount;
        };

        type Status = record {
            num_blocks_synced : BlockIndex;
        };

        type FeeCollectorRanges = record {
            ranges : vec  record { Account; vec record { BlockIndex; BlockIndex } };
        }

        service : (index_arg: opt IndexArg) -> {
            get_account_transactions : (GetAccountTransactionsArgs) -> (GetTransactionsResult) query;
            get_blocks : (GetBlocksRequest) -> (GetBlocksResponse) query;
            get_fee_collectors_ranges : () -> (FeeCollectorRanges) query;
            icrc1_balance_of : (Account) -> (Tokens) query;
            ledger_id : () -> (principal) query;
            list_subaccounts : (ListSubaccountsArgs) -> (vec SubAccount) query;
            status : () -> (Status) query;
        }
   "##;

        test_single_candid(candid4, "candid4");

        let candid5 = r##"//123
        /*123*/
        service: { test: (text) -> (text) query }"##;
        test_single_candid(candid5, "candid5");

        let candid6 = r##"type AccountIdentifier = record {
  hash : blob;
};

type Action = variant {
  RegisterKnownNeuron : KnownNeuron;
  ManageNeuron : ManageNeuron;
  UpdateCanisterSettings : UpdateCanisterSettings;
  InstallCode : InstallCode;
  StopOrStartCanister : StopOrStartCanister;
  CreateServiceNervousSystem : CreateServiceNervousSystem;
  ExecuteNnsFunction : ExecuteNnsFunction;
  RewardNodeProvider : RewardNodeProvider;
  OpenSnsTokenSwap : OpenSnsTokenSwap;
  SetSnsTokenSwapOpenTimeWindow : SetSnsTokenSwapOpenTimeWindow;
  SetDefaultFollowees : SetDefaultFollowees;
  RewardNodeProviders : RewardNodeProviders;
  ManageNetworkEconomics : NetworkEconomics;
  ApproveGenesisKyc : Principals;
  AddOrRemoveNodeProvider : AddOrRemoveNodeProvider;
  Motion : Motion;
};

type AddHotKey = record {
  new_hot_key : opt principal;
};

type AddOrRemoveNodeProvider = record {
  change : opt Change;
};

type Amount = record {
  e8s : nat64;
};

type ApproveGenesisKyc = record {
  principals : vec principal;
};

type Ballot = record {
  vote : int32;
  voting_power : nat64;
};

type BallotInfo = record {
  vote : int32;
  proposal_id : opt ProposalId;
};

type By = variant {
  NeuronIdOrSubaccount : record {};
  MemoAndController : ClaimOrRefreshNeuronFromAccount;
  Memo : nat64;
};

type Canister = record {
  id : opt principal;
};

type CanisterSettings = record {
  freezing_threshold : opt nat64;
  controllers : opt Controllers;
  log_visibility : opt int32;
  wasm_memory_limit : opt nat64;
  memory_allocation : opt nat64;
  compute_allocation : opt nat64;
};

type CanisterStatusResultV2 = record {
  status : opt int32;
  freezing_threshold : opt nat64;
  controllers : vec principal;
  memory_size : opt nat64;
  cycles : opt nat64;
  idle_cycles_burned_per_day : opt nat64;
  module_hash : blob;
};

type CanisterSummary = record {
  status : opt CanisterStatusResultV2;
  canister_id : opt principal;
};

type Change = variant {
  ToRemove : NodeProvider;
  ToAdd : NodeProvider;
};

type ChangeAutoStakeMaturity = record {
  requested_setting_for_auto_stake_maturity : bool;
};

type ClaimOrRefresh = record {
  by : opt By;
};

type ClaimOrRefreshNeuronFromAccount = record {
  controller : opt principal;
  memo : nat64;
};

type ClaimOrRefreshNeuronFromAccountResponse = record {
  result : opt Result_1;
};

type ClaimOrRefreshResponse = record {
  refreshed_neuron_id : opt NeuronId;
};

type Command = variant {
  Spawn : Spawn;
  Split : Split;
  Follow : Follow;
  ClaimOrRefresh : ClaimOrRefresh;
  Configure : Configure;
  RegisterVote : RegisterVote;
  Merge : Merge;
  DisburseToNeuron : DisburseToNeuron;
  MakeProposal : Proposal;
  StakeMaturity : StakeMaturity;
  MergeMaturity : MergeMaturity;
  Disburse : Disburse;
};

type Command_1 = variant {
  Error : GovernanceError;
  Spawn : SpawnResponse;
  Split : SpawnResponse;
  Follow : record {};
  ClaimOrRefresh : ClaimOrRefreshResponse;
  Configure : record {};
  RegisterVote : record {};
  Merge : MergeResponse;
  DisburseToNeuron : SpawnResponse;
  MakeProposal : MakeProposalResponse;
  StakeMaturity : StakeMaturityResponse;
  MergeMaturity : MergeMaturityResponse;
  Disburse : DisburseResponse;
};

type Command_2 = variant {
  Spawn : NeuronId;
  Split : Split;
  Configure : Configure;
  Merge : Merge;
  DisburseToNeuron : DisburseToNeuron;
  SyncCommand : record {};
  ClaimOrRefreshNeuron : ClaimOrRefresh;
  MergeMaturity : MergeMaturity;
  Disburse : Disburse;
};

type Committed = record {
  total_direct_contribution_icp_e8s : opt nat64;
  total_neurons_fund_contribution_icp_e8s : opt nat64;
  sns_governance_canister_id : opt principal;
};

type Committed_1 = record {
  total_direct_participation_icp_e8s : opt nat64;
  total_neurons_fund_participation_icp_e8s : opt nat64;
  sns_governance_canister_id : opt principal;
};

type Configure = record {
  operation : opt Operation;
};

type Controllers = record {
  controllers : vec principal;
};

type Countries = record {
  iso_codes : vec text;
};

type CreateServiceNervousSystem = record {
  url : opt text;
  governance_parameters : opt GovernanceParameters;
  fallback_controller_principal_ids : vec principal;
  logo : opt Image;
  name : opt text;
  ledger_parameters : opt LedgerParameters;
  description : opt text;
  dapp_canisters : vec Canister;
  swap_parameters : opt SwapParameters;
  initial_token_distribution : opt InitialTokenDistribution;
};

type DateRangeFilter = record {
  start_timestamp_seconds : opt nat64;
  end_timestamp_seconds : opt nat64;
};

type Decimal = record {
  human_readable : opt text;
};

type DerivedProposalInformation = record {
  swap_background_information : opt SwapBackgroundInformation;
};

type DeveloperDistribution = record {
  developer_neurons : vec NeuronDistribution;
};

type Disburse = record {
  to_account : opt AccountIdentifier;
  amount : opt Amount;
};

type DisburseResponse = record {
  transfer_block_height : nat64;
};

type DisburseToNeuron = record {
  dissolve_delay_seconds : nat64;
  kyc_verified : bool;
  amount_e8s : nat64;
  new_controller : opt principal;
  nonce : nat64;
};

type DissolveState = variant {
  DissolveDelaySeconds : nat64;
  WhenDissolvedTimestampSeconds : nat64;
};

type Duration = record {
  seconds : opt nat64;
};

type ExecuteNnsFunction = record {
  nns_function : int32;
  payload : blob;
};

type Follow = record {
  topic : int32;
  followees : vec NeuronId;
};

type Followees = record {
  followees : vec NeuronId;
};

type Followers = record {
  followers : vec NeuronId;
};

type FollowersMap = record {
  followers_map : vec record { nat64; Followers };
};

type GetNeuronsFundAuditInfoRequest = record {
  nns_proposal_id : opt ProposalId;
};

type GetNeuronsFundAuditInfoResponse = record {
  result : opt Result_6;
};

type GlobalTimeOfDay = record {
  seconds_after_utc_midnight : opt nat64;
};

type Governance = record {
  default_followees : vec record { int32; Followees };
  making_sns_proposal : opt MakingSnsProposal;
  most_recent_monthly_node_provider_rewards : opt MonthlyNodeProviderRewards;
  maturity_modulation_last_updated_at_timestamp_seconds : opt nat64;
  wait_for_quiet_threshold_seconds : nat64;
  metrics : opt GovernanceCachedMetrics;
  neuron_management_voting_period_seconds : opt nat64;
  node_providers : vec NodeProvider;
  cached_daily_maturity_modulation_basis_points : opt int32;
  economics : opt NetworkEconomics;
  restore_aging_summary : opt RestoreAgingSummary;
  spawning_neurons : opt bool;
  latest_reward_event : opt RewardEvent;
  to_claim_transfers : vec NeuronStakeTransfer;
  short_voting_period_seconds : nat64;
  topic_followee_index : vec record { int32; FollowersMap };
  migrations : opt Migrations;
  proposals : vec record { nat64; ProposalData };
  xdr_conversion_rate : opt XdrConversionRate;
  in_flight_commands : vec record { nat64; NeuronInFlightCommand };
  neurons : vec record { nat64; Neuron };
  genesis_timestamp_seconds : nat64;
};

type GovernanceCachedMetrics = record {
  total_maturity_e8s_equivalent : nat64;
  not_dissolving_neurons_e8s_buckets : vec record { nat64; float64 };
  dissolving_neurons_staked_maturity_e8s_equivalent_sum : nat64;
  garbage_collectable_neurons_count : nat64;
  dissolving_neurons_staked_maturity_e8s_equivalent_buckets : vec record {
    nat64;
    float64;
  };
  neurons_with_invalid_stake_count : nat64;
  not_dissolving_neurons_count_buckets : vec record { nat64; nat64 };
  ect_neuron_count : nat64;
  total_supply_icp : nat64;
  neurons_with_less_than_6_months_dissolve_delay_count : nat64;
  dissolved_neurons_count : nat64;
  community_fund_total_maturity_e8s_equivalent : nat64;
  total_staked_e8s_seed : nat64;
  total_staked_maturity_e8s_equivalent_ect : nat64;
  total_staked_e8s : nat64;
  not_dissolving_neurons_count : nat64;
  total_locked_e8s : nat64;
  neurons_fund_total_active_neurons : nat64;
  total_voting_power_non_self_authenticating_controller : opt nat64;
  total_staked_maturity_e8s_equivalent : nat64;
  not_dissolving_neurons_e8s_buckets_ect : vec record { nat64; float64 };
  total_staked_e8s_ect : nat64;
  not_dissolving_neurons_staked_maturity_e8s_equivalent_sum : nat64;
  dissolved_neurons_e8s : nat64;
  total_staked_e8s_non_self_authenticating_controller : opt nat64;
  dissolving_neurons_e8s_buckets_seed : vec record { nat64; float64 };
  neurons_with_less_than_6_months_dissolve_delay_e8s : nat64;
  not_dissolving_neurons_staked_maturity_e8s_equivalent_buckets : vec record {
    nat64;
    float64;
  };
  dissolving_neurons_count_buckets : vec record { nat64; nat64 };
  dissolving_neurons_e8s_buckets_ect : vec record { nat64; float64 };
  non_self_authenticating_controller_neuron_subset_metrics : opt NeuronSubsetMetrics;
  dissolving_neurons_count : nat64;
  dissolving_neurons_e8s_buckets : vec record { nat64; float64 };
  total_staked_maturity_e8s_equivalent_seed : nat64;
  community_fund_total_staked_e8s : nat64;
  not_dissolving_neurons_e8s_buckets_seed : vec record { nat64; float64 };
  public_neuron_subset_metrics : opt NeuronSubsetMetrics;
  timestamp_seconds : nat64;
  seed_neuron_count : nat64;
};

type GovernanceError = record {
  error_message : text;
  error_type : int32;
};

type GovernanceParameters = record {
  neuron_maximum_dissolve_delay_bonus : opt Percentage;
  neuron_maximum_age_for_age_bonus : opt Duration;
  neuron_maximum_dissolve_delay : opt Duration;
  neuron_minimum_dissolve_delay_to_vote : opt Duration;
  neuron_maximum_age_bonus : opt Percentage;
  neuron_minimum_stake : opt Tokens;
  proposal_wait_for_quiet_deadline_increase : opt Duration;
  proposal_initial_voting_period : opt Duration;
  proposal_rejection_fee : opt Tokens;
  voting_reward_parameters : opt VotingRewardParameters;
};

type IdealMatchedParticipationFunction = record {
  serialized_representation : opt text;
};

type Image = record {
  base64_encoding : opt text;
};

type IncreaseDissolveDelay = record {
  additional_dissolve_delay_seconds : nat32;
};

type InitialTokenDistribution = record {
  treasury_distribution : opt SwapDistribution;
  developer_distribution : opt DeveloperDistribution;
  swap_distribution : opt SwapDistribution;
};

type InstallCode = record {
  skip_stopping_before_installing : opt bool;
  wasm_module_hash : opt blob;
  canister_id : opt principal;
  arg_hash : opt blob;
  install_mode : opt int32;
};

type InstallCodeRequest = record {
  arg : opt blob;
  wasm_module : opt blob;
  skip_stopping_before_installing : opt bool;
  canister_id : opt principal;
  install_mode : opt int32;
};

type KnownNeuron = record {
  id : opt NeuronId;
  known_neuron_data : opt KnownNeuronData;
};

type KnownNeuronData = record {
  name : text;
  description : opt text;
};

type LedgerParameters = record {
  transaction_fee : opt Tokens;
  token_symbol : opt text;
  token_logo : opt Image;
  token_name : opt text;
};

type ListKnownNeuronsResponse = record {
  known_neurons : vec KnownNeuron;
};

type ListNeurons = record {
  include_public_neurons_in_full_neurons : opt bool;
  neuron_ids : vec nat64;
  include_empty_neurons_readable_by_caller : opt bool;
  include_neurons_readable_by_caller : bool;
};

type ListNeuronsResponse = record {
  neuron_infos : vec record { nat64; NeuronInfo };
  full_neurons : vec Neuron;
};

type ListNodeProviderRewardsRequest = record {
  date_filter : opt DateRangeFilter;
};

type ListNodeProviderRewardsResponse = record {
  rewards : vec MonthlyNodeProviderRewards;
};

type ListNodeProvidersResponse = record {
  node_providers : vec NodeProvider;
};

type ListProposalInfo = record {
  include_reward_status : vec int32;
  omit_large_fields : opt bool;
  before_proposal : opt ProposalId;
  limit : nat32;
  exclude_topic : vec int32;
  include_all_manage_neuron_proposals : opt bool;
  include_status : vec int32;
};

type ListProposalInfoResponse = record {
  proposal_info : vec ProposalInfo;
};

type MakeProposalRequest = record {
  url : text;
  title : opt text;
  action : opt ProposalActionRequest;
  summary : text;
};

type MakeProposalResponse = record {
  message : opt text;
  proposal_id : opt ProposalId;
};

type MakingSnsProposal = record {
  proposal : opt Proposal;
  caller : opt principal;
  proposer_id : opt NeuronId;
};

type ManageNeuron = record {
  id : opt NeuronId;
  command : opt Command;
  neuron_id_or_subaccount : opt NeuronIdOrSubaccount;
};

type ManageNeuronCommandRequest = variant {
  Spawn : Spawn;
  Split : Split;
  Follow : Follow;
  ClaimOrRefresh : ClaimOrRefresh;
  Configure : Configure;
  RegisterVote : RegisterVote;
  Merge : Merge;
  DisburseToNeuron : DisburseToNeuron;
  MakeProposal : MakeProposalRequest;
  StakeMaturity : StakeMaturity;
  MergeMaturity : MergeMaturity;
  Disburse : Disburse;
};

type ManageNeuronRequest = record {
  id : opt NeuronId;
  command : opt ManageNeuronCommandRequest;
  neuron_id_or_subaccount : opt NeuronIdOrSubaccount;
};

type ManageNeuronResponse = record {
  command : opt Command_1;
};

type Merge = record {
  source_neuron_id : opt NeuronId;
};

type MergeMaturity = record {
  percentage_to_merge : nat32;
};

type MergeMaturityResponse = record {
  merged_maturity_e8s : nat64;
  new_stake_e8s : nat64;
};

type MergeResponse = record {
  target_neuron : opt Neuron;
  source_neuron : opt Neuron;
  target_neuron_info : opt NeuronInfo;
  source_neuron_info : opt NeuronInfo;
};

type Migration = record {
  status : opt int32;
  failure_reason : opt text;
  progress : opt Progress;
};

type Migrations = record {
  neuron_indexes_migration : opt Migration;
  copy_inactive_neurons_to_stable_memory_migration : opt Migration;
};

type MonthlyNodeProviderRewards = record {
  minimum_xdr_permyriad_per_icp : opt nat64;
  registry_version : opt nat64;
  node_providers : vec NodeProvider;
  timestamp : nat64;
  rewards : vec RewardNodeProvider;
  xdr_conversion_rate : opt XdrConversionRate;
  maximum_node_provider_rewards_e8s : opt nat64;
};

type Motion = record {
  motion_text : text;
};

type NetworkEconomics = record {
  neuron_minimum_stake_e8s : nat64;
  max_proposals_to_keep_per_topic : nat32;
  neuron_management_fee_per_proposal_e8s : nat64;
  reject_cost_e8s : nat64;
  transaction_fee_e8s : nat64;
  neuron_spawn_dissolve_delay_seconds : nat64;
  minimum_icp_xdr_rate : nat64;
  maximum_node_provider_rewards_e8s : nat64;
  neurons_fund_economics : opt NeuronsFundEconomics;
};

type Neuron = record {
  id : opt NeuronId;
  staked_maturity_e8s_equivalent : opt nat64;
  controller : opt principal;
  recent_ballots : vec BallotInfo;
  kyc_verified : bool;
  neuron_type : opt int32;
  not_for_profit : bool;
  maturity_e8s_equivalent : nat64;
  cached_neuron_stake_e8s : nat64;
  created_timestamp_seconds : nat64;
  auto_stake_maturity : opt bool;
  aging_since_timestamp_seconds : nat64;
  hot_keys : vec principal;
  account : blob;
  joined_community_fund_timestamp_seconds : opt nat64;
  dissolve_state : opt DissolveState;
  followees : vec record { int32; Followees };
  neuron_fees_e8s : nat64;
  visibility : opt int32;
  transfer : opt NeuronStakeTransfer;
  known_neuron_data : opt KnownNeuronData;
  spawn_at_timestamp_seconds : opt nat64;
};

type NeuronBasketConstructionParameters = record {
  dissolve_delay_interval : opt Duration;
  count : opt nat64;
};

type NeuronBasketConstructionParameters_1 = record {
  dissolve_delay_interval_seconds : nat64;
  count : nat64;
};

type NeuronDistribution = record {
  controller : opt principal;
  dissolve_delay : opt Duration;
  memo : opt nat64;
  vesting_period : opt Duration;
  stake : opt Tokens;
};

type NeuronId = record {
  id : nat64;
};

type ProposalId = record {
  id : nat64;
};

type NeuronIdOrSubaccount = variant {
  Subaccount : blob;
  NeuronId : NeuronId;
};

type NeuronInFlightCommand = record {
  command : opt Command_2;
  timestamp : nat64;
};

type NeuronInfo = record {
  dissolve_delay_seconds : nat64;
  recent_ballots : vec BallotInfo;
  neuron_type : opt int32;
  created_timestamp_seconds : nat64;
  state : int32;
  stake_e8s : nat64;
  joined_community_fund_timestamp_seconds : opt nat64;
  retrieved_at_timestamp_seconds : nat64;
  visibility : opt int32;
  known_neuron_data : opt KnownNeuronData;
  voting_power : nat64;
  age_seconds : nat64;
};

type NeuronStakeTransfer = record {
  to_subaccount : blob;
  neuron_stake_e8s : nat64;
  from : opt principal;
  memo : nat64;
  from_subaccount : blob;
  transfer_timestamp : nat64;
  block_height : nat64;
};

type NeuronSubsetMetrics = record {
  total_maturity_e8s_equivalent : opt nat64;
  maturity_e8s_equivalent_buckets : vec record { nat64; nat64 };
  voting_power_buckets : vec record { nat64; nat64 };
  total_staked_e8s : opt nat64;
  count : opt nat64;
  total_staked_maturity_e8s_equivalent : opt nat64;
  staked_maturity_e8s_equivalent_buckets : vec record { nat64; nat64 };
  staked_e8s_buckets : vec record { nat64; nat64 };
  total_voting_power : opt nat64;
  count_buckets : vec record { nat64; nat64 };
};

type NeuronsFundAuditInfo = record {
  final_neurons_fund_participation : opt NeuronsFundParticipation;
  initial_neurons_fund_participation : opt NeuronsFundParticipation;
  neurons_fund_refunds : opt NeuronsFundSnapshot;
};

type NeuronsFundData = record {
  final_neurons_fund_participation : opt NeuronsFundParticipation;
  initial_neurons_fund_participation : opt NeuronsFundParticipation;
  neurons_fund_refunds : opt NeuronsFundSnapshot;
};

type NeuronsFundEconomics = record {
  maximum_icp_xdr_rate : opt Percentage;
  neurons_fund_matched_funding_curve_coefficients : opt NeuronsFundMatchedFundingCurveCoefficients;
  max_theoretical_neurons_fund_participation_amount_xdr : opt Decimal;
  minimum_icp_xdr_rate : opt Percentage;
};

type NeuronsFundMatchedFundingCurveCoefficients = record {
  contribution_threshold_xdr : opt Decimal;
  one_third_participation_milestone_xdr : opt Decimal;
  full_participation_milestone_xdr : opt Decimal;
};

type NeuronsFundNeuron = record {
  controller : opt principal;
  hotkeys : opt Principals;
  is_capped : opt bool;
  nns_neuron_id : opt nat64;
  amount_icp_e8s : opt nat64;
};

type NeuronsFundNeuronPortion = record {
  controller : opt principal;
  hotkeys : vec principal;
  is_capped : opt bool;
  maturity_equivalent_icp_e8s : opt nat64;
  nns_neuron_id : opt NeuronId;
  amount_icp_e8s : opt nat64;
};

type NeuronsFundParticipation = record {
  total_maturity_equivalent_icp_e8s : opt nat64;
  intended_neurons_fund_participation_icp_e8s : opt nat64;
  direct_participation_icp_e8s : opt nat64;
  swap_participation_limits : opt SwapParticipationLimits;
  max_neurons_fund_swap_participation_icp_e8s : opt nat64;
  neurons_fund_reserves : opt NeuronsFundSnapshot;
  ideal_matched_participation_function : opt IdealMatchedParticipationFunction;
  allocated_neurons_fund_participation_icp_e8s : opt nat64;
};

type NeuronsFundSnapshot = record {
  neurons_fund_neuron_portions : vec NeuronsFundNeuronPortion;
};

type NodeProvider = record {
  id : opt principal;
  reward_account : opt AccountIdentifier;
};

type Ok = record {
  neurons_fund_audit_info : opt NeuronsFundAuditInfo;
};

type Ok_1 = record {
  neurons_fund_neuron_portions : vec NeuronsFundNeuron;
};

type OpenSnsTokenSwap = record {
  community_fund_investment_e8s : opt nat64;
  target_swap_canister_id : opt principal;
  params : opt Params;
};

type Operation = variant {
  RemoveHotKey : RemoveHotKey;
  AddHotKey : AddHotKey;
  ChangeAutoStakeMaturity : ChangeAutoStakeMaturity;
  StopDissolving : record {};
  StartDissolving : record {};
  IncreaseDissolveDelay : IncreaseDissolveDelay;
  SetVisibility : SetVisibility;
  JoinCommunityFund : record {};
  LeaveCommunityFund : record {};
  SetDissolveTimestamp : SetDissolveTimestamp;
};

type Params = record {
  min_participant_icp_e8s : nat64;
  neuron_basket_construction_parameters : opt NeuronBasketConstructionParameters_1;
  max_icp_e8s : nat64;
  swap_due_timestamp_seconds : nat64;
  min_participants : nat32;
  sns_token_e8s : nat64;
  sale_delay_seconds : opt nat64;
  max_participant_icp_e8s : nat64;
  min_direct_participation_icp_e8s : opt nat64;
  min_icp_e8s : nat64;
  max_direct_participation_icp_e8s : opt nat64;
};

type Percentage = record {
  basis_points : opt nat64;
};

type Principals = record {
  principals : vec principal;
};

type Progress = variant {
  LastNeuronId : NeuronId;
};

type Proposal = record {
  url : text;
  title : opt text;
  action : opt Action;
  summary : text;
};

type ProposalActionRequest = variant {
  RegisterKnownNeuron : KnownNeuron;
  ManageNeuron : ManageNeuronRequest;
  UpdateCanisterSettings : UpdateCanisterSettings;
  InstallCode : InstallCodeRequest;
  StopOrStartCanister : StopOrStartCanister;
  CreateServiceNervousSystem : CreateServiceNervousSystem;
  ExecuteNnsFunction : ExecuteNnsFunction;
  RewardNodeProvider : RewardNodeProvider;
  RewardNodeProviders : RewardNodeProviders;
  ManageNetworkEconomics : NetworkEconomics;
  ApproveGenesisKyc : Principals;
  AddOrRemoveNodeProvider : AddOrRemoveNodeProvider;
  Motion : Motion;
};

type ProposalData = record {
  id : opt ProposalId;
  failure_reason : opt GovernanceError;
  ballots : vec record { nat64; Ballot };
  proposal_timestamp_seconds : nat64;
  reward_event_round : nat64;
  failed_timestamp_seconds : nat64;
  neurons_fund_data : opt NeuronsFundData;
  reject_cost_e8s : nat64;
  derived_proposal_information : opt DerivedProposalInformation;
  latest_tally : opt Tally;
  sns_token_swap_lifecycle : opt int32;
  decided_timestamp_seconds : nat64;
  proposal : opt Proposal;
  proposer : opt NeuronId;
  wait_for_quiet_state : opt WaitForQuietState;
  executed_timestamp_seconds : nat64;
  original_total_community_fund_maturity_e8s_equivalent : opt nat64;
};

type ProposalInfo = record {
  id : opt ProposalId;
  status : int32;
  topic : int32;
  failure_reason : opt GovernanceError;
  ballots : vec record { nat64; Ballot };
  proposal_timestamp_seconds : nat64;
  reward_event_round : nat64;
  deadline_timestamp_seconds : opt nat64;
  failed_timestamp_seconds : nat64;
  reject_cost_e8s : nat64;
  derived_proposal_information : opt DerivedProposalInformation;
  latest_tally : opt Tally;
  reward_status : int32;
  decided_timestamp_seconds : nat64;
  proposal : opt Proposal;
  proposer : opt NeuronId;
  executed_timestamp_seconds : nat64;
};

type RegisterVote = record {
  vote : int32;
  proposal : opt ProposalId;
};

type RemoveHotKey = record {
  hot_key_to_remove : opt principal;
};

type RestoreAgingNeuronGroup = record {
  count : opt nat64;
  previous_total_stake_e8s : opt nat64;
  current_total_stake_e8s : opt nat64;
  group_type : int32;
};

type RestoreAgingSummary = record {
  groups : vec RestoreAgingNeuronGroup;
  timestamp_seconds : opt nat64;
};

type Result = variant {
  Ok;
  Err : GovernanceError;
};

type Result_1 = variant {
  Error : GovernanceError;
  NeuronId : NeuronId;
};

type Result_10 = variant {
  Ok : Ok_1;
  Err : GovernanceError;
};

type Result_2 = variant {
  Ok : Neuron;
  Err : GovernanceError;
};

type Result_3 = variant {
  Ok : GovernanceCachedMetrics;
  Err : GovernanceError;
};

type Result_4 = variant {
  Ok : MonthlyNodeProviderRewards;
  Err : GovernanceError;
};

type Result_5 = variant {
  Ok : NeuronInfo;
  Err : GovernanceError;
};

type Result_6 = variant {
  Ok : Ok;
  Err : GovernanceError;
};

type Result_7 = variant {
  Ok : NodeProvider;
  Err : GovernanceError;
};

type Result_8 = variant {
  Committed : Committed;
  Aborted : record {} };

type Result_9 = variant {
  Committed : Committed_1;
  Aborted : record {} };

type RewardEvent = record {
  rounds_since_last_distribution : opt nat64;
  day_after_genesis : nat64;
  actual_timestamp_seconds : nat64;
  total_available_e8s_equivalent : nat64;
  latest_round_available_e8s_equivalent : opt nat64;
  distributed_e8s_equivalent : nat64;
  settled_proposals : vec ProposalId;
};

type RewardMode = variant {
  RewardToNeuron : RewardToNeuron;
  RewardToAccount : RewardToAccount;
};

type RewardNodeProvider = record {
  node_provider : opt NodeProvider;
  reward_mode : opt RewardMode;
  amount_e8s : nat64;
};

type RewardNodeProviders = record {
  use_registry_derived_rewards : opt bool;
  rewards : vec RewardNodeProvider;
};

type RewardToAccount = record {
  to_account : opt AccountIdentifier;
};

type RewardToNeuron = record {
  dissolve_delay_seconds : nat64;
};

type SetDefaultFollowees = record {
  default_followees : vec record { int32; Followees };
};

type SetDissolveTimestamp = record {
  dissolve_timestamp_seconds : nat64;
};

type SetOpenTimeWindowRequest = record {
  open_time_window : opt TimeWindow;
};

type SetSnsTokenSwapOpenTimeWindow = record {
  request : opt SetOpenTimeWindowRequest;
  swap_canister_id : opt principal;
};

type SetVisibility = record {
  visibility : opt int32;
};

type SettleCommunityFundParticipation = record {
  result : opt Result_8;
  open_sns_token_swap_proposal_id : opt nat64;
};

type SettleNeuronsFundParticipationRequest = record {
  result : opt Result_9;
  nns_proposal_id : opt nat64;
};

type SettleNeuronsFundParticipationResponse = record {
  result : opt Result_10;
};

type Spawn = record {
  percentage_to_spawn : opt nat32;
  new_controller : opt principal;
  nonce : opt nat64;
};

type SpawnResponse = record {
  created_neuron_id : opt NeuronId;
};

type Split = record {
  amount_e8s : nat64;
};

type StakeMaturity = record {
  percentage_to_stake : opt nat32;
};

type StakeMaturityResponse = record {
  maturity_e8s : nat64;
  staked_maturity_e8s : nat64;
};

type StopOrStartCanister = record {
  action : opt int32;
  canister_id : opt principal;
};

type SwapBackgroundInformation = record {
  ledger_index_canister_summary : opt CanisterSummary;
  fallback_controller_principal_ids : vec principal;
  ledger_archive_canister_summaries : vec CanisterSummary;
  ledger_canister_summary : opt CanisterSummary;
  swap_canister_summary : opt CanisterSummary;
  governance_canister_summary : opt CanisterSummary;
  root_canister_summary : opt CanisterSummary;
  dapp_canister_summaries : vec CanisterSummary;
};

type SwapDistribution = record {
  total : opt Tokens;
};

type SwapParameters = record {
  minimum_participants : opt nat64;
  neurons_fund_participation : opt bool;
  duration : opt Duration;
  neuron_basket_construction_parameters : opt NeuronBasketConstructionParameters;
  confirmation_text : opt text;
  maximum_participant_icp : opt Tokens;
  minimum_icp : opt Tokens;
  minimum_direct_participation_icp : opt Tokens;
  minimum_participant_icp : opt Tokens;
  start_time : opt GlobalTimeOfDay;
  maximum_direct_participation_icp : opt Tokens;
  maximum_icp : opt Tokens;
  neurons_fund_investment_icp : opt Tokens;
  restricted_countries : opt Countries;
};

type SwapParticipationLimits = record {
  min_participant_icp_e8s : opt nat64;
  max_participant_icp_e8s : opt nat64;
  min_direct_participation_icp_e8s : opt nat64;
  max_direct_participation_icp_e8s : opt nat64;
};

type Tally = record {
  no : nat64;
  yes : nat64;
  total : nat64;
  timestamp_seconds : nat64;
};

type TimeWindow = record {
  start_timestamp_seconds : nat64;
  end_timestamp_seconds : nat64;
};

type Tokens = record {
  e8s : opt nat64;
};

type UpdateCanisterSettings = record {
  canister_id : opt principal;
  settings : opt CanisterSettings;
};

type UpdateNodeProvider = record {
  reward_account : opt AccountIdentifier;
};

type VotingRewardParameters = record {
  reward_rate_transition_duration : opt Duration;
  initial_reward_rate : opt Percentage;
  final_reward_rate : opt Percentage;
};

type WaitForQuietState = record {
  current_deadline_timestamp_seconds : nat64;
};

type XdrConversionRate = record {
  xdr_permyriad_per_icp : opt nat64;
  timestamp_seconds : opt nat64;
};

service : (Governance) -> {
  claim_gtc_neurons : (principal, vec NeuronId) -> (Result);
  claim_or_refresh_neuron_from_account : (ClaimOrRefreshNeuronFromAccount) -> (
      ClaimOrRefreshNeuronFromAccountResponse,
    );
  get_build_metadata : () -> (text) query;
  get_full_neuron : (nat64) -> (Result_2) query;
  get_full_neuron_by_id_or_subaccount : (NeuronIdOrSubaccount) -> (
      Result_2,
    ) query;
  get_latest_reward_event : () -> (RewardEvent) query;
  get_metrics : () -> (Result_3) query;
  get_monthly_node_provider_rewards : () -> (Result_4);
  get_most_recent_monthly_node_provider_rewards : () -> (
      opt MonthlyNodeProviderRewards,
    ) query;
  get_network_economics_parameters : () -> (NetworkEconomics) query;
  get_neuron_ids : () -> (vec nat64) query;
  get_neuron_info : (nat64) -> (Result_5) query;
  get_neuron_info_by_id_or_subaccount : (NeuronIdOrSubaccount) -> (
      Result_5,
    ) query;
  get_neurons_fund_audit_info : (GetNeuronsFundAuditInfoRequest) -> (
      GetNeuronsFundAuditInfoResponse,
    ) query;
  get_node_provider_by_caller : (null) -> (Result_7) query;
  get_pending_proposals : () -> (vec ProposalInfo) query;
  get_proposal_info : (nat64) -> (opt ProposalInfo) query;
  get_restore_aging_summary : () -> (RestoreAgingSummary) query;
  list_known_neurons : () -> (ListKnownNeuronsResponse) query;
  list_neurons : (ListNeurons) -> (ListNeuronsResponse) query;
  list_node_provider_rewards : (ListNodeProviderRewardsRequest) -> (
      ListNodeProviderRewardsResponse,
    ) query;
  list_node_providers : () -> (ListNodeProvidersResponse) query;
  list_proposals : (ListProposalInfo) -> (ListProposalInfoResponse) query;
  manage_neuron : (ManageNeuronRequest) -> (ManageNeuronResponse);
  settle_community_fund_participation : (SettleCommunityFundParticipation) -> (
      Result,
    );
  settle_neurons_fund_participation : (
      SettleNeuronsFundParticipationRequest,
    ) -> (SettleNeuronsFundParticipationResponse);
  simulate_manage_neuron : (ManageNeuronRequest) -> (ManageNeuronResponse);
  transfer_gtc_neuron : (NeuronId, NeuronId) -> (Result);
  update_node_provider : (UpdateNodeProvider) -> (Result);
}
"##;
        test_single_candid(candid6, "candid6");
    }

    #[test]
    fn test_parse_candid2() {
        let candid7 = r##"
        type Proposal = record {
  url : text;
  title : opt text;
  action : opt Action;
  summary : text;
};

type Action = variant {
  ManageNeuron : ManageNeuron;
};

type ManageNeuron = record {
  command : opt Command;
};

type Command = variant {
  MakeProposal : Proposal;
};

type WrappedPropoasl = record {
      proposal: opt Proposal
};

service : {
  claim_gtc_neurons : (Proposal) -> (WrappedPropoasl);
}
"##;
        test_single_candid(candid7, "candid7");
    }
    #[test]
    fn test_parse_candid3() {
        let candid8 = r##"
        type BatchId = nat;
type ChunkId = nat;
type Key = text;
type Time = int;

type CreateAssetArguments = record {
  key: Key;
  content_type: text;
  max_age: opt nat64;
  headers: opt vec HeaderField;
  enable_aliasing: opt bool;
  allow_raw_access: opt bool;
};

// Add or change content for an asset, by content encoding
type SetAssetContentArguments = record {
  key: Key;
  content_encoding: text;
  chunk_ids: vec ChunkId;
  sha256: opt blob;
};

// Remove content for an asset, by content encoding
type UnsetAssetContentArguments = record {
  key: Key;
  content_encoding: text;
};

// Delete an asset
type DeleteAssetArguments = record {
  key: Key;
};

// Reset everything
type ClearArguments = record {};

type BatchOperationKind = variant {
  CreateAsset: CreateAssetArguments;
  SetAssetContent: SetAssetContentArguments;

  SetAssetProperties: SetAssetPropertiesArguments;

  UnsetAssetContent: UnsetAssetContentArguments;
  DeleteAsset: DeleteAssetArguments;

  Clear: ClearArguments;
};

type CommitBatchArguments = record {
  batch_id: BatchId;
  operations: vec BatchOperationKind
};

type CommitProposedBatchArguments = record {
  batch_id: BatchId;
  evidence: blob;
};

type ComputeEvidenceArguments = record {
  batch_id: BatchId;
  max_iterations: opt nat16
};

type DeleteBatchArguments = record {
  batch_id: BatchId;
};

type HeaderField = record { text; text; };

type HttpRequest = record {
  method: text;
  url: text;
  headers: vec HeaderField;
  body: blob;
  certificate_version: opt nat16;
};

type HttpResponse = record {
  status_code: nat16;
  headers: vec HeaderField;
  body: blob;
  streaming_strategy: opt StreamingStrategy;
};

type StreamingCallbackHttpResponse = record {
  body: blob;
  token: opt StreamingCallbackToken;
};

type StreamingCallbackToken = record {
  key: Key;
  content_encoding: text;
  index: nat;
  sha256: opt blob;
};

type StreamingStrategy = variant {
  Callback: record {
    callback: func (StreamingCallbackToken) -> (opt StreamingCallbackHttpResponse) query;
    token: StreamingCallbackToken;
  };
};

type SetAssetPropertiesArguments = record {
  key: Key;
  max_age: opt opt nat64;
  headers: opt opt vec HeaderField;
  allow_raw_access: opt opt bool;
  is_aliased: opt opt bool;
};

type ConfigurationResponse = record {
  max_batches: opt nat64;
  max_chunks: opt nat64;
  max_bytes: opt nat64;
};

type ConfigureArguments = record {
  max_batches: opt opt nat64;
  max_chunks: opt opt nat64;
  max_bytes: opt opt nat64;
};

type Permission = variant {
  Commit;
  ManagePermissions;
  Prepare;
};

type GrantPermission = record {
  to_principal: principal;
  permission: Permission;
};
type RevokePermission = record {
  of_principal: principal;
  permission: Permission;
};
type ListPermitted = record { permission: Permission };

type ValidationResult = variant { Ok : text; Err : text };

type AssetCanisterArgs = variant {
  Init: InitArgs;
  Upgrade: UpgradeArgs;
};

type InitArgs = record {};

type UpgradeArgs = record {
  set_permissions: opt SetPermissions;
};

/// Sets the list of principals granted each permission.
type SetPermissions = record {
  prepare: vec principal;
  commit: vec principal;
  manage_permissions: vec principal;
};

service: (asset_canister_args: opt AssetCanisterArgs) -> {
  api_version: () -> (nat16) query;

  get: (record {
    key: Key;
    accept_encodings: vec text;
  }) -> (record {
    content: blob; // may be the entirety of the content, or just chunk index 0
    content_type: text;
    content_encoding: text;
    sha256: opt blob; // sha256 of entire asset encoding, calculated by dfx and passed in SetAssetContentArguments
    total_length: nat; // all chunks except last have size == content.size()
  }) query;

  // if get() returned chunks > 1, call this to retrieve them.
  // chunks may or may not be split up at the same boundaries as presented to create_chunk().
  get_chunk: (record {
    key: Key;
    content_encoding: text;
    index: nat;
    sha256: opt blob;  // sha256 of entire asset encoding, calculated by dfx and passed in SetAssetContentArguments
  }) -> (record { content: blob }) query;

  list : (record {}) -> (vec record {
    key: Key;
    content_type: text;
    encodings: vec record {
      content_encoding: text;
      sha256: opt blob; // sha256 of entire asset encoding, calculated by dfx and passed in SetAssetContentArguments
      length: nat; // Size of this encoding's blob. Calculated when uploading assets.
      modified: Time;
    };
  }) query;

  certified_tree : (record {}) -> (record {
    certificate: blob;
    tree: blob;
  }) query;

  create_batch : (record {}) -> (record { batch_id: BatchId });

  create_chunk: (record { batch_id: BatchId; content: blob }) -> (record { chunk_id: ChunkId });
  create_chunks: (record { batch_id: BatchId; content: vec blob }) -> (record { chunk_ids: vec ChunkId });

  // Perform all operations successfully, or reject
  commit_batch: (CommitBatchArguments) -> ();

  // Save the batch operations for later commit
  propose_commit_batch: (CommitBatchArguments) -> ();

  // Given a batch already proposed, perform all operations successfully, or reject
  commit_proposed_batch: (CommitProposedBatchArguments) -> ();

  // Compute a hash over the CommitBatchArguments.  Call until it returns Some(evidence).
  compute_evidence: (ComputeEvidenceArguments) -> (opt blob);

  // Delete a batch that has been created, or proposed for commit, but not yet committed
  delete_batch: (DeleteBatchArguments) -> ();

  create_asset: (CreateAssetArguments) -> ();
  set_asset_content: (SetAssetContentArguments) -> ();
  unset_asset_content: (UnsetAssetContentArguments) -> ();

  delete_asset: (DeleteAssetArguments) -> ();

  clear: (ClearArguments) -> ();

  // Single call to create an asset with content for a single content encoding that
  // fits within the message ingress limit.
  store: (record {
    key: Key;
    content_type: text;
    content_encoding: text;
    content: blob;
    sha256: opt blob
  }) -> ();

  http_request: (request: HttpRequest) -> (HttpResponse) query;
  http_request_streaming_callback: (token: StreamingCallbackToken) -> (opt StreamingCallbackHttpResponse) query;

  authorize: (principal) -> ();
  deauthorize: (principal) -> ();
  list_authorized: () -> (vec principal);
  grant_permission: (GrantPermission) -> ();
  revoke_permission: (RevokePermission) -> ();
  list_permitted: (ListPermitted) -> (vec principal);
  take_ownership: () -> ();

  get_asset_properties : (key: Key) -> (record {
    max_age: opt nat64;
    headers: opt vec HeaderField;
    allow_raw_access: opt bool;
    is_aliased: opt bool; } ) query;
  set_asset_properties: (SetAssetPropertiesArguments) -> ();

  get_configuration: () -> (ConfigurationResponse);
  configure: (ConfigureArguments) -> ();

  validate_grant_permission: (GrantPermission) -> (ValidationResult);
  validate_revoke_permission: (RevokePermission) -> (ValidationResult);
  validate_take_ownership: () -> (ValidationResult);
  validate_commit_proposed_batch: (CommitProposedBatchArguments) -> (ValidationResult);
  validate_configure: (ConfigureArguments) -> (ValidationResult);
}
"##;
        test_single_candid(candid8, "candid8");
    }
}
