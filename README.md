# ⌚ Relojería Program (Solana Anchor)

Un contrato inteligente desarrollado en **Solana** utilizando el framework **Anchor**. Este programa permite a los usuarios gestionar su propia colección de relojes de forma descentralizada, permitiendo crear colecciones, añadir piezas, eliminar registros y gestionar la disponibilidad de cada reloj.

---

## 🚀 Características

* **Creación de Colección Personalizada:** Cada usuario puede inicializar su propia cuenta de colección vinculada a su `PublicKey`.
* **Gestión de Inventario:** Funciones para agregar y eliminar relojes mediante metadatos (marca, modelo, precio).
* **Control de Estado:** Posibilidad de alternar la disponibilidad (stock) de los relojes.
* **Seguridad:** Validaciones integradas para asegurar que solo el propietario de la colección pueda realizar modificaciones.
* **Espacio Optimizado:** Uso de `InitSpace` para un cálculo preciso del alquiler (rent) en la blockchain.

---

## 🛠️ Estructura de Datos

### ColeccionRelojes (Account)
Es la cuenta principal que almacena la información del usuario:
* `owner`: Pubkey del dueño.
* `nombre`: Nombre de la colección (máx. 60 caracteres).
* `relojes`: Un vector dinámico de estructuras `Reloj`.

### Reloj (Struct)
* `marca`: String (máx. 30 caracteres).
* `modelo`: String (máx. 40 caracteres).
* `precio`: Valor numérico `u16`.
* `disponible`: Booleano para control de inventario.

---

## 📋 Funciones del Programa

| Función | Descripción |
| :--- | :--- |
| `crear_coleccion` | Inicializa la cuenta de la colección usando PDAs con semillas `[b"relojes", owner_key]`. |
| `agregar_reloj` | Añade un nuevo objeto `Reloj` al vector de la colección. |
| `eliminar_reloj` | Busca un reloj por su modelo y lo remueve del vector. |
| `ver_relojes` | Imprime en los logs de la transacción la lista actual de relojes. |
| `alternar_estado` | Cambia el estado de `disponible` entre `true` y `false`. |

---

## ⚠️ Requisitos Previos

* [Rust](https://www.rust-lang.org/tools/install)
* [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
* [Anchor Framework](https://www.anchor-lang.com/docs/installation)

---

## 🔧 Configuración y Despliegue

1.  **Clonar el repositorio e instalar dependencias:**
    ```bash
    anchor build
    ```

2.  **Obtener el Program ID:**
    El ID del programa generado es: `"9ZLfyHxn9mkSPi2euph8dw5z6suGyxy8nM2VcQJwRhZi"`. Si deseas cambiarlo, ejecuta `anchor keys list` y reemplázalo en `declare_id!()` y `Anchor.toml`.

3.  **Desplegar en Localnet:**
    ```bash
    solana-test-validator
    anchor deploy
    ```

4.  **Ejecutar Tests:**
    ```bash
    anchor test
    ```

---

## 🛡️ Manejo de Errores

El programa incluye un módulo `Errores` para proporcionar feedback claro:
* `NoEresElOwner`: Se dispara si alguien intenta modificar una colección que no le pertenece.
* `RelojNoExiste`: Se dispara si se intenta eliminar o modificar un modelo que no se encuentra en el vector.

---

## 📝 Notas de Implementación
* **PDA (Program Derived Address):** La cuenta de la colección se deriva de la frase semilla `"relojes"` y la llave pública del usuario, lo que garantiza que cada usuario tenga una única colección predecible.
* **Límites de Memoria:** El vector de relojes tiene un límite definido de **10 elementos** mediante el atributo `#[max_len(10)]` para evitar exceder el límite de tamaño de cuenta de Solana.

---

**Desarrollado con ❤️ para el ecosistema Solana.**
