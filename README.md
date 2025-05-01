# TaskBot（Rust製 Discord タスク管理Bot）

⚠️ 注意：このbotはまだ開発中です！⚠️

Rustで開発されたシンプルなタスク管理用Discord Botです。  
ユーザーごとにToDoリストを管理し、以下のような操作が可能です：

- タスクの追加
- タスクの一覧表示
- タスクの完了（削除）
- PingによるBotの稼働チェック

## 機能一覧

| コマンド        | 説明                                   |
|-----------------|----------------------------------------|
| `!ping`         | Botが稼働中か確認。`Pong!`を返します    |
| `!add タスク名` | タスクを追加します                      |
| `!list`         | 追加済みのタスクを一覧表示します        |
| `!done 番号`    | 指定した番号のタスクを完了・削除します  |

---

## 使用技術

- [Rust](https://www.rust-lang.org/)
- [Serenity](https://github.com/serenity-rs/serenity)（Discord Botクレート）
- [Tokio](https://tokio.rs/)（非同期ランタイム）
- [serde / serde_json](https://serde.rs/)（JSON管理）
- [dotenv](https://crates.io/crates/dotenv)（環境変数管理）

---

## セットアップ方法

### 1. リポジトリをクローン

```bash
git clone https://github.com/your-username/taskbot.git
cd taskbot
```

### 2. 必要なクレートをインストール

```bash
cargo build
```

### 3. .env ファイルを作成

Discord Botのトークンを記載します：

```bash
DISCORD_TOKEN=your_discord_token_here
```

### 4. 実行

```bash
cargo run
```

### データの保存

タスクは tasks.json に保存されます

ユーザーごとにIDで分けて記録されています

### 使用イメージ

```bash
User: !add 牛乳を買う
Bot: ✅ タスクを追加しました：「牛乳を買う」

User: !list
Bot:
1. 牛乳を買う

User: !done 1
Bot: ✅ 「牛乳を買う」を完了しました。
```

### 今後の予定（ToDo）

SQLiteへの永続化対応

リマインダー機能

スレッドでの通知

Webダッシュボードとの連携（Rocketなど）

### ライセンス

MIT License