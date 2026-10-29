# 開発ガイドライン

## コーディング規約

### 共通

- 文字コードは UTF-8 を使用する
- インデントはスペース 4 つ（Rust）、スペース 2 つ（HTML/CSS/JS）
- 1行の最大文字数は 120 文字を目安とする
- コメントは日本語で記述する（コード上の識別子は英語）

### Rust

- `rustfmt` によるフォーマットを必須とする（`cargo fmt`）
- `clippy` による静的解析を必須とする（`cargo clippy`）
- `unwrap()` / `expect()` の使用は最小限に留め、エラーは `Result` 型で伝播させる
- OS固有コードは `#[cfg(target_os = "windows")]` / `#[cfg(target_os = "macos")]` で分岐する
- 公開APIには `///` ドキュメントコメントを記述する

```rust
// 良い例
fn load_config(path: &Path) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

// 悪い例
fn load_config(path: &Path) -> Config {
    let content = fs::read_to_string(path).unwrap();
    serde_json::from_str(&content).unwrap()
}
```

### JavaScript

- `const` を優先し、再代入が必要な場合のみ `let` を使用する（`var` は使用禁止）
- 非同期処理は `async/await` を使用する（コールバック・Promise チェーンは避ける）
- Tauri APIの呼び出しは `@tauri-apps/api` を使用する

```js
// 良い例
try {
  const result = await invoke('launch_app', { slotId: 0 });
} catch (e) {
  console.error('アプリ起動に失敗しました', e);
}

// 悪い例
invoke('launch_app', { slotId: 0 }).then(result => { ... });
```

- Tauriコマンドの呼び出しは必ず `try/catch` で囲み、エラーを握りつぶさない

### HTML / CSS

- セマンティックなHTML要素を使用する（`<div>` の乱用を避ける）
- CSSクラス名はケバブケース（kebab-case）を使用する
- インラインスタイルは使用しない（`.css` ファイルに記述する）

---

## 命名規則

### Rust

| 対象 | 規則 | 例 |
|---|---|---|
| 変数・関数 | スネークケース | `app_path`, `load_config()` |
| 構造体・列挙型 | パスカルケース | `SlotConfig`, `ActionType` |
| 定数 | スクリーミングスネークケース | `MAX_SLOTS`, `DEFAULT_TRIGGER_KEY` |
| モジュール | スネークケース | `launcher`, `config` |
| ファイル名 | スネークケース | `store.rs`, `windows.rs` |

### JavaScript

| 対象 | 規則 | 例 |
|---|---|---|
| 変数・関数 | キャメルケース | `slotId`, `loadConfig()` |
| 定数 | スクリーミングスネークケース | `MAX_SLOTS` |
| ファイル名 | ケバブケース | `launcher.js`, `settings.js` |

### CSS

| 対象 | 規則 | 例 |
|---|---|---|
| クラス名 | ケバブケース | `.pie-menu`, `.slot-item`, `.slot-item--disabled` |
| CSS変数 | ケバブケース + `--` プレフィックス | `--color-primary`, `--radius-menu` |

---

## スタイリング規約

- CSSはVanilla CSSのみ使用する（外部UIライブラリは使用しない）
- カラー・サイズなどの定数はCSS変数（カスタムプロパティ）で管理する
- パイメニューのアニメーションは `transition` / `transform` を使用し、`top` / `left` のアニメーションは避ける（パフォーマンスのため）
- ダークモード対応は初期実装では不要だが、CSS変数を使用することで将来対応できる構造とする

```css
/* CSS変数の定義例 */
:root {
  --color-bg: rgba(30, 30, 30, 0.85);
  --color-text: #ffffff;
  --color-disabled: #555555;
  --radius-menu: 80px;
  --size-slot: 56px;
}
```

---

## テスト規約

### Rustユニットテスト

- 各モジュールの末尾に `#[cfg(test)]` ブロックでユニットテストを記述する
- テスト対象はビジネスロジック（設定の読み書き、パスのバリデーションなど）を優先する
- OS依存の処理（アプリ起動・フォーカス）はユニットテストの対象外とし、手動テストで確認する

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_has_six_slots() {
        let config = Config::default();
        assert_eq!(config.slots.len(), 6);
    }
}
```

### 手動テスト

自動化が困難な以下の項目は手動でテストする。

| テスト項目 | 確認内容 |
|---|---|
| グローバルショートカット | 各アプリ上からトリガーキーでランチャーが起動するか |
| パイメニュー表示位置 | 画面端近くでも表示が画面内に収まるか |
| アプリ起動 | 未起動アプリが正しく起動するか |
| アプリフォーカス | 起動中アプリが前面に出るか |
| ドラッグ&ドロップ登録 | 実行ファイルをドロップして正しく登録されるか |
| macOS権限案内 | アクセシビリティ未許可時に案内が表示されるか |

---

## Git規約

### ブランチ戦略

```
main          # 安定版・リリースブランチ
└── feature/[機能名]   # 機能開発ブランチ
└── fix/[修正内容]     # バグ修正ブランチ
```

- `main` への直接コミットは禁止（PRを経由する）
- 初期実装フェーズは例外として `main` への直接コミットを許容する

### コミットメッセージ

以下のプレフィックスを使用する。

| プレフィックス | 用途 |
|---|---|
| `feat:` | 新機能の追加 |
| `fix:` | バグ修正 |
| `docs:` | ドキュメントのみの変更 |
| `style:` | フォーマット・スタイルの変更（動作に影響しない） |
| `refactor:` | リファクタリング |
| `chore:` | ビルド設定・依存パッケージの更新など |

```
# 例
feat: パイメニューの表示・非表示アニメーションを追加
fix: 画面端でパイメニューが切れる問題を修正
docs: functional-design.mdにスロット設定フローを追記
```

### コミットの粒度

- 1コミットは1つの目的に限定する
- 動作する状態でコミットする（ビルドが通らない状態でコミットしない）
