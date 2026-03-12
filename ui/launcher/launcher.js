const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const RADIUS = 90; // スロット中心からの距離（px）
const SLOT_COUNT = 6;

const menu = document.getElementById('pie-menu');
const settingsButton = document.getElementById('settings-button');

let config = null;
const iconCache = new Map(); // アプリパス → data URL のインメモリキャッシュ

// 設定を読み込みスロットを描画する
async function loadAndRender() {
  try {
    config = await invoke('get_config');
    renderSlots(config.slots);
  } catch (e) {
    console.error('設定の読み込みに失敗しました', e);
  }
}

// スロットを描画する
function renderSlots(slots) {
  const uncachedPaths = [];

  for (let i = 0; i < SLOT_COUNT; i++) {
    const slot = slots[i];
    const el = document.getElementById(`slot-${i}`);
    if (!el) continue;

    // スロット座標を計算する（12時方向から時計回り、60°間隔）
    const angleDeg = -90 + i * 60;
    const angleRad = (angleDeg * Math.PI) / 180;
    const cx = 150; // ウィンドウ中心X
    const cy = 150; // ウィンドウ中心Y
    const x = Math.round(cx + RADIUS * Math.cos(angleRad));
    const y = Math.round(cy + RADIUS * Math.sin(angleRad));

    // translate値をCSSカスタムプロパティとして設定する（hover時のscaleに使用）
    const tx = x - cx;
    const ty = y - cy;
    el.style.setProperty('--tx', `${tx}px`);
    el.style.setProperty('--ty', `${ty}px`);
    el.style.transform = `translate(${tx - 30}px, ${ty - 30}px)`;

    const iconEl = el.querySelector('.slot-icon');
    const labelEl = el.querySelector('.slot-label');

    if (slot.app_path) {
      // 割り当て済みスロット
      el.classList.remove('slot--disabled');
      labelEl.textContent = slot.app_name || getFileName(slot.app_path);

      // キャッシュがあれば即表示、なければフォールバックを表示しておく
      if (iconCache.has(slot.app_path)) {
        iconEl.innerHTML = `<img src="${iconCache.get(slot.app_path)}" alt="" />`;
      } else {
        iconEl.innerHTML = '📦';
        uncachedPaths.push(slot.app_path);
      }
    } else {
      // 未割り当てスロット
      el.classList.add('slot--disabled');
      labelEl.textContent = '';
      iconEl.innerHTML = '';
    }
  }

  // 未キャッシュのアイコンを1回の呼び出しで一括取得する
  if (uncachedPaths.length > 0) {
    invoke('get_app_icons', { appPaths: uncachedPaths }).then((iconsMap) => {
      for (const [path, dataUrl] of Object.entries(iconsMap)) {
        iconCache.set(path, dataUrl);
      }
      // 取得できたアイコンをスロットに反映する
      for (let i = 0; i < SLOT_COUNT; i++) {
        const slot = slots[i];
        if (!slot?.app_path) continue;
        const dataUrl = iconsMap[slot.app_path];
        if (!dataUrl) continue;
        const el = document.getElementById(`slot-${i}`);
        if (!el) continue;
        el.querySelector('.slot-icon').innerHTML = `<img src="${dataUrl}" alt="" />`;
      }
    }).catch(() => {});
  }
}

// パスからファイル名を取得する
function getFileName(path) {
  return path.split(/[\\/]/).pop().replace(/\.(exe|app)$/i, '');
}

// パイメニューを表示する
function showMenu() {
  menu.classList.remove('hidden');
  menu.classList.add('visible');
  loadAndRender();
}

// パイメニューを非表示にする
async function hideMenu() {
  menu.classList.add('hidden');
  menu.classList.remove('visible');
  try {
    await invoke('close_launcher');
  } catch (e) {
    console.error('ウィンドウの非表示に失敗しました', e);
  }
}

// バックエンドからの表示イベントを受信する
listen('launcher-show', () => {
  showMenu();
});

// スロットクリックイベント
document.querySelectorAll('.slot').forEach((el) => {
  el.addEventListener('click', async () => {
    if (el.classList.contains('slot--disabled')) return;

    const slotId = parseInt(el.dataset.slotId, 10);
    try {
      await invoke('launch_app', { slotId });
    } catch (e) {
      console.error('アプリの起動に失敗しました', e);
    }
    // バックエンド側でウィンドウを非表示にするが、UI側も即時リセット
    menu.classList.add('hidden');
    menu.classList.remove('visible');
  });
});

// 設定ボタンクリックイベント
settingsButton.addEventListener('click', async () => {
  // macOS: アプリがフォーカスを持っている間に設定画面を開く（先に hide するとフォーカスが外れる）
  try {
    await invoke('open_settings');
  } catch (e) {
    console.error('設定画面を開けませんでした', e);
  }
  await hideMenu();
});

// Escキーでパイメニューを閉じる
document.addEventListener('keydown', (e) => {
  if (e.key === 'Escape') {
    hideMenu();
  }
});

// ウィンドウがフォーカスを失ったとき（範囲外クリック等）にパイメニューを閉じる
window.addEventListener('blur', () => {
  hideMenu();
});

// 初期化
loadAndRender();
