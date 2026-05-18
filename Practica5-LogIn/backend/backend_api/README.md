# Backend API — Práctica 5 (LogIn)

## 1. Configuración (`.env`)

Copia `.env.example` a `.env` y rellena los valores. **El `.env` no se versiona** (está en `.gitignore`).

| Variable        | Descripción                                                        |
|-----------------|--------------------------------------------------------------------|
| `APP_URL`       | URL pública del frontend (sin slash final). Se usa en los enlaces de correo. |
| `PORT`          | Puerto donde escucha la API (normalmente `3000`).                  |
| `JWT_SECRET`    | Secreto para firmar los JWT de sesión. Generar con `openssl rand -hex 32`. |
| `DATABASE_FILE` | Ruta del archivo SQLite (p. ej. `sqlite.db`).                      |
| `SQL_FILE_PATH` | Ruta a `../sql/db.sql` (esquema inicial).                          |
| `SMTP_SERVER`   | Host SMTP. Con Resend: `smtp.resend.com`.                          |
| `SMTP_USER`     | Usuario SMTP. Con Resend: literalmente `resend`.                   |
| `SMTP_PASS`     | Contraseña SMTP. Con Resend: tu **API key** (`re_...`).            |

### Servidor de correo: Resend (recomendado)

El backend envía correos por SMTP con STARTTLS (`starttls_relay`). Para usar Resend:

1. Crea una cuenta en https://resend.com y verifica un dominio (o usa el dominio de pruebas).
2. Genera una **API key**.
3. En tu `.env`:

   ```
   SMTP_SERVER=smtp.resend.com
   SMTP_USER=resend
   SMTP_PASS=re_tu_api_key_aqui
   ```

> El remitente (`From`) que arma el código usa `SMTP_USER` como dirección. Con Resend,
> ese `From` debe ser una dirección de un dominio verificado en tu cuenta, así que
> probablemente quieras ajustar el remitente en `send_email` / `send_verification_email`
> a algo como `no-reply@tudominio.com` antes de salir a producción.

Cualquier otro proveedor SMTP con STARTTLS (Gmail con App Password, Brevo, Mailgun…)
también funciona usando sus propios `SMTP_SERVER/USER/PASS`.

## 2. Dependencias del sistema

`rusqlite` se compila con SQLite embebido (feature `bundled`), por lo que solo necesitas
un compilador de C. TLS se maneja con `rustls` (no requiere OpenSSL del sistema).

- Debian/Ubuntu: `sudo apt install build-essential`
- Fedora: `sudo dnf install gcc`

## 3. Base de datos

El esquema (`../sql/db.sql`) se crea automáticamente la primera vez. Al arrancar, el
servidor además:

- agrega las columnas `verified` y `salt` a `users` si no existen;
- **reconstruye** la tabla `restore_token` (los tokens de recuperación son efímeros).

> Despliegue limpio: se asume una base de datos **nueva**. Una BD anterior a estos
> cambios tendría usuarios sin `salt` (`NULL`); esas cuentas no podrán iniciar sesión
> hasta restablecer la contraseña. No hay migración de datos antiguos.

## 4. Seguridad de tokens

- **Contraseñas**: `SHA3-256(salt || password)` con un `salt` aleatorio de 16 bytes por usuario.
- **Recuperación**: token opaco aleatorio de 32 bytes; en la BD se guarda **solo su hash
  SHA3-256** y un `expires_at` (vida útil: 1 hora). El valor en claro solo viaja en el
  correo. Ya **no** se incrusta el hash de la contraseña en un JWT (corrección de seguridad).

## 5. Endpoints (puerto `PORT`, normalmente 3000)

```
/signin       [POST] { username, email, password }   -> envía correo de verificación
/verify       [POST] { token }                        -> activa la cuenta
/login        [POST] { email, password }              -> requiere cuenta verificada
/val          [POST] { jwt }
/restore/init [POST] { email }                        -> envía correo de recuperación
/restore      [POST] { new_pass, token }
```

## 6. Ejecutar

```
cargo build --release
./target/release/backend_api
```
