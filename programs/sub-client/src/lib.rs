use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[event]
#[derive(Debug)]
pub struct MyEvent {
    pub event: String,
}

#[program]
pub mod sub_client {
    use super::*;

    pub fn instruction_1(_ctx: Context<Instruction1>) -> Result<()> {
        msg!("Instruction1");

        emit!(MyEvent {
            event: String::from("Instruction1")
        });
        Ok(())
    }

    pub fn instruction_2(_ctx: Context<Instruction2>) -> Result<()> {
        msg!("Instruction2");
        emit!(MyEvent {
            event: String::from("Instruction2")
        });
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Instruction1<'info> {
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Instruction2<'info> {
    pub payer: Signer<'info>,
}
