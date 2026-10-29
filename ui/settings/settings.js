const { invoke } = window.__TAURI__.core;
const { open: openDialog } = window.__TAURI__.dialog;
const { getCurrentWebviewWindow } = window.__TAURI__.webviewWindow;

const SLOT_LABELS = ['上 (12時)', '右上 (2時)', '右下 (4時)', '下 (6時)', '左下 (8時)', '左上 (10時)'];

let config = null;
let isCapturingKey = false;

// 設定を読み込んで画面を描画する
async function loadAndRender() {
  try {
    config = await invoke('get_config');
    renderTriggerKey(config.trigger_key);
    renderSlots(config.slots);
  } catch (e) {
    console.error('設定の読み込みに失敗しました', e);
  }
}

// トリガーキーを描画する
function renderTriggerKey(key) {
  document.getElementById('trigger-key-display').textContent = key;
}

// スロット一覧を描画する
function renderSlots(slots) {
  const list = document.getElementById('slot-list');
  list.innerHTML = '';

  for (const slot of slots) {
    const row = createSlotRow(slot);
    list.appendChild(row);
  }
}

// スロット行要素を生成する
function createSlotRow(slot) {
  const row = document.createElement('div');
  row.className = 'slot-row';
  row.dataset.slotId = slot.id;

  const label = document.createElement('span');
  label.className = 'slot-label';
  label.textContent = SLOT_LABELS[slot.id] || `スロット ${slot.id}`;

  const appName = document.createElement('span');
  appName.className = slot.app_name || slot.app_path ? 'slot-app-name' : 'slot-app-name empty';
  appName.textContent = slot.app_name || getFileName(slot.app_path) || '未設定';

  const actions = document.createElement('div');
  actions.className = 'slot-actions';

  // 設定/変更ボタン
  const setBtn = document.createElement('button');
  setBtn.className = 'btn btn--primary';
  setBtn.textContent = slot.app_path ? '変更' : '設定';
  setBtn.addEventListener('click', () => handleSetSlot(slot.id));

  actions.appendChild(setBtn);

  // 削除ボタン（割り当て済みの場合のみ）
  if (slot.app_path) {
    const deleteBtn = document.createElement('button');
    deleteBtn.className = 'btn btn--danger';
    deleteBtn.textContent = '削除';
    deleteBtn.addEventListener('click', () => handleDeleteSlot(slot.id));
    actions.appendChild(deleteBtn);
  }

  row.appendChild(label);
  row.appendChild(appName);
  row.appendChild(actions);

  // ドラッグ&ドロップのビジュアルフィードバック
  row.addEventListener('dragover', (e) => {
    e.preventDefault();
    row.classList.add('drag-over');
  });
  row.addEventListener('dragleave', () => {
    row.classList.remove('drag-over');
  });

  return row;
}

// ファイル選択ダイアログでスロットを設定する
async function handleSetSlot(slotId) {
  try {
    const selected = await openDialog({
      multiple: false,
      filters: [
        { name: 'アプリケーション', extensions: ['exe', 'lnk', 'bat', 'app'] },
        { name: 'すべてのファイル', extensions: ['*'] },
      ],
    });
    if (!selected) return;

    const appPath = typeof selected === 'string' ? selected : selected[0];
    await updateSlot(slotId, appPath);
  } catch (e) {
    console.error('ファイルの選択に失敗しました', e);
  }
}

// スロットを削除する
async function handleDeleteSlot(slotId) {
  try {
    await invoke('delete_slot', { slotId });
    await loadAndRender();
  } catch (e) {
    console.error('スロットの削除に失敗しました', e);
  }
}

// スロットを更新する
async function updateSlot(slotId, appPath) {
  const appName = getFileName(appPath);
  try {
    await invoke('update_slot', {
      slot: {
        id: slotId,
        action_type: 'app',
        app_path: appPath,
        app_name: appName,
        icon_path: null,
      },
    });
    await loadAndRender();
  } catch (e) {
    console.error('スロットの更新に失敗しました', e);
  }
}

