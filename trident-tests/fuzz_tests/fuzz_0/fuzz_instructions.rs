pub mod anchor_basic_amm_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use trident_client::fuzzing::*;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
        CreatePool(CreatePool),
        AddLiquidity(AddLiquidity),
        Swap(Swap),
        RemoveLiquidity(RemoveLiquidity),
    }
    #[derive(Arbitrary, Debug)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeAccounts {
        pub signer: AccountId,
        pub config: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {
        pub fee: u16,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreatePool {
        pub accounts: CreatePoolAccounts,
        pub data: CreatePoolData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreatePoolAccounts {
        pub maker: AccountId,
        pub pool: AccountId,
        pub mint_x: AccountId,
        pub mint_y: AccountId,
        pub mint_lp: AccountId,
        pub pool_x_ata: AccountId,
        pub pool_y_ata: AccountId,
        pub mint_x_token_program: AccountId,
        pub mint_y_token_program: AccountId,
        pub token_program: AccountId,
        pub associated_token_program: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct CreatePoolData {}
    #[derive(Arbitrary, Debug)]
    pub struct AddLiquidity {
        pub accounts: AddLiquidityAccounts,
        pub data: AddLiquidityData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct AddLiquidityAccounts {
        pub signer: AccountId,
        pub pool: AccountId,
        pub mint_x: AccountId,
        pub mint_y: AccountId,
        pub mint_lp: AccountId,
        pub user_x_ata: AccountId,
        pub user_y_ata: AccountId,
        pub pool_x_ata: AccountId,
        pub pool_y_ata: AccountId,
        pub user_lp_ata: AccountId,
        pub mint_x_token_program: AccountId,
        pub mint_y_token_program: AccountId,
        pub token_program: AccountId,
        pub associated_token_program: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct AddLiquidityData {
        pub amount_x: u64,
        pub amount_y: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Swap {
        pub accounts: SwapAccounts,
        pub data: SwapData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapAccounts {
        pub signer: AccountId,
        pub pool: AccountId,
        pub config: AccountId,
        pub mint_x: AccountId,
        pub mint_y: AccountId,
        pub user_x_ata: AccountId,
        pub user_y_ata: AccountId,
        pub pool_x_ata: AccountId,
        pub pool_y_ata: AccountId,
        pub mint_x_token_program: AccountId,
        pub mint_y_token_program: AccountId,
        pub token_program: AccountId,
        pub associated_token_program: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct SwapData {
        pub x_to_y: bool,
        pub amount_in: u64,
        pub minimum_amount_out: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RemoveLiquidity {
        pub accounts: RemoveLiquidityAccounts,
        pub data: RemoveLiquidityData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RemoveLiquidityAccounts {
        pub signer: AccountId,
        pub pool: AccountId,
        pub mint_x: AccountId,
        pub mint_y: AccountId,
        pub mint_lp: AccountId,
        pub user_x_ata: AccountId,
        pub user_y_ata: AccountId,
        pub pool_x_ata: AccountId,
        pub pool_y_ata: AccountId,
        pub user_lp_ata: AccountId,
        pub mint_x_token_program: AccountId,
        pub mint_y_token_program: AccountId,
        pub token_program: AccountId,
        pub associated_token_program: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct RemoveLiquidityData {
        pub shares: u64,
    }
    impl<'info> IxOps<'info> for Initialize {
        type IxData = anchor_basic_amm::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = anchor_basic_amm::instruction::Initialize { fee: self.data.fee };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.signer.get_or_create_account(
                self.accounts.signer,
                client,
                10 * LAMPORTS_PER_SOL,
            );
            let config = fuzz_accounts
                .config
                .get_or_create_account(self.accounts.config, &[b"config"], &anchor_basic_amm::ID)
                .unwrap();
            let signers = vec![signer.clone()];
            let acc_meta = anchor_basic_amm::accounts::Initialize {
                signer: signer.pubkey(),
                config: config.pubkey(),
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    // impl<'info> IxOps<'info> for CreatePool {
    //     type IxData = anchor_basic_amm::instruction::CreatePool;
    //     type IxAccounts = FuzzAccounts;
    //     type IxSnapshot = CreatePoolSnapshot<'info>;
    //     fn get_data(
    //         &self,
    //         _client: &mut impl FuzzClient,
    //         _fuzz_accounts: &mut FuzzAccounts,
    //     ) -> Result<Self::IxData, FuzzingError> {
    //         let data = anchor_basic_amm::instruction::CreatePool {};
    //         Ok(data)
    //     }
    //     fn get_accounts(
    //         &self,
    //         client: &mut impl FuzzClient,
    //         fuzz_accounts: &mut FuzzAccounts,
    //     ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
    //         let signers = vec![todo!()];
    //         let acc_meta = anchor_basic_amm::accounts::CreatePool {
    //             maker: todo!(),
    //             pool: todo!(),
    //             mint_x: todo!(),
    //             mint_y: todo!(),
    //             mint_lp: todo!(),
    //             pool_x_ata: todo!(),
    //             pool_y_ata: todo!(),
    //             mint_x_token_program: todo!(),
    //             mint_y_token_program: todo!(),
    //             token_program: todo!(),
    //             associated_token_program: todo!(),
    //             system_program: todo!(),
    //         }
    //         .to_account_metas(None);
    //         Ok((signers, acc_meta))
    //     }
    // }
    // impl<'info> IxOps<'info> for AddLiquidity {
    //     type IxData = anchor_basic_amm::instruction::AddLiquidity;
    //     type IxAccounts = FuzzAccounts;
    //     type IxSnapshot = AddLiquiditySnapshot<'info>;
    //     fn get_data(
    //         &self,
    //         _client: &mut impl FuzzClient,
    //         _fuzz_accounts: &mut FuzzAccounts,
    //     ) -> Result<Self::IxData, FuzzingError> {
    //         let data = anchor_basic_amm::instruction::AddLiquidity {
    //             amount_x: todo!(),
    //             amount_y: todo!(),
    //         };
    //         Ok(data)
    //     }
    //     fn get_accounts(
    //         &self,
    //         client: &mut impl FuzzClient,
    //         fuzz_accounts: &mut FuzzAccounts,
    //     ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
    //         let signers = vec![todo!()];
    //         let acc_meta = anchor_basic_amm::accounts::AddLiquidity {
    //             signer: todo!(),
    //             pool: todo!(),
    //             mint_x: todo!(),
    //             mint_y: todo!(),
    //             mint_lp: todo!(),
    //             user_x_ata: todo!(),
    //             user_y_ata: todo!(),
    //             pool_x_ata: todo!(),
    //             pool_y_ata: todo!(),
    //             user_lp_ata: todo!(),
    //             mint_x_token_program: todo!(),
    //             mint_y_token_program: todo!(),
    //             token_program: todo!(),
    //             associated_token_program: todo!(),
    //             system_program: todo!(),
    //         }
    //         .to_account_metas(None);
    //         Ok((signers, acc_meta))
    //     }
    // }
    // impl<'info> IxOps<'info> for Swap {
    //     type IxData = anchor_basic_amm::instruction::Swap;
    //     type IxAccounts = FuzzAccounts;
    //     type IxSnapshot = SwapSnapshot<'info>;
    //     fn get_data(
    //         &self,
    //         _client: &mut impl FuzzClient,
    //         _fuzz_accounts: &mut FuzzAccounts,
    //     ) -> Result<Self::IxData, FuzzingError> {
    //         let data = anchor_basic_amm::instruction::Swap {
    //             x_to_y: todo!(),
    //             amount_in: todo!(),
    //             minimum_amount_out: todo!(),
    //         };
    //         Ok(data)
    //     }
    //     fn get_accounts(
    //         &self,
    //         client: &mut impl FuzzClient,
    //         fuzz_accounts: &mut FuzzAccounts,
    //     ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
    //         let signers = vec![todo!()];
    //         let acc_meta = anchor_basic_amm::accounts::Swap {
    //             signer: todo!(),
    //             pool: todo!(),
    //             config: todo!(),
    //             mint_x: todo!(),
    //             mint_y: todo!(),
    //             user_x_ata: todo!(),
    //             user_y_ata: todo!(),
    //             pool_x_ata: todo!(),
    //             pool_y_ata: todo!(),
    //             mint_x_token_program: todo!(),
    //             mint_y_token_program: todo!(),
    //             token_program: todo!(),
    //             associated_token_program: todo!(),
    //             system_program: todo!(),
    //         }
    //         .to_account_metas(None);
    //         Ok((signers, acc_meta))
    //     }
    // }
    // impl<'info> IxOps<'info> for RemoveLiquidity {
    //     type IxData = anchor_basic_amm::instruction::RemoveLiquidity;
    //     type IxAccounts = FuzzAccounts;
    //     type IxSnapshot = RemoveLiquiditySnapshot<'info>;
    //     fn get_data(
    //         &self,
    //         _client: &mut impl FuzzClient,
    //         _fuzz_accounts: &mut FuzzAccounts,
    //     ) -> Result<Self::IxData, FuzzingError> {
    //         let data = anchor_basic_amm::instruction::RemoveLiquidity { shares: todo!() };
    //         Ok(data)
    //     }
    //     fn get_accounts(
    //         &self,
    //         client: &mut impl FuzzClient,
    //         fuzz_accounts: &mut FuzzAccounts,
    //     ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
    //         let signers = vec![todo!()];
    //         let acc_meta = anchor_basic_amm::accounts::RemoveLiquidity {
    //             signer: todo!(),
    //             pool: todo!(),
    //             mint_x: todo!(),
    //             mint_y: todo!(),
    //             mint_lp: todo!(),
    //             user_x_ata: todo!(),
    //             user_y_ata: todo!(),
    //             pool_x_ata: todo!(),
    //             pool_y_ata: todo!(),
    //             user_lp_ata: todo!(),
    //             mint_x_token_program: todo!(),
    //             mint_y_token_program: todo!(),
    //             token_program: todo!(),
    //             associated_token_program: todo!(),
    //             system_program: todo!(),
    //         }
    //         .to_account_metas(None);
    //         Ok((signers, acc_meta))
    //     }
    // }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        associated_token_program: AccountsStorage<ProgramStore>,
        config: AccountsStorage<PdaStore>,
        maker: AccountsStorage<Keypair>,
        mint_lp: AccountsStorage<MintStore>,
        mint_x: AccountsStorage<MintStore>,
        mint_x_token_program: AccountsStorage<TokenStore>,
        mint_y: AccountsStorage<MintStore>,
        mint_y_token_program: AccountsStorage<TokenStore>,
        pool: AccountsStorage<PdaStore>,
        pool_x_ata: AccountsStorage<TokenStore>,
        pool_y_ata: AccountsStorage<TokenStore>,
        signer: AccountsStorage<Keypair>,
        // system_program: AccountsStorage<ProgramStore>,
        // token_program: AccountsStorage<ProgramStore>,
        user_lp_ata: AccountsStorage<TokenStore>,
        user_x_ata: AccountsStorage<TokenStore>,
        user_y_ata: AccountsStorage<TokenStore>,
    }
}
