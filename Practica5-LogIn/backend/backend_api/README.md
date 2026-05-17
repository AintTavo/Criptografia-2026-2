Crear tu archivo .env

E incluir dos campos
- Nombre de la app
  - APP_URL
- Puerto
  - PORT
- Manejo de tokens
  - JWT_SECRET
- Base de datos
  - DATABASE_FILE
  - SQL_FILE_PATH
- Servidor de correos
  - SMTP_SERVER
  - SMTP_USER
  - SMTP_PASS

Para generar el JWT_SECRET ejecutar en terminal :
openssl rand -hex 32

Se requiera haber instalado dependencias criptograficas del sistema
- sudo apt install pkg-config libssl-dev
- sudo dnf install pkgconfig openssl-devel

Normalmente en el puerto 3000:

/signin [POST] {
  username : String
  email : String
  password : String
}

/login [GET] {
  email : String
  password : String
}

/val [GET] {
  jwt : String
}

/restore/init [POST] {
  email : String
}

/restore [POST] {
  new_pass: String,
  token : String
}
