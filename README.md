# コンピュータ囲碁ライブラリ「棋理」
「棋理」はRustで書かれたオープンソースのコンピュータ囲碁ライブラリです。

## 現状
碁盤モデルだけです。

### ベンチマーク
19路盤でのランダムロールアウトの速度は、

#### go_rule version 0.1.0

```
cargo bench -p go_rule
test tests::bench_rollout ... bench:   6,910,624 ns/iter (+/- 3,223,550)
```

#### go_rule version 0.1.1

```
cargo bench -p go_rule
test tests::bench_rollout ... bench:   3,245,351 ns/iter (+/- 1,583,076)
```

## 目標
- DCNN + MCTS囲碁思考エンジン
- 読みやすいコード
- 十分な速度
- 拡張しやすいコード
- ブラウザ上での動作
- 誰でも使いやすいライセンス(MIT)

# ライセンス
MIT
