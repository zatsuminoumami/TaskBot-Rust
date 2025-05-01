use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::{collections::HashMap, env, fs};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug, Default)]
struct UserTasks {
    tasks: HashMap<u64, Vec<String>>, // user_id: [task1, task2]
}

impl UserTasks {
    fn new() -> Self {
        if let Ok(data) = fs::read_to_string("tasks.json") {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            UserTasks {
                tasks: HashMap::new(),
            }
        }
    }

    fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self) {
            let _ = fs::write("tasks.json", json);
        }
    }

    fn add_task(&mut self, user_id: u64, task: String) {
        self.tasks.entry(user_id).or_default().push(task);
        self.save();
    }

    fn list_tasks(&self, user_id: u64) -> Vec<String> {
        self.tasks.get(&user_id).cloned().unwrap_or_default()
    }

    fn remove_task(&mut self, user_id: u64, index: usize) -> Option<String> {
        if let Some(tasks) = self.tasks.get_mut(&user_id) {
            if index < tasks.len() {
                let removed = tasks.remove(index);
                self.save();
                return Some(removed);
            }
        }
        None
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let mut tasks = UserTasks::new();
        let user_id = msg.author.id.0;
        let content = msg.content.trim();

        if content == "!ping" {
            let _ = msg.channel_id.say(&ctx.http, "Pong!").await;
        } else if content == "!add" {
            let _ = msg.channel_id.say(&ctx.http, "⚠️ タスク内容を入力してください。例: `!add 牛乳を買う`").await;
        } else if content.starts_with("!add ") {
            let task = content[5..].trim();
            if !task.is_empty() {
                tasks.add_task(user_id, task.to_string());
                let _ = msg.channel_id.say(&ctx.http, format!("✅ タスクを追加しました：「{}」", task)).await;
            } else {
                let _ = msg.channel_id.say(&ctx.http, "⚠️ タスク内容を入力してください。例: `!add 勉強する`").await;
            }
        } else if content == "!list" {
            let user_tasks = tasks.list_tasks(user_id);
            if user_tasks.is_empty() {
                let _ = msg.channel_id.say(&ctx.http, "タスクはありません。").await;
            } else {
                let list = user_tasks.iter().enumerate()
                    .map(|(i, task)| format!("{}: {}", i + 1, task))
                    .collect::<Vec<_>>()
                    .join("\n");
                let _ = msg.channel_id.say(&ctx.http, format!("📋 あなたのタスク一覧：\n{}", list)).await;
            }
        } else if content.starts_with("!done ") {
            if let Ok(index) = content[6..].trim().parse::<usize>() {
                match tasks.remove_task(user_id, index - 1) {
                    Some(task) => {
                        let _ = msg.channel_id.say(&ctx.http, format!("✅ タスク「{}」を完了しました。", task)).await;
                    }
                    None => {
                        let _ = msg.channel_id.say(&ctx.http, "⚠️ 指定された番号のタスクが見つかりません。").await;
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} が起動しました！", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKENが.envに設定されていません");

    let mut client = Client::builder(&token, GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT)
        .event_handler(Handler)
        .await
        .expect("クライアント作成に失敗しました");

    if let Err(why) = client.start().await {
        println!("Botエラー: {:?}", why);
    }
}
