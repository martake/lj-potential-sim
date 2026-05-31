# LJ Potential Simulator

レナード・ジョーンズ（LJ）ポテンシャルの概念再現シミュレータです。

「原子核のプラスの力場（陽のバリア）」と「電子の自由行動圏（縄張り）」のせめぎ合いを簡易ルールでモデル化し、LJポテンシャルと同様の挙動（近距離で猛反発、中距離で微弱な引力）が創発される様子を視覚的に観察できます。

## 技術スタック

- Rust + egui / eframe
- WebAssembly (trunk でビルド)

## 必要環境

- [Rust](https://rustup.rs/) (1.80+)
- wasm32 ターゲット: `rustup target add wasm32-unknown-unknown`
- [trunk](https://trunkrs.dev/): `cargo install trunk`

## 実行方法

### ネイティブ (デスクトップ)

```bash
cargo run
```

### Web (Wasm)

```bash
trunk serve
```

ブラウザで `http://127.0.0.1:8080` を開いてください。

### プロダクションビルド (Wasm)

```bash
trunk build --release
```

`dist/` ディレクトリに静的ファイルが出力されます。

## 操作方法

左パネルのスライダーでパラメータをリアルタイムに調整できます。

| パラメータ | 説明 |
|---|---|
| Atom Count | 原子の数 (2-30) |
| R_zone | 電子の自由行動圏の半径 |
| R_shield | 陽のバリアの半径 |
| Repulsion Strength | 斥力の強さ |
| Attraction Strength | 引力の強さ |
| Electron Noise | 電子のランダム揺らぎの大きさ |
| Damping | 速度の減衰率 |
| Timestep (dt) | シミュレーションの時間刻み |
