# altare-api

跑在 [Altare](https://altare.gg) 上的小 API 后端，只服务自己。Rust + Axum。

## 本地跑

```sh
cargo run
# 默认监听 0.0.0.0:3000；改端口：PORT=8080 cargo run
curl localhost:3000/health
```

## 部署到 Altare

- 仓库推到 GitHub，控制台 Apps → New app → 选这个仓库，Altare 自动识别 Axum。
- 启动命令在 `Procfile` 里（`web` 那一行）。
- 部署设置 → 健康检查路径填 `/health`。
- 每次 push 到 main 自动部署；构建失败不影响线上旧版。

## 存配置

- **KV**（配置/会话/缓存）：控制台 Storage → KV 建命名空间 → Connect to app，
  注入 `KV_URL`、`KV_NAMESPACE`、`KV_TOKEN`；代码里用 reqwest 调它的 REST 接口。
- **U3 桶**（文件）：Storage 里建桶（选 Asia 区）→ Connect to app，
  注入 `AWS_ENDPOINT_URL_S3`、`AWS_ACCESS_KEY_ID`、`AWS_SECRET_ACCESS_KEY`、`S3_BUCKET`；
  代码里用 `aws-sdk-s3`，记得开 path-style。
- 密钥只走环境变量，不要进仓库；本地用 `.env`（已 gitignore）。

## 工作流

- 写代码 → 推 GitHub → Alice 拉到 InstaBox 跑 `cargo check` / `cargo test` 验证。
