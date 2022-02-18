use {
    crate::error::CustomError,
    solana_program::{
        borsh::{try_from_slice_unchecked},
        account_info::{next_account_info, AccountInfo},
        rent::Rent,
        entrypoint::ProgramResult,
        program_error::ProgramError,
        system_instruction,
        pubkey::Pubkey,
        program,
        program_pack::Pack,
        clock::Clock,
        sysvar::Sysvar,
    },
    spl_token::{
        state::{
            Account,
            Mint
        },
    },
};

pub fn castVote<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    vote: u64,
    vote_option: u64
) -> ProgramResult {
    let accounts_iter     = &mut accounts.iter();

    let payer_account     = next_account_info(accounts_iter)?; // 0
    let mint_account      = next_account_info(accounts_iter)?; // 1
    let token_account     = next_account_info(accounts_iter)?; // 2
    let metadata_account  = next_account_info(accounts_iter)?; // 3
    let vote_account      = next_account_info(accounts_iter)?; // 4
    let sys_account       = next_account_info(accounts_iter)?; // 5
    let vote_info_account = next_account_info(accounts_iter)?; // 6

    let token_data: Account = Pack::unpack(&token_account.data.borrow())?;
    let mint_data: Mint     = Pack::unpack(&mint_account.data.borrow())?;

    let meta_program_key = &Pubkey::new_from_array([11, 112, 101, 177, 227, 209, 124,  69,
                                                    56, 157,  82, 127, 107,   4, 195, 205,
                                                    88, 184, 108, 115,  26, 160, 253, 181,
                                                    73, 182, 209, 188,   3, 248,   41, 70]);

    let meta_seeds = &[
        b"metadata",
        meta_program_key.as_ref(),
        mint_account.key.as_ref()
    ];

    let (valid_meta_key, _seed) = Pubkey::find_program_address(meta_seeds, meta_program_key);

    if valid_meta_key              != *metadata_account.key { return Err(CustomError::InvalidMint.into());  }
    if token_data.amount           != 1                     { return Err(CustomError::InvalidMint.into());  }
    if mint_data.supply            != 1                     { return Err(CustomError::InvalidMint.into());  }
    if token_data.mint             != *mint_account.key     { return Err(CustomError::InvalidMint.into());  }
    if token_data.owner            != *payer_account.key    { return Err(CustomError::InvalidMint.into());  }
    if !payer_account.is_signer                             { return Err(ProgramError::InvalidAccountData); }

    let md: metaplex_token_metadata::state::Metadata = try_from_slice_unchecked(&metadata_account.data.borrow())?;

    if md.mint != *mint_account.key { return Err(CustomError::InvalidMint.into()); }

    let creators      = md.data.creators.unwrap();
    let first_creator = &creators[0];

    if first_creator.verified == false { return Err(CustomError::AuthKeyFailure.into()); }

    let auth_seeds = &[
        mint_account.key.as_ref(),
        &vote.to_le_bytes()
    ];

    let (auth_key, bump_seed) = Pubkey::find_program_address(auth_seeds, program_id);
    let authority_seeds: &[&[_]] = &[
        mint_account.key.as_ref(),
        &vote.to_le_bytes(),
        &[bump_seed]
    ];

    if auth_key != *vote_account.key { return Err(CustomError::AuthKeyFailure.into()); }

    let vote_info_seeds = &[
        first_creator.address.as_ref(),
        &vote.to_le_bytes()
    ];

    let (vote_info_key, _bump) = Pubkey::find_program_address(vote_info_seeds, program_id);

    if vote_info_key != *vote_info_account.key { return Err(CustomError::AuthKeyFailure.into()); }
    if vote_info_account.data.borrow().len() == 0 { return Err(CustomError::VoteDoesntExist.into()); }

    if vote_account.data.borrow().len() == 0 {
        program::invoke_signed(
            &system_instruction::create_account(
                payer_account.key,
                vote_account.key,
                Rent::get().unwrap().minimum_balance(116),
                116,
                program_id
            ),
            &[
                payer_account.clone(),
                vote_account.clone(),
                sys_account.clone()
            ],
            &[authority_seeds]
        )?;
    } else {
        // vote already cast.
        return Err(CustomError::VoteCastAlready.into()); 
    }

    let mut auth_data = vote_account.data.borrow_mut();

    let clock: Clock = Sysvar::get()?;
    let unix_time = clock.unix_timestamp;

    auth_data[..32].copy_from_slice(&mint_account.key.to_bytes());
    auth_data[32..64].copy_from_slice(&first_creator.address.to_bytes());
    auth_data[64..96].copy_from_slice(&payer_account.key.to_bytes());
    auth_data[96..104].copy_from_slice(&vote.to_le_bytes());
    auth_data[104..108].copy_from_slice(&unix_time.to_le_bytes()[..4]);
    auth_data[108..116].copy_from_slice(&vote_option.to_le_bytes());


    Ok(())
}

