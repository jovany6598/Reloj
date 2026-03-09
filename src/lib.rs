use anchor_lang::prelude::*;

declare_id!("9ZLfyHxn9mkSPi2euph8dw5z6suGyxy8nM2VcQJwRhZi");

#[program]
pub mod relojeria {
    use super::*;

    //////////////////////////// CREAR COLECCION DE RELOJES ////////////////////////////

    pub fn crear_coleccion(context: Context<NuevaColeccion>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let relojes: Vec<Reloj> = Vec::new();

        context.accounts.coleccion.set_inner(ColeccionRelojes {
            owner: owner_id,
            nombre,
            relojes,
        });

        Ok(())
    }

    //////////////////////////// AGREGAR RELOJ ////////////////////////////

    pub fn agregar_reloj(
        context: Context<NuevoReloj>,
        marca: String,
        modelo: String,
        precio: u16,
    ) -> Result<()> {

        require!(
            context.accounts.coleccion.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let reloj = Reloj {
            marca,
            modelo,
            precio,
            disponible: true,
        };

        context.accounts.coleccion.relojes.push(reloj);

        Ok(())
    }

    //////////////////////////// ELIMINAR RELOJ ////////////////////////////

    pub fn eliminar_reloj(context: Context<NuevoReloj>, modelo: String) -> Result<()> {

        require!(
            context.accounts.coleccion.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let relojes = &mut context.accounts.coleccion.relojes;

        for i in 0..relojes.len() {
            if relojes[i].modelo == modelo {
                relojes.remove(i);
                msg!("Reloj {} eliminado!", modelo);
                return Ok(());
            }
        }

        Err(Errores::RelojNoExiste.into())
    }

    //////////////////////////// VER RELOJES ////////////////////////////

    pub fn ver_relojes(context: Context<NuevoReloj>) -> Result<()> {

        require!(
            context.accounts.coleccion.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "Lista de relojes registrados: {:#?}",
            context.accounts.coleccion.relojes
        );

        Ok(())
    }

    //////////////////////////// CAMBIAR DISPONIBILIDAD ////////////////////////////

    pub fn alternar_estado(context: Context<NuevoReloj>, modelo: String) -> Result<()> {

        require!(
            context.accounts.coleccion.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let relojes = &mut context.accounts.coleccion.relojes;

        for i in 0..relojes.len() {

            if relojes[i].modelo == modelo {

                let estado = relojes[i].disponible;

                let nuevo_estado = !estado;

                relojes[i].disponible = nuevo_estado;

                msg!(
                    "El reloj {} ahora tiene disponibilidad: {}",
                    modelo,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        Err(Errores::RelojNoExiste.into())
    }
}

//////////////////////////// ERRORES ////////////////////////////

#[error_code]
pub enum Errores {

    #[msg("Error: No eres el propietario de esta coleccion")]
    NoEresElOwner,

    #[msg("Error: El reloj no existe")]
    RelojNoExiste,
}

//////////////////////////// CUENTA PRINCIPAL ////////////////////////////

#[account]
#[derive(InitSpace)]
pub struct ColeccionRelojes {

    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    relojes: Vec<Reloj>,
}

//////////////////////////// STRUCT RELOJ ////////////////////////////

#[derive(
    AnchorSerialize,
    AnchorDeserialize,
    Clone,
    InitSpace,
    PartialEq,
    Debug
)]
pub struct Reloj {

    #[max_len(30)]
    marca: String,

    #[max_len(40)]
    modelo: String,

    precio: u16,

    disponible: bool,
}

//////////////////////////// CONTEXTOS ////////////////////////////

#[derive(Accounts)]
pub struct NuevaColeccion<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = ColeccionRelojes::INIT_SPACE + 8,
        seeds = [b"relojes", owner.key().as_ref()],
        bump
    )]
    pub coleccion: Account<'info, ColeccionRelojes>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoReloj<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub coleccion: Account<'info, ColeccionRelojes>,
}
