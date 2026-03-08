🎨 ArtChain — Gestión de Arte Digital en Solana

ArtChain es un programa on-chain desarrollado en Rust con Anchor Framework sobre la blockchain de Solana.
Permite a artistas crear, gestionar y administrar sus obras de arte digital de forma descentralizada, transparente e inmutable.

📌 ¿Qué hace el proyecto?

ArtChain implementa un sistema CRUD completo para administrar arte digital:

Crear una galería de arte vinculada a tu wallet (owner)

Registrar obras de arte con título, autor y precio

Eliminar obras cerrando su cuenta en la blockchain

Activar o desactivar una obra (ej: obra en venta o no disponible)

Actualizar el precio o información de la obra

Cada galería y cada obra de arte son cuentas derivadas (PDA) únicas en Solana, lo que garantiza que:

No puede haber obras duplicadas

Solo el owner autorizado puede modificarlas

🏗️ Arquitectura
Owner (Wallet)
    │
    └── Galería (PDA)
            │
            ├── Obra A (PDA)
            ├── Obra B (PDA)
            └── Obra C (PDA)
📦 Structs principales
Galeria
Campo	Tipo	Descripción
owner	Pubkey	Wallet del artista
nombre	String	Nombre de la galería
obras	Vec<Pubkey>	Lista de PDAs de obras
Obra
Campo	Tipo	Descripción
galeria	String	Nombre de la galería
titulo	String	Título de la obra
precio	u64	Precio del arte digital
disponible	bool	Estado de venta de la obra
⚙️ Instrucciones (Funciones del programa)
Instrucción	Descripción
crear_galeria(nombre)	Crea la cuenta de la galería vinculada al artista
registrar_obra(titulo, precio)	Registra una nueva obra de arte digital
eliminar_obra(titulo)	Elimina una obra y cierra su cuenta
alternar_disponibilidad(titulo)	Activa o desactiva la venta de la obra
actualizar_precio(titulo, precio)	Actualiza el precio de la obra
🔐 PDAs (Program Derived Addresses)

Las cuentas se derivan con los siguientes seeds:

Galería:

["galeria", nombre_galeria, owner_pubkey]

Obra:

["obra", titulo_obra, owner_pubkey]

Esto garantiza que:

Cada artista tiene su propia galería única

No pueden existir dos obras con el mismo título en la misma galería

🚀 Cómo usar el proyecto (Solana Playground)

Abre Solana Playground

Haz fork de este repositorio o pega el contenido de src/lib.rs

Conecta tu wallet (devnet)

Haz clic en Build y luego Deploy

Usa el panel Test para interactuar con el programa

Ejemplo de flujo
1. crear_galeria("ArteSara")
2. registrar_obra("Atardecer Digital", 2)
3. alternar_disponibilidad("Atardecer Digital") → obra no disponible
4. actualizar_precio("Atardecer Digital", 5) → nuevo precio
5. eliminar_obra("Atardecer Digital") → elimina la obra
🛠️ Tecnologías

Solana — Blockchain de alta velocidad

Anchor Framework — Framework para programas Solana en Rust

Rust — Lenguaje del programa on-chain

👤 Autor

Proyecto desarrollado como parte de la certificación de Solana.
Autor: Sarahi De Jesús 🎨🚀
