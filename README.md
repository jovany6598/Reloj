# ⌚ CRUD de Relojes en Solana con Anchor

Este proyecto implementa un **CRUD (Create, Read, Update, Delete)** de relojes utilizando **Rust, Anchor Framework y la blockchain de Solana**.

El programa permite a un usuario crear una colección de relojes y administrar los relojes almacenados en ella.

---

# 📚 Descripción del Proyecto

El contrato inteligente crea una **cuenta PDA (Program Derived Address)** donde se almacenan los datos de una colección de relojes.

Cada colección contiene:

* Propietario (`owner`)
* Nombre de la colección
* Lista de relojes

Cada reloj contiene:

* Marca
* Modelo
* Precio
* Disponibilidad

Todas las operaciones solo pueden ser realizadas por el **propietario de la colección**.

---

# 🧱 Estructura del Programa

El programa está dividido en varias partes principales:

## 1️⃣ Programa Principal

```rust
#[program]
pub mod relojeria
