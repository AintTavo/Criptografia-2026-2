# Despliegue — `crypto.eddndev.work`

Guía para desplegar la Práctica 5 en un VPS propio, con un **único origen**
detrás de Caddy (HTTPS automático) que sirve el frontend estático y hace de
reverse proxy del backend Rust.

```
                 Internet
                    │  443 (HTTPS)
            ┌───────▼────────┐
            │     Caddy      │  crypto.eddndev.work
            │  (TLS + proxy) │
            └───┬────────┬───┘
       estático │        │ /login /signin /val
                │        │ /restore /restore/init /verify
        ┌───────▼──┐  ┌──▼───────────────┐
        │ frontend │  │ backend_api Rust │  127.0.0.1:3000
        │  (gui/)  │  │  + SQLite        │
        └──────────┘  └──────────────────┘
```

> **Requisito previo:** compila el backend desde la rama del PR de hardening
> (`feat/security-hardening`, PR #3) o desde `main` ya con ese PR mergeado.
> Ese PR trae el `.env.example`, el salt y el token de recuperación seguro.

---

## 1. DNS

En tu proveedor DNS de `eddndev.work`, crea un registro:

```
Tipo: A      Nombre: crypto      Valor: <IP pública del VPS>
```

(Si usas Cloudflare, deja el proxy **desactivado** —nube gris— al menos para
el primer arranque, así Caddy puede emitir el certificado por HTTP-01.)

Verifica: `dig +short crypto.eddndev.work` debe devolver la IP del VPS.

## 2. Dependencias en el VPS

```bash
# Toolchain Rust (si no está)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Compilador de C para rusqlite (bundled). TLS = rustls, NO necesita openssl.
sudo apt install -y build-essential        # Debian/Ubuntu
# Caddy
sudo apt install -y debian-keyring debian-archive-keyring apt-transport-https curl
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | sudo gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | sudo tee /etc/apt/sources.list.d/caddy-stable.list
sudo apt update && sudo apt install -y caddy
```

## 3. Compilar y colocar archivos

En tu máquina o en el VPS, dentro del repo:

```bash
cd Practica5-LogIn/backend/backend_api
cargo build --release
```

En el VPS:

```bash
sudo useradd --system --no-create-home --shell /usr/sbin/nologin cryptosvc
sudo mkdir -p /opt/crypto-login/backend_api /opt/crypto-login/sql /var/www/crypto-login

# Binario + esquema SQL
sudo cp target/release/backend_api            /opt/crypto-login/backend_api/
sudo cp ../sql/db.sql                          /opt/crypto-login/sql/db.sql

# Frontend estático (contenido de gui/, no la carpeta)
sudo cp -r ../../frontend/gui/.                /var/www/crypto-login/

sudo chown -R cryptosvc:cryptosvc /opt/crypto-login
sudo chown -R caddy:caddy /var/www/crypto-login
```

## 4. Archivo `.env`

```bash
sudo -u cryptosvc tee /opt/crypto-login/backend_api/.env >/dev/null <<'EOF'
APP_URL=https://crypto.eddndev.work
PORT=3000
JWT_SECRET=__pega_aqui_la_salida_de_openssl_rand_-hex_32__
DATABASE_FILE=sqlite.db
SQL_FILE_PATH=../sql/db.sql
SMTP_SERVER=smtp.resend.com
SMTP_USER=resend
SMTP_PASS=re_tu_api_key_real
EOF
sudo chmod 600 /opt/crypto-login/backend_api/.env
```

Genera el secreto con `openssl rand -hex 32`.
`DATABASE_FILE=sqlite.db` se crea solo al primer arranque (relativo a
`WorkingDirectory`). `SQL_FILE_PATH=../sql/db.sql` resuelve a
`/opt/crypto-login/sql/db.sql`.

### Resend

1. En https://resend.com verifica el dominio **`eddndev.work`** (registros
   SPF/DKIM que te da Resend, en tu DNS).
2. Crea una **API key** y ponla en `SMTP_PASS`.
3. ⚠️ El backend usa `SMTP_USER` como dirección del remitente (`From`). Resend
   exige que el `From` sea de un dominio verificado. Si dejas `SMTP_USER=resend`
   el `From` será inválido. Opciones:
   - **Recomendado:** ajustar el `From` en `send_email` /
     `send_verification_email` a `no-reply@eddndev.work` (cambio de 1 línea,
     fuera del alcance de este PR — anótalo como seguimiento), o
   - usar `SMTP_USER=no-reply@eddndev.work` y la API key igual en `SMTP_PASS`
     (Resend SMTP acepta `resend` o el email; el `From` saldría correcto).

## 5. Servicio systemd

```bash
sudo cp Practica5-LogIn/deploy/backend_api.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now backend_api
sudo systemctl status backend_api          # debe decir "Listening on port 3000"
```

## 6. Caddy

```bash
sudo cp Practica5-LogIn/deploy/Caddyfile /etc/caddy/Caddyfile
sudo mkdir -p /var/log/caddy && sudo chown caddy:caddy /var/log/caddy
sudo systemctl reload caddy
sudo journalctl -u caddy -f                # ver emisión del certificado
```

## 7. Firewall

Solo 80/443 públicos. El backend (3000) **nunca** debe ser accesible desde
fuera; Caddy lo alcanza por loopback.

```bash
sudo ufw allow 80,443/tcp
sudo ufw deny 3000/tcp
sudo ufw enable
```

## 8. Comprobación

- `https://crypto.eddndev.work/` → pantalla de login con candado HTTPS.
- Registro → llega correo de verificación; el enlace
  `https://crypto.eddndev.work/frontend/gui/html/verify.html?token=...` activa
  la cuenta.
- Login bloqueado hasta verificar; tras verificar, entra y redirige a
  `secret.html`.
- "¿Olvidaste tu contraseña?" → correo con enlace de recuperación (válido 1 h).

## Notas

- **Origen único:** los 3 JS usan `const API = ''` (rutas relativas), por eso
  todo funciona sin CORS. El `CorsLayer(Any)` del backend queda inocuo.
- **BD limpia:** al venir de `.gitignore`, el `sqlite.db` se crea vacío en el
  primer arranque. No hay usuarios heredados.
- **Reinicios:** reiniciar el backend invalida los tokens de recuperación en
  curso (la tabla se reconstruye). Aceptable para esta práctica.
- **Actualizar:** recompila, `sudo systemctl restart backend_api`, y para el
  frontend recopia `gui/` a `/var/www/crypto-login`.
