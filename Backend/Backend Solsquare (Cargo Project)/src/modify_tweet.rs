use borsh::BorshSerialize;
use solana_program::account_info::{AccountInfo, next_account_info};
use solana_program::borsh0_10::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::state::TweetDataMod;

pub(crate) fn modify_tweet(
    accounts: &[AccountInfo],
    data: TweetDataMod
) -> ProgramResult {

    let account_info_iter = &mut accounts.iter();
    let payer_account_info = next_account_info(account_info_iter)?;
    let pda_account_info = next_account_info(account_info_iter)?;
    // Get Tweet Data
    let mut tweet_data = try_from_slice_unchecked::<TweetDataMod>(&pda_account_info.data.borrow()).unwrap();

    // Security Check
    {
        if Pubkey::new_from_array(data.owner).ne(&payer_account_info.key) {
            return Err(ProgramError::IllegalOwner);
        }
    }

    msg!("Prev Data: {:?}", tweet_data);

    tweet_data.owner= data.owner;
    tweet_data.parentPost= data.parentPost;
    tweet_data.rudeness = data.rudeness;
    tweet_data.cid= data.cid;
    tweet_data.content = data.content;
    tweet_data.timestamp = data.timestamp;


    msg!("New Data: {:?}", tweet_data);
    tweet_data.serialize(&mut &mut pda_account_info.data.borrow_mut()[..])?;

    Ok(())
}
