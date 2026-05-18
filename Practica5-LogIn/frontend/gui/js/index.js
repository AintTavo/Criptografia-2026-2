// ── Utilidades ──────────────────────────────────────────────────────────
// Mismo origen: el reverse proxy enruta /login, /signin, etc. al backend.
// Para desarrollo con el backend en otro puerto: const API = 'http://localhost:3000';
const API = '';

function $(id) { return document.getElementById(id); }

function showToast(msg, type = 'success', duration = 3200) {
  const t = $('toast');
  t.textContent = msg;
  t.className = `toast ${type} show`;
  clearTimeout(t._timer);
  t._timer = setTimeout(() => { t.className = 'toast'; }, duration);
}

function setLoading(btnId, on) {
  const btn = $(btnId);
  btn.classList.toggle('loading', on);
  btn.disabled = on;
}

function clearErrors(ids) {
  ids.forEach(id => {
    const el = document.getElementById(id + '-err');
    const inp = document.getElementById(id);
    if (el)  el.classList.remove('show');
    if (inp) inp.classList.remove('error');
  });
}

function showError(id, msg) {
  const err = $(id + '-err');
  const inp = $(id);
  if (err) { err.textContent = msg || err.textContent; err.classList.add('show'); }
  if (inp)  inp.classList.add('error');
}

// ── JWT en memoria ──────────────────────────────────────────────────────
let SESSION_JWT = null;

function saveJWT(token) {
  SESSION_JWT = token;
  sessionStorage.setItem('jwt', token); // sessionStorage (tab-level)
}

function getJWT() {
  return SESSION_JWT || sessionStorage.getItem('jwt');
}

// ── Auto-redirect si ya hay sesión ──────────────────────────────────────
(async function checkSession() {
  const jwt = getJWT();
  if (!jwt) return;
  try {
    const res = await fetch(`${API}/val`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ jwt })
    });
    if (res.ok) {
      window.location.href = './html/secret.html';
    }
  } catch { /* red caída, mostrar login normal */ }
})();

// ── Cambio de modo ──────────────────────────────────────────────────────
let currentMode = 'login';

function setMode(mode) {
  currentMode = mode;

  // Tabs
  $('tab-login').className  = 'mode-tab' + (mode === 'login'  ? ' active-login'  : '');
  $('tab-signin').className = 'mode-tab' + (mode === 'signin' ? ' active-signin' : '');

  // Paneles
  $('panel-login').classList.toggle('active', mode === 'login');
  $('panel-signin').classList.toggle('active', mode === 'signin');

  // Body class (para el fondo)
  document.body.classList.toggle('mode-signin', mode === 'signin');

  // Variables CSS de acento
  const root = document.documentElement;
  if (mode === 'login') {
    root.style.setProperty('--accent',  'var(--login-accent)');
    root.style.setProperty('--accent2', 'var(--login-accent2)');
    root.style.setProperty('--glow',    'var(--login-glow)');
    root.style.setProperty('--glow2',   'var(--login-glow2)');
    root.style.setProperty('--aborder', 'var(--login-border)');
    $('logo').style.color = 'var(--login-accent)';
  } else {
    root.style.setProperty('--accent',  'var(--signin-accent)');
    root.style.setProperty('--accent2', 'var(--signin-accent2)');
    root.style.setProperty('--glow',    'var(--signin-glow)');
    root.style.setProperty('--glow2',   'var(--signin-glow2)');
    root.style.setProperty('--aborder', 'var(--signin-border)');
    $('logo').style.color = 'var(--signin-accent)';
  }
}

// ── Validaciones ────────────────────────────────────────────────────────
function validEmail(v) { return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(v.trim()); }

// ── Login ────────────────────────────────────────────────────────────────
async function doLogin() {
  clearErrors(['l-email', 'l-pass']);
  const email = $('l-email').value.trim();
  const pass  = $('l-pass').value;
  let ok = true;

  if (!validEmail(email))  { showError('l-email', 'Correo no válido');              ok = false; }
  if (pass.length < 1)     { showError('l-pass',  'La contraseña no puede estar vacía'); ok = false; }
  if (!ok) return;

  setLoading('btn-login', true);
  try {
    const res = await fetch(`${API}/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email, password: pass })
    });
    const data = await res.json();
    if (res.ok && data.jwt) {
      saveJWT(data.jwt);
      showToast('✓ Acceso concedido. Redirigiendo…', 'success');
      setTimeout(() => { window.location.href = './html/secret.html'; }, 1000);
    } else {
      showToast(data.msg_err || 'Credenciales incorrectas', 'error');
      showError('l-pass', 'Credenciales incorrectas');
    }
  } catch (e) {
    showToast('Error de conexión con el servidor', 'error');
  } finally {
    setLoading('btn-login', false);
  }
}

// ── Sign in ─────────────────────────────────────────────────────────────
async function doSignin() {
  clearErrors(['s-user', 's-email', 's-pass']);
  const user  = $('s-user').value.trim();
  const email = $('s-email').value.trim();
  const pass  = $('s-pass').value;
  let ok = true;

  if (user.length < 3 || /\s/.test(user)) { showError('s-user',  'Mínimo 3 caracteres, sin espacios'); ok = false; }
  if (!validEmail(email))                   { showError('s-email', 'Correo no válido');                  ok = false; }
  if (pass.length < 8)                      { showError('s-pass',  'Mínimo 8 caracteres');               ok = false; }
  if (!ok) return;

  setLoading('btn-signin', true);
  try {
    const res = await fetch(`${API}/signin`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username: user, email, password: pass })
    });
    const data = await res.json();
    if (res.ok) {
      showToast('✓ Cuenta creada. Revisa tu correo para verificarla.', 'success', 5000);
      setTimeout(() => { setMode('login'); }, 3000); // Regresa a la pestaña de login
    } else {
      showToast(data.msg_err || 'Error al crear la cuenta', 'error');
    }
  } catch (e) {
    showToast('Error de conexión con el servidor', 'error');
  } finally {
    setLoading('btn-signin', false);
  }
}

// ── Restablecer contraseña ───────────────────────────────────────────────
function openRestore()  { $('modal-overlay').classList.add('open');    }
function closeRestore() { $('modal-overlay').classList.remove('open'); }
function closeRestoreOutside(e) { if (e.target === $('modal-overlay')) closeRestore(); }

async function doRestore() {
  clearErrors(['r-email']);
  const email = $('r-email').value.trim();
  if (!validEmail(email)) { showError('r-email', 'Correo no válido'); return; }

  try {
    const res = await fetch(`${API}/restore/init`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ email })
    });
    const data = await res.json().catch(() => ({}));
    if (res.ok) {
      showToast('✉ Enlace enviado a tu correo', 'success');
      closeRestore();
      $('r-email').value = '';
    } else {
      const errorMsg = data.err_msg || 'Error al enviar el correo';
      showToast(errorMsg, 'error');
      showError('r-email', errorMsg);
    }
  } catch {
    showToast('Error de conexión con el servidor', 'error');
  }
}

// Enter para submit
document.addEventListener('keydown', e => {
  if (e.key !== 'Enter') return;
  if ($('modal-overlay').classList.contains('open')) { doRestore(); return; }
  if (currentMode === 'login')  doLogin();
  else                          doSignin();
});