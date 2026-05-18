// Mismo origen (reverse proxy). Para desarrollo: const API = 'http://localhost:3000';
const API = '';

function $(id) { return document.getElementById(id); }

function showToast(msg, type = 'success', duration = 3000) {
  const t = $('toast');
  t.textContent = msg;
  t.className = `toast ${type} show`;
  clearTimeout(t._timer);
  t._timer = setTimeout(() => { t.className = ''; }, duration);
}

// Obtener el token de la URL (ej. ?token=xxxxx)
const urlParams = new URLSearchParams(window.location.search);
const token = urlParams.get('token');

if (!token) {
  $('restore-form').innerHTML = `
    <h2 style="color: #ff4c4c;">ENLACE INVÁLIDO</h2>
    <p>No se encontró el token de seguridad. El enlace está incompleto o ha sido modificado.</p>
    <br><a href="../index.html" style="color: #00ffcc; text-decoration: none;">Volver al inicio</a>
  `;
}

async function submitRestore() {
  const pass = $('new-pass').value;
  const passConfirm = $('new-pass-confirm').value;
  const btn = $('btn-restore');
  
  $('pass-err').style.display = 'none';
  $('confirm-err').style.display = 'none';

  let valid = true;

  if (pass.length < 8) {
    $('pass-err').textContent = 'Mínimo 8 caracteres';
    $('pass-err').style.display = 'block';
    valid = false;
  }
  
  if (pass !== passConfirm) {
    $('confirm-err').textContent = 'Las contraseñas no coinciden';
    $('confirm-err').style.display = 'block';
    valid = false;
  }

  if (!valid) return;

  btn.disabled = true;
  btn.textContent = 'PROCESANDO...';

  try {
    const res = await fetch(`${API}/restore`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ token: token, new_pass: pass })
    });
    
    const data = await res.json();
    
    if (res.ok) {
      showToast('Contraseña actualizada con éxito', 'success');
      setTimeout(() => {
        window.location.href = '../index.html';
      }, 2000);
    } else {
      showToast(data.err_msg || 'Error al actualizar', 'error');
      btn.disabled = false;
      btn.textContent = 'ACTUALIZAR CREDENCIALES';
    }
  } catch (err) {
    showToast('Error de conexión con el servidor', 'error');
    btn.disabled = false;
    btn.textContent = 'ACTUALIZAR CREDENCIALES';
  }
}