pub fn createVote<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    vote: u64,
    uri: &[u8]
) -> ProgramResult {
    let accounts_iter     = &mut accounts.iter();

    let payer_account     = next_account_info(accounts_iter)?; // 0
    let mint_account      = next_account_info(accounts_iter)?; // 1
    let token_account     = next_account_info(accounts_iter)?; // 2
    let metadata_account  = next_account_info(accounts_iter)?; // 3
    let vote_account      = next_account_info(accounts_iter)?; // 4
    let sys_account       = next_account_info(accounts_iter)?; // 5

    let token_data: Account = Pack::unpack(&token_account.data.borrow())?;
    let mint_data: Mint     = Pack::unpack(&mint_account.data.borrow())?;

    let meta_program_key = &Pubkey::new_from_array([11, 112, 101, 177, 227, 209, 124,  69,
                                                    56, 157,  82, 127, 107,   4, 195, 205,
                                                    88, 184, 108, 115,  26, 160, 253, 181,
                                                    73, 182, 209, 188,   3, 248,   41, 70]);

    let meta_seeds = &[
        b"metadata",
        meta_program_key.as_ref(),
        mint_account.key.as_ref()
    ];

    let (valid_meta_key, _seed) = Pubkey::find_program_address(meta_seeds, meta_program_key);

    if valid_meta_key              != *metadata_account.key { return Err(CustomError::InvalidMint.into());  }
    if token_data.amount           != 1                     { return Err(CustomError::InvalidMint.into());  }
    if mint_data.supply            != 1                     { return Err(CustomError::InvalidMint.into());  }
    if token_data.mint             != *mint_account.key     { return Err(CustomError::InvalidMint.into());  }
    if token_data.owner            != *payer_account.key    { return Err(CustomError::InvalidMint.into());  }
    if !payer_account.is_signer                             { return Err(ProgramError::InvalidAccountData); }

    let md: metaplex_token_metadata::state::Metadata = try_from_slice_unchecked(&metadata_account.data.borrow())?;

    if md.mint != *mint_account.key { return Err(CustomError::InvalidMint.into()); }

    let creators      = md.data.creators.unwrap();
    let first_creator = &creators[0];

    // nft is verified to belong in a set
    if first_creator.verified == false { return Err(CustomError::AuthKeyFailure.into()); }

    let auth_seeds = &[
        first_creator.address.as_ref(),
        &vote.to_le_bytes()
    ];

    let (auth_key, bump_seed) = Pubkey::find_program_address(auth_seeds, program_id);
    let authority_seeds: &[&[_]] = &[
        first_creator.address.as_ref(),
        &vote.to_le_bytes(),
        &[bump_seed]
    ];

    if auth_key != *vote_account.key { return Err(CustomError::AuthKeyFailure.into()); }

    if vote_account.data.borrow().len() == 0 {
        program::invoke_signed(
            &system_instruction::create_account(
                payer_account.key,
                vote_account.key,
                Rent::get().unwrap().minimum_balance(148),
                148,
                program_id
            ),
            &[
                payer_account.clone(),
                vote_account.clone(),
                sys_account.clone()
            ],
            &[authority_seeds]
        )?;
    } else {
        // vote already created.
        return Err(CustomError::VoteCreatedAlready.into()); 
    }

    let mut auth_data = vote_account.data.borrow_mut();

    if uri.len() >= 100 {
        return Err(CustomError::AuthKeyFailure.into());
    }

    let clock: Clock = Sysvar::get()?;
    let unix_time = clock.unix_timestamp;

    auth_data[..uri.len()].copy_from_slice(uri);
    auth_data[100..108].copy_from_slice(&vote.to_le_bytes());
    auth_data[112..116].copy_from_slice(&unix_time.to_le_bytes()[..4]);
    auth_data[116..148].copy_from_slice(&first_creator.address.to_bytes());

    Ok(())
}
