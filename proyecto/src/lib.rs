use anchor_lang::prelude::*;

declare_id!("28J2HpqohH5ddw6hNtEHTjegCcErirsbPpyQTUGntDWq");

#[program]
pub mod badge_system {
    use super::*;

    // ============================================
    // 1. CREAR CURSO
    // ============================================
    pub fn crear_curso(
        ctx: Context<CrearCurso>,
        id_curso: u64,
        nombre: String,
        descripcion: String,
        total_modulos: u8,
    ) -> Result<()> {
        let owner_id = ctx.accounts.owner.key();

        // Validaciones
        require!(nombre.len() <= 50, ErrorCode::NombreMuyLargo);
        require!(descripcion.len() <= 200, ErrorCode::DescripcionMuyLarga);
        require!(
            total_modulos > 0 && total_modulos <= 50,
            ErrorCode::ModulosInvalidos
        );

        let badges = Vec::<Pubkey>::new();

        ctx.accounts.curso.set_inner(Curso {
            owner: owner_id,
            id_curso,
            nombre: nombre.clone(),
            descripcion: descripcion.clone(),
            total_modulos,
            badges_creados: 0,
            badges,
        });

        msg!(
            "Curso '{}' creado exitosamente! Owner: {}",
            nombre,
            owner_id
        );
        Ok(())
    }

    // ============================================
    // 2. CREAR BADGE (solo el dueño del curso)
    // ============================================
    pub fn crear_badge(
        ctx: Context<CrearBadge>,
        id_badge: u64,
        nombre: String,
        descripcion: String,
        modulo_requerido: u8,
    ) -> Result<()> {
        // Verificar que el owner sea quien modifica
        require!(
            ctx.accounts.curso.owner == ctx.accounts.owner.key(),
            ErrorCode::NoEresElOwner
        );

        // Validaciones
        require!(nombre.len() <= 40, ErrorCode::NombreMuyLargo);
        require!(descripcion.len() <= 150, ErrorCode::DescripcionMuyLarga);
        require!(
            modulo_requerido > 0 && modulo_requerido <= ctx.accounts.curso.total_modulos,
            ErrorCode::ModuloInvalido
        );

        let badge = Badge {
            curso: ctx.accounts.curso.key(),
            id_badge,
            nombre: nombre.clone(),
            descripcion: descripcion.clone(),
            modulo_requerido,
        };

        ctx.accounts.badge.set_inner(badge);

        // Agregar badge al vector del curso
        ctx.accounts.curso.badges.push(ctx.accounts.badge.key());
        ctx.accounts.curso.badges_creados += 1;

        msg!(
            "Badge '{}' creado para el curso '{}' (requiere módulo {})",
            nombre,
            ctx.accounts.curso.nombre,
            modulo_requerido
        );

        Ok(())
    }

    // ============================================
    // 3. COMPLETAR MÓDULO Y OBTENER BADGE
    // ============================================
    pub fn completar_modulo(ctx: Context<CompletarModulo>, modulo_completado: u8) -> Result<()> {
        // Verificar que el estudiante completó al menos el módulo requerido
        require!(
            modulo_completado >= ctx.accounts.badge.modulo_requerido,
            ErrorCode::ModuloNoCompletado
        );

        // Verificar que el badge pertenece a este curso
        require!(
            ctx.accounts.badge.curso == ctx.accounts.curso.key(),
            ErrorCode::BadgeNoPertenece
        );

        let clock = Clock::get()?;

        let estudiante_badge = EstudianteBadge {
            estudiante: ctx.accounts.estudiante.key(),
            badge: ctx.accounts.badge.key(),
            curso: ctx.accounts.curso.key(),
            obtenido_en: clock.unix_timestamp,
        };

        ctx.accounts.estudiante_badge.set_inner(estudiante_badge);

        msg!(
            "Estudiante {} obtuvo el badge '{}' del curso '{}'",
            ctx.accounts.estudiante.key(),
            ctx.accounts.badge.nombre,
            ctx.accounts.curso.nombre
        );

        Ok(())
    }

    // ============================================
    // 4. ACTUALIZAR CURSO (solo dueño)
    // ============================================
    pub fn actualizar_curso(
        ctx: Context<ActualizarCurso>,
        nuevo_nombre: String,
        nueva_descripcion: String,
    ) -> Result<()> {
        require!(
            ctx.accounts.curso.owner == ctx.accounts.owner.key(),
            ErrorCode::NoEresElOwner
        );

        require!(nuevo_nombre.len() <= 50, ErrorCode::NombreMuyLargo);
        require!(
            nueva_descripcion.len() <= 200,
            ErrorCode::DescripcionMuyLarga
        );

        let curso = &mut ctx.accounts.curso;
        curso.nombre = nuevo_nombre.clone();
        curso.descripcion = nueva_descripcion.clone();

        msg!("Curso actualizado a: '{}'", nuevo_nombre);
        Ok(())
    }

    // ============================================
    // 5. ELIMINAR CURSO (solo si no tiene badges)
    // ============================================
    pub fn eliminar_curso(ctx: Context<EliminarCurso>) -> Result<()> {
        require!(
            ctx.accounts.curso.owner == ctx.accounts.owner.key(),
            ErrorCode::NoEresElOwner
        );

        require!(
            ctx.accounts.curso.badges_creados == 0,
            ErrorCode::CursoTieneBadges
        );

        msg!(
            "Curso '{}' eliminado exitosamente",
            ctx.accounts.curso.nombre
        );
        Ok(())
    }

    // ============================================
    // 6. VERIFICAR SI ESTUDIANTE TIENE UN BADGE
    // ============================================
    pub fn verificar_badge(ctx: Context<VerificarBadge>) -> Result<bool> {
        // Esta función solo verifica si existe la cuenta
        Ok(ctx.accounts.estudiante_badge.is_some())
    }
}

