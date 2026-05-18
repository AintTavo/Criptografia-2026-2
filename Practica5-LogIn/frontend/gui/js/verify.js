const API = 'http://localhost:3000'; // Ajusta si el backend corre en otro puerto

function $(id) { return document.getElementById(id); }

async function verifyAccount() {
  const urlParams = new URLSearchParams(window.location.search);
  const token = urlParams.get('token');

  const title = $('status-title');
  const msg = $('status-msg');
  const loader = $('loader');
  const btn = $('btn-home');

  if (!token) {
    loader.style.display = 'none';
    title.textContent = 'ENLACE INVÁLIDO';
    title.style.color = '#ff4c4c';
    msg.textContent = 'No se encontró el token de seguridad. El enlace está incompleto.';
    btn.style.display = 'inline-block';
    return;
  }

  try {
    const res = await fetch(`${API}/verify`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token: token })
    });
    
    const data = await res.json();
    loader.style.display = 'none';
    
    if (res.ok) {
      title.textContent = 'CUENTA VERIFICADA';
      title.style.color = '#00ffcc';
      msg.textContent = 'Tu correo electrónico ha sido confirmado. Ya puedes iniciar sesión de forma segura.';
    } else {
      title.textContent = 'ERROR DE VERIFICACIÓN';
      title.style.color = '#ff4c4c';
      msg.textContent = data.err_msg || 'El enlace es inválido o ha expirado.';
    }
    
    btn.style.display = 'inline-block';

  } catch (err) {
    loader.style.display = 'none';
    title.textContent = 'ERROR DE CONEXIÓN';
    title.style.color = '#ff4c4c';
    msg.textContent = 'No se pudo conectar con el servidor de autenticación.';
    btn.style.display = 'inline-block';
  }
}

// Ejecutar la verificación al cargar la página
window.addEventListener('DOMContentLoaded', verifyAccount);
