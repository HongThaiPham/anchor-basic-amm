use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::Token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use trident_client::fuzzing::{anchor_lang, FuzzingError};
pub struct InitializeSnapshot<'info> {
    pub signer: Signer<'info>,
    pub config: Option<Account<'info, anchor_basic_amm::state::config::AmmConfig>>,
    pub system_program: Program<'info, System>,
}
pub struct CreatePoolSnapshot<'info> {
    pub maker: Signer<'info>,
    pub pool: Option<Account<'info, anchor_basic_amm::state::pool::Pool>>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    pub mint_lp: Option<InterfaceAccount<'info, Mint>>,
    pub pool_x_ata: Option<InterfaceAccount<'info, TokenAccount>>,
    pub pool_y_ata: Option<InterfaceAccount<'info, TokenAccount>>,
    pub mint_x_token_program: Interface<'info, TokenInterface>,
    pub mint_y_token_program: Interface<'info, TokenInterface>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
pub struct AddLiquiditySnapshot<'info> {
    pub signer: Signer<'info>,
    pub pool: Account<'info, anchor_basic_amm::state::pool::Pool>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    pub mint_lp: InterfaceAccount<'info, Mint>,
    pub user_x_ata: InterfaceAccount<'info, TokenAccount>,
    pub user_y_ata: InterfaceAccount<'info, TokenAccount>,
    pub pool_x_ata: InterfaceAccount<'info, TokenAccount>,
    pub pool_y_ata: InterfaceAccount<'info, TokenAccount>,
    pub user_lp_ata: Option<InterfaceAccount<'info, TokenAccount>>,
    pub mint_x_token_program: Interface<'info, TokenInterface>,
    pub mint_y_token_program: Interface<'info, TokenInterface>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
pub struct SwapSnapshot<'info> {
    pub signer: Signer<'info>,
    pub pool: Account<'info, anchor_basic_amm::state::pool::Pool>,
    pub config: Account<'info, anchor_basic_amm::state::config::AmmConfig>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    pub user_x_ata: Option<InterfaceAccount<'info, TokenAccount>>,
    pub user_y_ata: Option<InterfaceAccount<'info, TokenAccount>>,
    pub pool_x_ata: InterfaceAccount<'info, TokenAccount>,
    pub pool_y_ata: InterfaceAccount<'info, TokenAccount>,
    pub mint_x_token_program: Interface<'info, TokenInterface>,
    pub mint_y_token_program: Interface<'info, TokenInterface>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
