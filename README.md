# Badge System - Programa Solana

Un sistema descentralizado para la creación y gestión de cursos con sistema de insignias (badges) construido en Solana usando Anchor Framework.

## Descripción

Este programa permite a educadores crear cursos y otorgar badges a estudiantes que completan los módulos requeridos. Cada badge es un NFT-like account en Solana que certifica el logro del estudiante.

## Características

- Crear cursos con nombre, descripción y número de módulos
- Crear badges asociados a módulos específicos
- Otorgar badges automáticamente al completar módulos
- Actualizar información del curso
- Eliminar cursos (solo si no tienen badges)
- Verificar si un estudiante posee un badge
- Control de acceso: solo el propietario puede modificar el curso

## 📦 Estructura de Cuentas

### Curso
```rust
pub struct Curso {
    pub owner: Pubkey,          // Dueño del curso
    pub id_curso: u64,          // ID único del curso
    pub nombre: String,         // Nombre (máx 50 chars)
    pub descripcion: String,    // Descripción (máx 200 chars)
    pub total_modulos: u8,      // Total de módulos (1-50)
    pub badges_creados: u8,     // Cantidad de badges
    pub badges: Vec<Pubkey>,    // Direcciones de badges
}
pub struct Badge {
    pub curso: Pubkey,          // Curso al que pertenece
    pub id_badge: u64,          // ID único del badge
    pub nombre: String,         // Nombre (máx 40 chars)
    pub descripcion: String,    // Descripción (máx 150 chars)
    pub modulo_requerido: u8,   // Módulo necesario
}
pub struct EstudianteBadge {
    pub estudiante: Pubkey,     // Estudiante que lo obtuvo
    pub badge: Pubkey,          // Badge obtenido
    pub curso: Pubkey,          // Curso relacionado
    pub obtenido_en: i64,       // Timestamp de obtención
}
