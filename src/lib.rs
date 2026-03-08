use anchor_lang::prelude::*;

declare_id!("Art111111111111111111111111111111111111111");

#[program]
pub mod tienda_arte {
    use super::*;

    pub fn crear_arte(
        ctx: Context<CrearArte>,
        titulo: String,
        autor: String,
        precio: u64,
        descripcion: String,
    ) -> Result<()> {

        let arte = &mut ctx.accounts.arte;

        arte.titulo = titulo;
        arte.autor = autor;
        arte.precio = precio;
        arte.descripcion = descripcion;
        arte.owner = *ctx.accounts.user.key;

        Ok(())
    }

    pub fn actualizar_arte(
        ctx: Context<ActualizarArte>,
        titulo: String,
        precio: u64,
    ) -> Result<()> {

        let arte = &mut ctx.accounts.arte;

        arte.titulo = titulo;
        arte.precio = precio;

        Ok(())
    }

    pub fn eliminar_arte(_ctx: Context<EliminarArte>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CrearArte<'info> {

    #[account(
        init,
        payer = user,
        space = 8 + 200
    )]
    pub arte: Account<'info, Arte>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarArte<'info> {

    #[account(mut)]
    pub arte: Account<'info, Arte>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct EliminarArte<'info> {

    #[account(mut, close = user)]
    pub arte: Account<'info, Arte>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct Arte {

    pub titulo: String,
    pub autor: String,
    pub precio: u64,
    pub descripcion: String,
    pub owner: Pubkey,
}