// パスからファイル名を取得する
function getFileName(path) {
  if (!path) return '';
  return path.split(/[\\/]/).pop().replace(/\.(exe|lnk|bat|app)$/i, '');
}

// ドラッグ&ドロップでスロットを設定する
async function setupDragDrop() {
  const appWindow = getCurrentWebviewWindow();
  await appWindow.onDragDropEvent(async (event) => {
    if (event.payload.type !== 'drop') {
      // drag-overのビジュアルフィードバックをリセットする
      document.querySelectorAll('.slot-row').forEach((r) => r.classList.remove('drag-over'));
      return;
    }

    const filePath = event.payload.paths[0];
    if (!filePath) return;

    // ドロップ座標からスロット行を特定する
    const { x, y } = event.payload.position;
    const slotRow = getSlotRowAt(x, y);
    if (!slotRow) return;

    const slotId = parseInt(slotRow.dataset.slotId, 10);
    await updateSlot(slotId, filePath);

    document.querySelectorAll('.slot-row').forEach((r) => r.classList.remove('drag-over'));
  });
}

// 座標からスロット行要素を取得する
function getSlotRowAt(x, y) {
  const rows = document.querySelectorAll('.slot-row');
  for (const row of rows) {
    const rect = row.getBoundingClientRect();
    if (x >= rect.left && x <= rect.right && y >= rect.top && y <= rect.bottom) {
      return row;
    }
  }
  return null;
}

// トリガーキー変更
const triggerChangeBtn = document.getElementById('trigger-change-btn');
const triggerHint = document.getElementById('trigger-hint');
const triggerError = document.getElementById('trigger-error');

triggerChangeBtn.addEventListener('click', () => {
  isCapturingKey = true;
  triggerHint.classList.remove('hidden');
  triggerError.classList.add('hidden');
  triggerChangeBtn.textContent = 'キャンセル';
  triggerChangeBtn.onclick = cancelCapture;
});

function cancelCapture() {
  isCapturingKey = false;
  triggerHint.classList.add('hidden');
  triggerError.classList.add('hidden');
  triggerChangeBtn.textContent = '変更';
  triggerChangeBtn.onclick = null;
  triggerChangeBtn.addEventListener('click', startCapture);
}

function startCapture() {
  isCapturingKey = true;
  triggerHint.classList.remove('hidden');
  triggerError.classList.add('hidden');
  triggerChangeBtn.textContent = 'キャンセル';
}

// キー入力を検知してトリガーキーを更新する
document.addEventListener('keydown', async (e) => {
  if (!isCapturingKey) return;
  e.preventDefault();

  if (e.key === 'Escape') {
    cancelCapture();
    return;
  }

  // 修飾キーのみの入力は無効とする
  const modifiers = ['Control', 'Shift', 'Alt', 'Meta'];
  if (modifiers.includes(e.key)) return;

  // 修飾キーが含まれているか確認する
  if (!e.ctrlKey && !e.shiftKey && !e.altKey && !e.metaKey) {
    triggerError.classList.remove('hidden');
    return;
  }

  // キーの組み合わせ文字列を生成する
  const parts = [];
  if (e.ctrlKey) parts.push('Ctrl');
  if (e.shiftKey) parts.push('Shift');
  if (e.altKey) parts.push('Alt');
  if (e.metaKey) parts.push('Super');

  // e.key が " " (スペース) の場合はプラグインが認識できる "Space" に変換する
  // 1文字のキーは大文字化、それ以外は元の名前をそのまま使う
  const keyName = e.key === ' ' ? 'Space' : e.key.length === 1 ? e.key.toUpperCase() : e.key;
  parts.push(keyName);

  const keyStr = parts.join('+');

  try {
    await invoke('update_trigger_key', { key: keyStr });
    renderTriggerKey(keyStr);
    cancelCapture();
  } catch (err) {
    console.error('トリガーキーの更新に失敗しました', err);
    triggerError.textContent = `エラー: ${err}`;
    triggerError.classList.remove('hidden');
  }
});

// 初期化
loadAndRender();
setupDragDrop();