pub struct RemoveLiquiditySnapshot<'info> {
    pub signer: Signer<'info>,
    pub pool: Account<'info, anchor_basic_amm::state::pool::Pool>,
    pub mint_x: InterfaceAccount<'info, Mint>,
    pub mint_y: InterfaceAccount<'info, Mint>,
    pub mint_lp: InterfaceAccount<'info, Mint>,
    pub user_x_ata: InterfaceAccount<'info, TokenAccount>,
    pub user_y_ata: InterfaceAccount<'info, TokenAccount>,
    pub pool_x_ata: InterfaceAccount<'info, TokenAccount>,
    pub pool_y_ata: InterfaceAccount<'info, TokenAccount>,
    pub user_lp_ata: Option<InterfaceAccount<'info, TokenAccount>>,
    pub mint_x_token_program: Interface<'info, TokenInterface>,
    pub mint_y_token_program: Interface<'info, TokenInterface>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitializeSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let config: Option<
            anchor_lang::accounts::account::Account<anchor_basic_amm::state::config::AmmConfig>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("config".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("config".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "config".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            signer,
            config,
            system_program,
        })
    }
}
impl<'info> CreatePoolSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let maker: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("maker".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("maker".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("maker".to_string()))?;
        let pool: Option<
            anchor_lang::accounts::account::Account<anchor_basic_amm::state::pool::Pool>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pool".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::account::Account::try_from(acc)
                        .map_err(|_| FuzzingError::CannotDeserializeAccount("pool".to_string()))
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided("pool".to_string()))
                }
            })
            .transpose()
            .unwrap_or(None);
        let mint_x: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint_x".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("mint_x".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("mint_x".to_string()))?;
        let mint_y: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint_y".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("mint_y".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("mint_y".to_string()))?;
        let mint_lp: Option<anchor_lang::accounts::interface_account::InterfaceAccount<Mint>> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint_lp".to_string()))?
                .as_ref()
                .map(|acc| {
                    if acc.key() != *_program_id {
                        anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                            .map_err(|_| {
                                FuzzingError::CannotDeserializeAccount("mint_lp".to_string())
                            })
                    } else {
                        Err(FuzzingError::OptionalAccountNotProvided(
                            "mint_lp".to_string(),
                        ))
                    }
                })
                .transpose()
                .unwrap_or(None);
        let pool_x_ata: Option<
            anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pool_x_ata".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("pool_x_ata".to_string())
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "pool_x_ata".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let pool_y_ata: Option<
            anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("pool_y_ata".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("pool_y_ata".to_string())
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "pool_y_ata".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let mint_x_token_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "mint_x_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "mint_x_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("mint_x_token_program".to_string())
                })?;
        let mint_y_token_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "mint_y_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "mint_y_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("mint_y_token_program".to_string())
                })?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let associated_token_program: anchor_lang::accounts::program::Program<AssociatedToken> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "associated_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::program::Program::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "associated_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("associated_token_program".to_string())
                })?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            maker,
            pool,
            mint_x,
            mint_y,
            mint_lp,
            pool_x_ata,
            pool_y_ata,
            mint_x_token_program,
            mint_y_token_program,
            token_program,
            associated_token_program,
            system_program,
        })
    }
}
impl<'info> AddLiquiditySnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let pool: anchor_lang::accounts::account::Account<anchor_basic_amm::state::pool::Pool> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::account::Account::try_from)
                .ok_or(FuzzingError::AccountNotFound("pool".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("pool".to_string()))?;
        let mint_x: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint_x".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("mint_x".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("mint_x".to_string()))?;
        let mint_y: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint_y".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("mint_y".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("mint_y".to_string()))?;
        let mint_lp: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint_lp".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("mint_lp".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("mint_lp".to_string()))?;
        let user_x_ata: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("user_x_ata".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("user_x_ata".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("user_x_ata".to_string()))?;
        let user_y_ata: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("user_y_ata".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("user_y_ata".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("user_y_ata".to_string()))?;
        let pool_x_ata: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool_x_ata".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("pool_x_ata".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_x_ata".to_string()))?;
        let pool_y_ata: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool_y_ata".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("pool_y_ata".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_y_ata".to_string()))?;
        let user_lp_ata: Option<
            anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user_lp_ata".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("user_lp_ata".to_string())
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "user_lp_ata".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let mint_x_token_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "mint_x_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "mint_x_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("mint_x_token_program".to_string())
                })?;
        let mint_y_token_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "mint_y_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "mint_y_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("mint_y_token_program".to_string())
                })?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let associated_token_program: anchor_lang::accounts::program::Program<AssociatedToken> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "associated_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::program::Program::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "associated_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("associated_token_program".to_string())
                })?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            signer,
            pool,
            mint_x,
            mint_y,
            mint_lp,
            user_x_ata,
            user_y_ata,
            pool_x_ata,
            pool_y_ata,
            user_lp_ata,
            mint_x_token_program,
            mint_y_token_program,
            token_program,
            associated_token_program,
            system_program,
        })
    }
}
impl<'info> SwapSnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let pool: anchor_lang::accounts::account::Account<anchor_basic_amm::state::pool::Pool> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::account::Account::try_from)
                .ok_or(FuzzingError::AccountNotFound("pool".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("pool".to_string()))?;
        let config: anchor_lang::accounts::account::Account<
            anchor_basic_amm::state::config::AmmConfig,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("config".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::account::Account::try_from)
            .ok_or(FuzzingError::AccountNotFound("config".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("config".to_string()))?;
        let mint_x: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint_x".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("mint_x".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("mint_x".to_string()))?;
        let mint_y: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint_y".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("mint_y".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("mint_y".to_string()))?;
        let user_x_ata: Option<
            anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user_x_ata".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("user_x_ata".to_string())
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "user_x_ata".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let user_y_ata: Option<
            anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user_y_ata".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("user_y_ata".to_string())
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "user_y_ata".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let pool_x_ata: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool_x_ata".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("pool_x_ata".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_x_ata".to_string()))?;
        let pool_y_ata: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool_y_ata".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("pool_y_ata".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_y_ata".to_string()))?;
        let mint_x_token_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "mint_x_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "mint_x_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("mint_x_token_program".to_string())
                })?;
        let mint_y_token_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "mint_y_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "mint_y_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("mint_y_token_program".to_string())
                })?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let associated_token_program: anchor_lang::accounts::program::Program<AssociatedToken> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "associated_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::program::Program::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "associated_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("associated_token_program".to_string())
                })?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            signer,
            pool,
            config,
            mint_x,
            mint_y,
            user_x_ata,
            user_y_ata,
            pool_x_ata,
            pool_y_ata,
            mint_x_token_program,
            mint_y_token_program,
            token_program,
            associated_token_program,
            system_program,
        })
    }
}
impl<'info> RemoveLiquiditySnapshot<'info> {
    pub fn deserialize_option(
        _program_id: &anchor_lang::prelude::Pubkey,
        accounts: &'info mut [Option<AccountInfo<'info>>],
    ) -> core::result::Result<Self, FuzzingError> {
        let mut accounts_iter = accounts.iter();
        let signer: Signer<'_> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("signer".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::signer::Signer::try_from)
            .ok_or(FuzzingError::AccountNotFound("signer".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("signer".to_string()))?;
        let pool: anchor_lang::accounts::account::Account<anchor_basic_amm::state::pool::Pool> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::account::Account::try_from)
                .ok_or(FuzzingError::AccountNotFound("pool".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("pool".to_string()))?;
        let mint_x: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint_x".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("mint_x".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("mint_x".to_string()))?;
        let mint_y: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint_y".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("mint_y".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("mint_y".to_string()))?;
        let mint_lp: anchor_lang::accounts::interface_account::InterfaceAccount<Mint> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("mint_lp".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("mint_lp".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("mint_lp".to_string()))?;
        let user_x_ata: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("user_x_ata".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("user_x_ata".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("user_x_ata".to_string()))?;
        let user_y_ata: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("user_y_ata".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("user_y_ata".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("user_y_ata".to_string()))?;
        let pool_x_ata: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool_x_ata".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("pool_x_ata".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_x_ata".to_string()))?;
        let pool_y_ata: anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts("pool_y_ata".to_string()))?
                .as_ref()
                .map(anchor_lang::accounts::interface_account::InterfaceAccount::try_from)
                .ok_or(FuzzingError::AccountNotFound("pool_y_ata".to_string()))?
                .map_err(|_| FuzzingError::CannotDeserializeAccount("pool_y_ata".to_string()))?;
        let user_lp_ata: Option<
            anchor_lang::accounts::interface_account::InterfaceAccount<TokenAccount>,
        > = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("user_lp_ata".to_string()))?
            .as_ref()
            .map(|acc| {
                if acc.key() != *_program_id {
                    anchor_lang::accounts::interface_account::InterfaceAccount::try_from(acc)
                        .map_err(|_| {
                            FuzzingError::CannotDeserializeAccount("user_lp_ata".to_string())
                        })
                } else {
                    Err(FuzzingError::OptionalAccountNotProvided(
                        "user_lp_ata".to_string(),
                    ))
                }
            })
            .transpose()
            .unwrap_or(None);
        let mint_x_token_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "mint_x_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "mint_x_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("mint_x_token_program".to_string())
                })?;
        let mint_y_token_program: anchor_lang::accounts::interface::Interface<TokenInterface> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "mint_y_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::interface::Interface::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "mint_y_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("mint_y_token_program".to_string())
                })?;
        let token_program: anchor_lang::accounts::program::Program<Token> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts("token_program".to_string()))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("token_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("token_program".to_string()))?;
        let associated_token_program: anchor_lang::accounts::program::Program<AssociatedToken> =
            accounts_iter
                .next()
                .ok_or(FuzzingError::NotEnoughAccounts(
                    "associated_token_program".to_string(),
                ))?
                .as_ref()
                .map(anchor_lang::accounts::program::Program::try_from)
                .ok_or(FuzzingError::AccountNotFound(
                    "associated_token_program".to_string(),
                ))?
                .map_err(|_| {
                    FuzzingError::CannotDeserializeAccount("associated_token_program".to_string())
                })?;
        let system_program: anchor_lang::accounts::program::Program<System> = accounts_iter
            .next()
            .ok_or(FuzzingError::NotEnoughAccounts(
                "system_program".to_string(),
            ))?
            .as_ref()
            .map(anchor_lang::accounts::program::Program::try_from)
            .ok_or(FuzzingError::AccountNotFound("system_program".to_string()))?
            .map_err(|_| FuzzingError::CannotDeserializeAccount("system_program".to_string()))?;
        Ok(Self {
            signer,
            pool,
            mint_x,
            mint_y,
            mint_lp,
            user_x_ata,
            user_y_ata,
            pool_x_ata,
            pool_y_ata,
            user_lp_ata,
            mint_x_token_program,
            mint_y_token_program,
            token_program,
            associated_token_program,
            system_program,
        })
    }
}
