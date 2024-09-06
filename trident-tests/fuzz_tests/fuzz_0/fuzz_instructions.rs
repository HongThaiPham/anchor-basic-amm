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
        #[arbitrary(with = |u: &mut arbitrary::Unstructured| u.int_in_range(10..=10000))]
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
                .get_or_create_account(self.accounts.config, &[b"ammconfig"], &anchor_basic_amm::ID)
                .ok_or(FuzzingError::Custom(3))?
                .pubkey();
            let signers = vec![signer.clone()];
            let acc_meta = anchor_basic_amm::accounts::Initialize {
                signer: signer.pubkey(),
                config: config,
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }

        fn check(
            &self,
            _pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            let config = post_ix.config;
            if config.unwrap().fee != ix_data.fee {
                return Err(FuzzingError::Custom(1));
            }
            Ok(())
        }
    }
    impl<'info> IxOps<'info> for CreatePool {
        type IxData = anchor_basic_amm::instruction::CreatePool;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = CreatePoolSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = anchor_basic_amm::instruction::CreatePool {};
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let maker = fuzz_accounts.maker.get_or_create_account(
                self.accounts.maker,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let signers = vec![maker.clone()];
            let mint_x = fuzz_accounts
                .mint_x
                .get_or_create_account(self.accounts.mint_x, client, 9, &maker.pubkey(), None)
                .unwrap();

            let mint_y = fuzz_accounts
                .mint_y
                .get_or_create_account(self.accounts.mint_y, client, 9, &maker.pubkey(), None)
                .unwrap();

            let pool = fuzz_accounts
                .pool
                .get_or_create_account(
                    self.accounts.pool,
                    &[b"pool", mint_x.as_ref(), mint_y.as_ref()],
                    &anchor_basic_amm::ID,
                )
                .ok_or(FuzzingError::Custom(3))?
                .pubkey();

            let mint_lp = fuzz_accounts
                .mint_lp
                .get_or_create_account(
                    self.accounts.mint_lp,
                    &[b"lp", pool.as_ref()],
                    &anchor_basic_amm::ID,
                )
                .unwrap()
                .pubkey();

            // let mint_lp = fuzz_accounts
            //     .mint_lp
            //     .get_or_create_account(self.accounts.mint_lp, client, 6, &pool, None)
            //     .unwrap();

            let pool_x_ata = fuzz_accounts
                .pool_x_ata
                .get_or_create_account(
                    self.accounts.pool_x_ata,
                    client,
                    mint_x,
                    maker.pubkey(),
                    10 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let pool_y_ata = fuzz_accounts
                .pool_y_ata
                .get_or_create_account(
                    self.accounts.pool_y_ata,
                    client,
                    mint_y,
                    maker.pubkey(),
                    10 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();
            let acc_meta = anchor_basic_amm::accounts::CreatePool {
                maker: maker.pubkey(),
                pool,
                mint_x,
                mint_y,
                mint_lp: mint_lp,
                pool_x_ata,
                pool_y_ata,
                mint_x_token_program: anchor_spl::token_2022::ID,
                mint_y_token_program: anchor_spl::token_2022::ID,
                token_program: anchor_spl::token::ID,
                associated_token_program: anchor_spl::associated_token::ID,
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    impl<'info> IxOps<'info> for AddLiquidity {
        type IxData = anchor_basic_amm::instruction::AddLiquidity;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = AddLiquiditySnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = anchor_basic_amm::instruction::AddLiquidity {
                amount_x: self.data.amount_x,
                amount_y: self.data.amount_y,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.maker.get_or_create_account(
                self.accounts.signer,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let signers = vec![signer.clone()];
            let mint_x = fuzz_accounts
                .mint_x
                .get_or_create_account(self.accounts.mint_x, client, 9, &signer.pubkey(), None)
                .unwrap();

            let mint_y = fuzz_accounts
                .mint_y
                .get_or_create_account(self.accounts.mint_y, client, 9, &signer.pubkey(), None)
                .unwrap();

            let pool = fuzz_accounts
                .pool
                .get_or_create_account(
                    self.accounts.pool,
                    &[b"pool", mint_x.as_ref(), mint_y.as_ref()],
                    &anchor_basic_amm::ID,
                )
                .ok_or(FuzzingError::Custom(3))?
                .pubkey();

            let mint_lp = fuzz_accounts
                .mint_lp
                .get_or_create_account(
                    self.accounts.mint_lp,
                    &[b"lp", pool.as_ref()],
                    &anchor_basic_amm::ID,
                )
                .unwrap()
                .pubkey();

            // let mint_lp = fuzz_accounts
            //     .mint_lp
            //     .get_or_create_account(self.accounts.mint_lp, client, 6, &pool, None)
            //     .unwrap();

            let pool_x_ata = fuzz_accounts
                .pool_x_ata
                .get_or_create_account(
                    self.accounts.pool_x_ata,
                    client,
                    mint_x,
                    signer.pubkey(),
                    10 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let pool_y_ata = fuzz_accounts
                .pool_y_ata
                .get_or_create_account(
                    self.accounts.pool_y_ata,
                    client,
                    mint_y,
                    signer.pubkey(),
                    10 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let user_x_ata = fuzz_accounts
                .user_x_ata
                .get_or_create_account(
                    self.accounts.user_x_ata,
                    client,
                    mint_x,
                    signer.pubkey(),
                    1000000 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let user_y_ata = fuzz_accounts
                .user_y_ata
                .get_or_create_account(
                    self.accounts.user_y_ata,
                    client,
                    mint_y,
                    signer.pubkey(),
                    1000000 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let user_lp_ata = fuzz_accounts
                .user_lp_ata
                .get_or_create_account(
                    self.accounts.user_lp_ata,
                    client,
                    mint_lp,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let acc_meta = anchor_basic_amm::accounts::AddLiquidity {
                signer: signer.pubkey(),
                pool,
                mint_x,
                mint_y,
                mint_lp,
                user_x_ata,
                user_y_ata,
                pool_x_ata,
                pool_y_ata,
                user_lp_ata,
                mint_x_token_program: anchor_spl::token_2022::ID,
                mint_y_token_program: anchor_spl::token_2022::ID,
                token_program: anchor_spl::token::ID,
                associated_token_program: anchor_spl::associated_token::ID,
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }

        fn check(
            &self,
            _pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            _ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            if post_ix.pool_x_ata.amount.gt(&0) {
                return Err(FuzzingError::Custom(3));
            };
            Ok(())
        }
    }
    impl<'info> IxOps<'info> for Swap {
        type IxData = anchor_basic_amm::instruction::Swap;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = SwapSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = anchor_basic_amm::instruction::Swap {
                x_to_y: self.data.x_to_y,
                amount_in: self.data.amount_in,
                minimum_amount_out: self.data.minimum_amount_out,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.maker.get_or_create_account(
                self.accounts.signer,
                client,
                10 * LAMPORTS_PER_SOL,
            );
            let signers = vec![signer.clone()];
            let mint_x = fuzz_accounts
                .mint_x
                .get_or_create_account(self.accounts.mint_x, client, 9, &signer.pubkey(), None)
                .unwrap();

            let mint_y = fuzz_accounts
                .mint_y
                .get_or_create_account(self.accounts.mint_y, client, 9, &signer.pubkey(), None)
                .unwrap();

            let pool = fuzz_accounts
                .pool
                .get_or_create_account(
                    self.accounts.pool,
                    &[b"pool", mint_x.as_ref(), mint_y.as_ref()],
                    &anchor_basic_amm::ID,
                )
                .ok_or(FuzzingError::Custom(3))?
                .pubkey();

            let pool_x_ata = fuzz_accounts
                .pool_x_ata
                .get_or_create_account(
                    self.accounts.pool_x_ata,
                    client,
                    mint_x,
                    signer.pubkey(),
                    10 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let pool_y_ata = fuzz_accounts
                .pool_y_ata
                .get_or_create_account(
                    self.accounts.pool_y_ata,
                    client,
                    mint_y,
                    signer.pubkey(),
                    10 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let user_x_ata = fuzz_accounts
                .user_x_ata
                .get_or_create_account(
                    self.accounts.user_x_ata,
                    client,
                    mint_x,
                    signer.pubkey(),
                    1000000 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let user_y_ata = fuzz_accounts
                .user_y_ata
                .get_or_create_account(
                    self.accounts.user_y_ata,
                    client,
                    mint_y,
                    signer.pubkey(),
                    1000000 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();
            let config = fuzz_accounts
                .config
                .get_or_create_account(self.accounts.config, &[b"ammconfig"], &anchor_basic_amm::ID)
                .ok_or(FuzzingError::Custom(3))?
                .pubkey();
            let acc_meta = anchor_basic_amm::accounts::Swap {
                signer: signer.pubkey(),
                pool,
                config,
                mint_x,
                mint_y,
                user_x_ata,
                user_y_ata,
                pool_x_ata,
                pool_y_ata,
                mint_x_token_program: anchor_spl::token_2022::ID,
                mint_y_token_program: anchor_spl::token_2022::ID,
                token_program: anchor_spl::token::ID,
                associated_token_program: anchor_spl::associated_token::ID,
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }

        fn check(
            &self,
            pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            if ix_data.x_to_y {
                if post_ix.pool_x_ata.amount.gt(&pre_ix.pool_x_ata.amount)
                    && post_ix.pool_y_ata.amount.lt(&pre_ix.pool_y_ata.amount)
                {
                    return Err(FuzzingError::Custom(3));
                }

                if post_ix
                    .user_x_ata
                    .unwrap()
                    .amount
                    .lt(&pre_ix.user_x_ata.unwrap().amount)
                    && post_ix
                        .user_y_ata
                        .unwrap()
                        .amount
                        .gt(&pre_ix.user_y_ata.unwrap().amount)
                {
                    return Err(FuzzingError::Custom(4));
                }
            } else {
                if post_ix.pool_y_ata.amount.gt(&pre_ix.pool_y_ata.amount)
                    && post_ix.pool_x_ata.amount.lt(&pre_ix.pool_x_ata.amount)
                {
                    return Err(FuzzingError::Custom(3));
                }

                if post_ix
                    .user_y_ata
                    .unwrap()
                    .amount
                    .lt(&pre_ix.user_y_ata.unwrap().amount)
                    && post_ix
                        .user_x_ata
                        .unwrap()
                        .amount
                        .gt(&pre_ix.user_x_ata.unwrap().amount)
                {
                    return Err(FuzzingError::Custom(4));
                }
            }

            Ok(())
        }
    }
    impl<'info> IxOps<'info> for RemoveLiquidity {
        type IxData = anchor_basic_amm::instruction::RemoveLiquidity;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = RemoveLiquiditySnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = anchor_basic_amm::instruction::RemoveLiquidity {
                shares: self.data.shares,
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signer = fuzz_accounts.maker.get_or_create_account(
                self.accounts.signer,
                client,
                10 * LAMPORTS_PER_SOL,
            );

            let signers = vec![signer.clone()];
            let mint_x = fuzz_accounts
                .mint_x
                .get_or_create_account(self.accounts.mint_x, client, 9, &signer.pubkey(), None)
                .unwrap();

            let mint_y = fuzz_accounts
                .mint_y
                .get_or_create_account(self.accounts.mint_y, client, 9, &signer.pubkey(), None)
                .unwrap();

            let pool = fuzz_accounts
                .pool
                .get_or_create_account(
                    self.accounts.pool,
                    &[b"pool", mint_x.as_ref(), mint_y.as_ref()],
                    &anchor_basic_amm::ID,
                )
                .ok_or(FuzzingError::Custom(3))?
                .pubkey();

            let mint_lp = fuzz_accounts
                .mint_lp
                .get_or_create_account(
                    self.accounts.mint_lp,
                    &[b"lp", pool.as_ref()],
                    &anchor_basic_amm::ID,
                )
                .unwrap()
                .pubkey();

            // let mint_lp = fuzz_accounts
            //     .mint_lp
            //     .get_or_create_account(self.accounts.mint_lp, client, 6, &pool, None)
            //     .unwrap();

            let pool_x_ata = fuzz_accounts
                .pool_x_ata
                .get_or_create_account(
                    self.accounts.pool_x_ata,
                    client,
                    mint_x,
                    signer.pubkey(),
                    10 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let pool_y_ata = fuzz_accounts
                .pool_y_ata
                .get_or_create_account(
                    self.accounts.pool_y_ata,
                    client,
                    mint_y,
                    signer.pubkey(),
                    10 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let user_x_ata = fuzz_accounts
                .user_x_ata
                .get_or_create_account(
                    self.accounts.user_x_ata,
                    client,
                    mint_x,
                    signer.pubkey(),
                    1000000 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let user_y_ata = fuzz_accounts
                .user_y_ata
                .get_or_create_account(
                    self.accounts.user_y_ata,
                    client,
                    mint_y,
                    signer.pubkey(),
                    1000000 * LAMPORTS_PER_SOL,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();

            let user_lp_ata = fuzz_accounts
                .user_lp_ata
                .get_or_create_account(
                    self.accounts.user_lp_ata,
                    client,
                    mint_lp,
                    signer.pubkey(),
                    0,
                    None,
                    None,
                    0,
                    None,
                )
                .unwrap();
            let acc_meta = anchor_basic_amm::accounts::RemoveLiquidity {
                signer: signer.pubkey(),
                pool,
                mint_x,
                mint_y,
                mint_lp,
                user_x_ata,
                user_y_ata,
                pool_x_ata,
                pool_y_ata,
                user_lp_ata,
                mint_x_token_program: anchor_spl::token_2022::ID,
                mint_y_token_program: anchor_spl::token_2022::ID,
                token_program: anchor_spl::token::ID,
                associated_token_program: anchor_spl::associated_token::ID,
                system_program: solana_sdk::system_program::ID,
            }
            .to_account_metas(None);
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        // associated_token_program: AccountsStorage<ProgramStore>,
        config: AccountsStorage<PdaStore>,
        maker: AccountsStorage<Keypair>,
        mint_lp: AccountsStorage<PdaStore>,
        mint_x: AccountsStorage<MintStore>,
        // mint_x_token_program: AccountsStorage<TokenStore>,
        mint_y: AccountsStorage<MintStore>,
        // mint_y_token_program: AccountsStorage<TokenStore>,
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
