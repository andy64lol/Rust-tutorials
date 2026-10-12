/*
 * Rust Dojo - a tiny Duolingo-style app for practising Rust.
 *
 * Lesson content is loaded from data/lessons.json.
 * Progress is stored in localStorage and can be exported/imported as JSON.
 */

const STORAGE_KEY = 'rust-dojo-progress-v1';

const defaultProgress = () => ({
  version: 1,
  xp: 0,
  completed: [],
  streak: 0,
  lastDay: null,
});

let lessonsData = null;
let progress = loadProgress();
let session = null;
let toastTimer = null;

const app = document.getElementById('app');

/* ------------------------------------------------------------------ */
/* Storage helpers                                                     */
/* ------------------------------------------------------------------ */

function loadProgress() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return defaultProgress();
    return { ...defaultProgress(), ...JSON.parse(raw) };
  } catch {
    return defaultProgress();
  }
}

function saveProgress() {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(progress));
}

/* ------------------------------------------------------------------ */
/* Small utilities                                                     */
/* ------------------------------------------------------------------ */

function escapeHtml(value) {
  return String(value)
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

function normalize(text) {
  return String(text).trim().toLowerCase().replace(/\s+/g, ' ');
}

function dayStr(date) {
  return date.toISOString().slice(0, 10);
}

function toast(message) {
  const node = document.getElementById('toast');
  node.textContent = message;
  node.classList.add('show');
  clearTimeout(toastTimer);
  toastTimer = setTimeout(() => node.classList.remove('show'), 2200);
}

/* ------------------------------------------------------------------ */
/* Catalog helpers                                                     */
/* ------------------------------------------------------------------ */

function allLessons() {
  return lessonsData.units.flatMap((unit) => unit.lessons);
}

function findLesson(id) {
  return allLessons().find((lesson) => lesson.id === id);
}

function lessonPosition(id) {
  return allLessons().findIndex((lesson) => lesson.id === id);
}

function isCompleted(id) {
  return progress.completed.includes(id);
}

function isUnlocked(id) {
  const index = lessonPosition(id);
  if (index <= 0) return true;
  return isCompleted(allLessons()[index - 1].id);
}

/* ------------------------------------------------------------------ */
/* Header                                                              */
/* ------------------------------------------------------------------ */

function updateHeader() {
  const validIds = new Set(allLessons().map((lesson) => lesson.id));
  const done = progress.completed.filter((id) => validIds.has(id)).length;
  document.getElementById('stat-xp').textContent = progress.xp;
  document.getElementById('stat-done').textContent = done;
  document.getElementById('stat-streak').textContent = progress.streak || 0;
}

/* ------------------------------------------------------------------ */
/* Home / path view                                                    */
/* ------------------------------------------------------------------ */

function renderHome() {
  updateHeader();

  const units = lessonsData.units
    .map((unit) => {
      const nodes = unit.lessons
        .map((lesson) => {
          const done = isCompleted(lesson.id);
          const unlocked = isUnlocked(lesson.id);
          const cls = done ? 'done' : unlocked ? 'current' : 'locked';
          const icon = done ? '✓' : unlocked ? '★' : '🔒';
          return `
            <button class="node ${cls}" data-lesson="${lesson.id}" ${
              unlocked ? '' : 'disabled'
            }>
              <span class="node-icon">${icon}</span>
              <span class="node-label">${escapeHtml(lesson.title)}</span>
            </button>`;
        })
        .join('');

      return `
        <section class="unit">
          <div class="unit-banner" style="--unit:${unit.color || '#58cc02'}">
            <h2>${escapeHtml(unit.title)}</h2>
            <p>${escapeHtml(unit.description || '')}</p>
          </div>
          <div class="path">${nodes}</div>
        </section>`;
    })
    .join('');

  app.innerHTML = `<div class="home">${units}</div>`;

  app.querySelectorAll('.node[data-lesson]').forEach((button) => {
    button.addEventListener('click', () => startLesson(button.dataset.lesson));
  });
}

/* ------------------------------------------------------------------ */
/* Lesson session                                                      */
/* ------------------------------------------------------------------ */

function startLesson(id) {
  const lesson = findLesson(id);
  if (!lesson) return;
  session = {
    lesson,
    index: 0,
    hearts: 5,
    correct: 0,
    mistakes: 0,
    answered: false,
  };
  renderExercise();
}

function renderExercise() {
  session.answered = false;
  const { lesson, index, hearts } = session;
  const exercise = lesson.exercises[index];
  const total = lesson.exercises.length;
  const percent = Math.round((index / total) * 100);
  const heartsHtml = '❤️'.repeat(hearts) + '🖤'.repeat(5 - hearts);

  let inputBlock = '';
  if (exercise.type === 'multiple-choice') {
    inputBlock = `<div class="options">${exercise.options
      .map(
        (option, i) =>
          `<button class="option" data-i="${i}">${escapeHtml(option)}</button>`
      )
      .join('')}</div>`;
  } else if (exercise.type === 'true-false') {
    inputBlock = `<div class="options">
      <button class="option" data-v="true">True</button>
      <button class="option" data-v="false">False</button>
    </div>`;
  } else if (exercise.type === 'fill-blank') {
    inputBlock = `<input id="blank-input" class="blank-input" type="text"
      autocomplete="off" autocapitalize="off" spellcheck="false"
      placeholder="Type your answer" />`;
  }

  app.innerHTML = `
    <div class="lesson-view">
      <div class="lesson-top">
        <button class="quit" id="quit" title="Quit lesson">✕</button>
        <div class="progress"><div class="progress-fill" style="width:${percent}%"></div></div>
        <div class="hearts" id="hearts">${heartsHtml}</div>
      </div>

      <div class="exercise">
        <p class="prompt">${escapeHtml(exercise.prompt)}</p>
        ${exercise.code ? `<pre class="code">${escapeHtml(exercise.code)}</pre>` : ''}
        ${
          exercise.template
            ? `<pre class="code">${escapeHtml(exercise.template)}</pre>`
            : ''
        }
        ${inputBlock}
      </div>

      <div class="feedback" id="feedback"></div>

      <div class="lesson-bottom">
        <button class="btn primary" id="action" disabled>Check</button>
      </div>
    </div>`;

  const action = document.getElementById('action');
  const input = document.getElementById('blank-input');
  let selected = null;

  document.getElementById('quit').addEventListener('click', quitLesson);

  function refreshEnabled() {
    if (exercise.type === 'fill-blank') {
      action.disabled = input.value.trim().length === 0;
    } else {
      action.disabled = selected === null;
    }
  }
  refreshEnabled();

  if (input) {
    input.focus();
    input.addEventListener('input', refreshEnabled);
  }

  app.querySelectorAll('.option').forEach((button) => {
    button.addEventListener('click', () => {
      app
        .querySelectorAll('.option')
        .forEach((other) => other.classList.remove('selected'));
      button.classList.add('selected');
      selected =
        exercise.type === 'true-false'
          ? button.dataset.v === 'true'
          : Number(button.dataset.i);
      refreshEnabled();
    });
  });

  function evaluate() {
    const value = exercise.type === 'fill-blank' ? input.value : selected;
    const correct = isCorrect(exercise, value);

    if (exercise.type === 'fill-blank') {
      input.disabled = true;
      input.classList.add(correct ? 'correct' : 'wrong');
    } else {
      app.querySelectorAll('.option').forEach((button) => {
        button.disabled = true;
        const isAnswer =
          exercise.type === 'true-false'
            ? (button.dataset.v === 'true') === exercise.answer
            : Number(button.dataset.i) === exercise.answer;
        if (isAnswer) button.classList.add('correct');
        else if (button.classList.contains('selected'))
          button.classList.add('wrong');
      });
    }

    showFeedback(correct, exercise);

    if (correct) {
      session.correct += 1;
    } else {
      session.mistakes += 1;
      session.hearts = Math.max(0, session.hearts - 1);
      document.getElementById('hearts').textContent =
        '❤️'.repeat(session.hearts) + '🖤'.repeat(5 - session.hearts);
    }

    session.answered = true;
    action.textContent = 'Continue';
    action.disabled = false;
    action.classList.toggle('danger', !correct);
  }

  action.addEventListener('click', () => {
    if (!session.answered) evaluate();
    else advance();
  });

  function onKey(event) {
    if (event.key !== 'Enter') return;
    if (!action.disabled) {
      event.preventDefault();
      action.click();
    }
  }
  document.addEventListener('keydown', onKey);
  session.cleanupKeys = () => document.removeEventListener('keydown', onKey);
}

function isCorrect(exercise, value) {
  if (exercise.type === 'multiple-choice') return value === exercise.answer;
  if (exercise.type === 'true-false') return value === exercise.answer;
  if (exercise.type === 'fill-blank') {
    const accepted = (exercise.answers || []).map(normalize);
    return accepted.includes(normalize(value));
  }
  return false;
}

function showFeedback(correct, exercise) {
  const feedback = document.getElementById('feedback');
  feedback.className = 'feedback show ' + (correct ? 'correct' : 'wrong');
  feedback.innerHTML =
    `<div class="fb-title">${correct ? 'Correct! 🎉' : 'Not quite 💡'}</div>` +
    `<div class="fb-text">${escapeHtml(exercise.explanation || '')}</div>`;
}

function advance() {
  if (session.hearts <= 0) {
    renderOutOfHearts();
    return;
  }
  session.index += 1;
  if (session.index >= session.lesson.exercises.length) completeLesson();
  else renderExercise();
}

function quitLesson() {
  if (session && session.cleanupKeys) session.cleanupKeys();
  session = null;
  renderHome();
}

/* ------------------------------------------------------------------ */
/* Completion / failure screens                                        */
/* ------------------------------------------------------------------ */

function bumpStreak() {
  const today = dayStr(new Date());
  if (progress.lastDay === today) return;
  const yesterday = dayStr(new Date(Date.now() - 86_400_000));
  progress.streak = progress.lastDay === yesterday ? (progress.streak || 0) + 1 : 1;
  progress.lastDay = today;
}

function completeLesson() {
  if (session.cleanupKeys) session.cleanupKeys();

  const { lesson, correct, mistakes } = session;
  const total = lesson.exercises.length;
  const firstTime = !isCompleted(lesson.id);
  const gained = firstTime ? lesson.xp || 10 : Math.round((lesson.xp || 10) / 2);

  if (firstTime) progress.completed.push(lesson.id);
  progress.xp += gained;
  bumpStreak();
  saveProgress();
  updateHeader();

  const accuracy = total === 0 ? 100 : Math.round((correct / total) * 100);

  app.innerHTML = `
    <div class="center-card">
      <div class="big">🎉</div>
      <h1>Lesson complete!</h1>
      <p>${escapeHtml(lesson.title)}</p>
      <div class="result-grid">
        <div class="result-box"><div class="value">+${gained}</div><div class="label">XP</div></div>
        <div class="result-box"><div class="value">${accuracy}%</div><div class="label">Accuracy</div></div>
        <div class="result-box"><div class="value">🔥 ${progress.streak || 0}</div><div class="label">Streak</div></div>
        <div class="result-box"><div class="value">${mistakes}</div><div class="label">Mistakes</div></div>
      </div>
      <button class="btn" id="continue">Continue</button>
    </div>`;

  document.getElementById('continue').addEventListener('click', () => {
    session = null;
    renderHome();
  });
}

function renderOutOfHearts() {
  if (session.cleanupKeys) session.cleanupKeys();
  const lessonId = session.lesson.id;

  app.innerHTML = `
    <div class="center-card">
      <div class="big">💔</div>
      <h1 class="fail">Out of hearts</h1>
      <p>Don't worry - mistakes are how you learn. Try this lesson again.</p>
      <button class="btn" id="retry">Try again</button>
      <button class="btn danger" id="map">Back to map</button>
    </div>`;

  document.getElementById('retry').addEventListener('click', () => startLesson(lessonId));
  document.getElementById('map').addEventListener('click', () => {
    session = null;
    renderHome();
  });
}

/* ------------------------------------------------------------------ */
/* Export / import / reset                                             */
/* ------------------------------------------------------------------ */

function downloadJson(filename, data) {
  const blob = new Blob([JSON.stringify(data, null, 2)], {
    type: 'application/json',
  });
  const url = URL.createObjectURL(blob);
  const link = document.createElement('a');
  link.href = url;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  link.remove();
  URL.revokeObjectURL(url);
}

function exportData() {
  downloadJson('rust-dojo-progress.json', {
    app: 'rust-dojo',
    version: lessonsData.version || 1,
    exportedAt: new Date().toISOString(),
    progress,
    lessons: lessonsData,
  });
  toast('Exported rust-dojo-progress.json');
}

function importData(file) {
  const reader = new FileReader();
  reader.onload = () => {
    try {
      const parsed = JSON.parse(reader.result);
      const incoming = parsed && parsed.progress ? parsed.progress : parsed;
      if (!incoming || typeof incoming !== 'object') throw new Error('bad shape');
      progress = { ...defaultProgress(), ...incoming };
      progress.completed = Array.isArray(progress.completed)
        ? progress.completed
        : [];
      saveProgress();
      renderHome();
      toast('Progress imported');
    } catch {
      toast('Could not read that JSON file');
    }
  };
  reader.readAsText(file);
}

function resetProgress() {
  if (!confirm('Erase all progress? This cannot be undone.')) return;
  progress = defaultProgress();
  localStorage.removeItem(STORAGE_KEY);
  renderHome();
  toast('Progress reset');
}

/* ------------------------------------------------------------------ */
/* Boot                                                                */
/* ------------------------------------------------------------------ */

function wireGlobalButtons() {
  document.getElementById('btn-export').addEventListener('click', exportData);

  const fileInput = document.getElementById('file-import');
  document.getElementById('btn-import').addEventListener('click', () => {
    fileInput.click();
  });
  fileInput.addEventListener('change', () => {
    if (fileInput.files && fileInput.files[0]) importData(fileInput.files[0]);
    fileInput.value = '';
  });

  document.getElementById('btn-reset').addEventListener('click', resetProgress);
}

async function init() {
  wireGlobalButtons();
  try {
    const response = await fetch('data/lessons.json', { cache: 'no-store' });
    if (!response.ok) throw new Error(String(response.status));
    lessonsData = await response.json();
  } catch {
    app.innerHTML = `
      <div class="error">
        Could not load <code>data/lessons.json</code>.<br />
        Browsers block <code>fetch()</code> on <code>file://</code> pages, so serve
        this folder over HTTP. From the <code>site/</code> directory run:<br /><br />
        <code>python3 -m http.server 8000</code><br /><br />
        then open <code>http://localhost:8000</code>.
      </div>`;
    return;
  }
  renderHome();
}

init();