// ============================================
// ESTRUCTURAS DE LAS CUENTAS
// ============================================

#[account]
#[derive(InitSpace)]
pub struct Curso {
    pub owner: Pubkey, // Dueño del curso (profesor)
    pub id_curso: u64, // ID único del curso
    #[max_len(50)]
    pub nombre: String, // Nombre del curso
    #[max_len(200)]
    pub descripcion: String, // Descripción del curso
    pub total_modulos: u8, // Total de módulos (1-50)
    pub badges_creados: u8, // Cantidad de badges creados
    #[max_len(20)]
    pub badges: Vec<Pubkey>, // Direcciones de los badges
}

#[account]
#[derive(InitSpace)]
pub struct Badge {
    pub curso: Pubkey, // Curso al que pertenece
    pub id_badge: u64, // ID único del badge
    #[max_len(40)]
    pub nombre: String, // Nombre del badge
    #[max_len(150)]
    pub descripcion: String, // Descripción del badge
    pub modulo_requerido: u8, // Módulo necesario para obtenerlo
}

#[account]
#[derive(InitSpace)]
pub struct EstudianteBadge {
    pub estudiante: Pubkey, // Estudiante que obtuvo el badge
    pub badge: Pubkey,      // Badge obtenido
    pub curso: Pubkey,      // Curso relacionado
    pub obtenido_en: i64,   // Timestamp de cuando lo obtuvo
}

// ============================================
// CÓDIGOS DE ERROR
// ============================================

#[error_code]
pub enum ErrorCode {
    #[msg("No eres el propietario del curso")]
    NoEresElOwner,
    #[msg("El nombre es demasiado largo (máx 50 caracteres)")]
    NombreMuyLargo,
    #[msg("La descripción es demasiado larga (máx 200 caracteres)")]
    DescripcionMuyLarga,
    #[msg("Número de módulos inválido (debe ser 1-50)")]
    ModulosInvalidos,
    #[msg("Módulo requerido inválido")]
    ModuloInvalido,
    #[msg("El estudiante no ha completado el módulo requerido")]
    ModuloNoCompletado,
    #[msg("El badge no pertenece a este curso")]
    BadgeNoPertenece,
    #[msg("El curso tiene badges creados, no se puede eliminar")]
    CursoTieneBadges,
    #[msg("Puntuación inválida (debe ser 0-100)")]
    PuntuacionInvalida,
}

// ============================================
// CONTEXTOS (VALIDACIÓN DE CUENTAS)
// ============================================

// Contexto para crear curso
#[derive(Accounts)]
#[instruction(id_curso: u64)]
pub struct CrearCurso<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = 8 + Curso::INIT_SPACE,
        seeds = [b"curso", owner.key().as_ref(), &id_curso.to_le_bytes()],
        bump
    )]
    pub curso: Account<'info, Curso>,

    pub system_program: Program<'info, System>,
}

// Contexto para crear badge
#[derive(Accounts)]
#[instruction(id_badge: u64)]
pub struct CrearBadge<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"curso", curso.owner.as_ref(), &curso.id_curso.to_le_bytes()],
        bump
    )]
    pub curso: Account<'info, Curso>,

    #[account(
        init,
        payer = owner,
        space = 8 + Badge::INIT_SPACE,
        seeds = [b"badge", curso.key().as_ref(), &id_badge.to_le_bytes()],
        bump
    )]
    pub badge: Account<'info, Badge>,

    pub system_program: Program<'info, System>,
}

// Contexto para completar módulo y obtener badge
#[derive(Accounts)]
pub struct CompletarModulo<'info> {
    #[account(mut)]
    pub estudiante: Signer<'info>,

    #[account(
        seeds = [b"curso", curso.owner.as_ref(), &curso.id_curso.to_le_bytes()],
        bump
    )]
    pub curso: Account<'info, Curso>,

    #[account(
        mut,
        seeds = [b"badge", curso.key().as_ref(), &badge.id_badge.to_le_bytes()],
        bump
    )]
    pub badge: Account<'info, Badge>,

    #[account(
        init,
        payer = estudiante,
        space = 8 + EstudianteBadge::INIT_SPACE,
        seeds = [b"estudiante_badge", estudiante.key().as_ref(), badge.key().as_ref()],
        bump
    )]
    pub estudiante_badge: Account<'info, EstudianteBadge>,

    pub system_program: Program<'info, System>,
}

// Contexto para actualizar curso
#[derive(Accounts)]
pub struct ActualizarCurso<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"curso", curso.owner.as_ref(), &curso.id_curso.to_le_bytes()],
        bump
    )]
    pub curso: Account<'info, Curso>,
}

// Contexto para eliminar curso
#[derive(Accounts)]
pub struct EliminarCurso<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        close = owner,
        seeds = [b"curso", curso.owner.as_ref(), &curso.id_curso.to_le_bytes()],
        bump
    )]
    pub curso: Account<'info, Curso>,
}

// Contexto para verificar badge (solo lectura)
#[derive(Accounts)]
pub struct VerificarBadge<'info> {
    pub estudiante: Signer<'info>,

    #[account(
        seeds = [b"curso", curso.owner.as_ref(), &curso.id_curso.to_le_bytes()],
        bump
    )]
    pub curso: Account<'info, Curso>,

    #[account(
        seeds = [b"badge", curso.key().as_ref(), &badge.id_badge.to_le_bytes()],
        bump
    )]
    pub badge: Account<'info, Badge>,

    // Option porque puede que el estudiante no tenga el badge
    pub estudiante_badge: Option<Account<'info, EstudianteBadge>>,
}

