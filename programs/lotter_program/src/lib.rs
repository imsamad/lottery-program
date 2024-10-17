use anchor_lang::prelude::*;

use solana_program::system_instruction;

mod constants;
mod error;

use constants::*;
use error::*;

declare_id!("GjqE189KCVNm2WQ9BC51rkikr7tiCTfosG3g4e9nJzn1");

#[program]
pub mod lotter_program {
    use super::*;

    pub fn initialize(ctx: Context<InitMaster>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_lottery(ctx: Context<CreateLotteryCtx>, ticket_price: u64) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;

        if lottery.winner_id.is_some() {
            return err!(LotteryError::WinnerAlreadyExists);
        }

        let master = &mut ctx.accounts.master;

        master.last_id += 1;

        lottery.id = master.last_id;
        lottery.authority = ctx.accounts.authority.key();

        lottery.ticket_price = ticket_price;
        msg!("created lottery!!! {lottery.id}");

        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicketCtx>, lottery_id: u32) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;

        let ticket = &mut ctx.accounts.ticket;

        let buyer = &ctx.accounts.buyer;

        let transfer_instruction =
            system_instruction::transfer(&buyer.key(), &lottery.key(), lottery.ticket_price);

        // transfer sol to lottery pda from buyer acc
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                buyer.to_account_info(),
                lottery.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        lottery.last_ticket_id += 1;

        ticket.id = lottery.last_ticket_id;

        ticket.lottery_id = lottery_id;

        ticket.authority = buyer.key();

        msg!("ticket created");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitMaster<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 4,
        seeds = [__program_id.as_ref(),MASTER_SEED.as_bytes()],
        bump
    )]
    pub master: Account<'info, Master>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Master {
    pub last_id: u32,
}

#[derive(Accounts)]
pub struct CreateLotteryCtx<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 62,
        seeds = [master.key().as_ref(),LOTTERY_SEED.as_bytes(), &(master.last_id + 1).to_le_bytes()],
        bump,
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [__program_id.as_ref(),MASTER_SEED.as_bytes()],
        bump,
    )]
    pub master: Account<'info, Master>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Lottery {
    pub authority: Pubkey,

    pub id: u32,
    pub ticket_price: u64,
    pub last_ticket_id: u32,
    pub winner_id: Option<u32>,
    pub claimed: bool,
}

#[derive(Accounts)]
#[instruction(lottery_id:u32)]
pub struct BuyTicketCtx<'info> {
    #[account(
        seeds = [__program_id.as_ref(), MASTER_SEED.as_bytes()],
        bump,
    )]
    pub master: Account<'info, Master>, // mut not needed if master is not modified

    #[account(
        mut,
        seeds = [master.key().as_ref(), LOTTERY_SEED.as_bytes(), &(master.last_id + 1).to_le_bytes()],
        bump
    )]
    pub lottery: Account<'info, Lottery>,

    #[account(
        init,
        payer = buyer,
        space = 4 + 4 + 32 + 8,
        seeds = [
            lottery.key().as_ref(),
            TICKET_SEED.as_bytes(),
            &(lottery.last_ticket_id + 1).to_le_bytes()
        ],
        bump,
    )]
    pub ticket: Account<'info, Ticket>,

    #[account(mut)]
    pub buyer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Ticket {
    pub id: u32,
    pub authority: Pubkey,
    pub lottery_id: u32,
}
