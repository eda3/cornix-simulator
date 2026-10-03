# ToDo

上から順に、`/next-todo` で進める。1項目＝1ループ＋1コミット。`/next-todo` は、【えだ】で始まる項目に来ると止まる。【えだ】の項目は、えだが手を動かして、えだがチェックを入れる（Claude Code に頼むことは、メモに書いてある）。

- 振る舞いの正は `docs/design.md`（以下 design。B-nn と、3節の表）。テストで確かめる中身は `docs/test-items.md`（①〜⑲）。事実の根拠は `docs/research.md`（以下 research）。この3つと食い違うことは書かない。食い違いを見つけたら、直さずに止まって聞く。
- 並べ方は「危ない所から」。1周目で、えだの PC でのキーの届き方（いちばん分かっていない所）を測る。2周目で、レイヤーの決まり（間違えると、動いているように見えたまま結果がずれる所）を、ブラウザなしで固める。
- 周ごとにブランチを切る（名前は、各周の見出しの下）。周の最初の項目に入る前に、そのブランチが無ければ、main の先頭から作って切り替える。周の終わりに、Claude Code が push して PR を作る。合流は、えだが差分を読んで頼む。
- 検証のコマンドは CLAUDE.md の Workflow。`cornix_web` を変えた項目は、wasm32 向けの clippy と、wasm-pack のビルドも通す。
- 決まっていない振る舞いに当たったら、推測で埋めずに、質問一覧だけ返して止まる。
- 依存クレートの版は、crates.io の最新の安定版を選ぶ（2026-10-03 の版は research 5.1節）。

## 1周目: 手元のキーの届き方を測る

ゴール: えだの PC で、どのキーがどう届くかが `docs/measurements.md` に書いてあり、下段の対応表と JIS の文字の表が design で決まっている。

ブランチ: `stage1-keytest`

- [x] workspace の骨組みと、計測ページ（keytest）を作り、①の自動の部分を満たす（B-46〜B-52）
  - メモ: ルートの `Cargo.toml` は `[workspace]` だけ（`members = ["cornix_logic", "cornix_web"]`、`resolver = "3"`）。edition は 2024。`cornix_logic` は lib で、まだ中身は空でよい（`//!` の説明だけ。serde_json は、まだ入れない）。`cornix_web` は `crate-type = ["cdylib", "rlib"]` で、`cornix_logic` に依存する。wasm-bindgen・web-sys・js-sys は `[target.'cfg(target_arch = "wasm32")'.dependencies]` に入れる（research 5.1節の、試した形）。まとめの表を作る部分（イベントの並び → 行の並びと、直近の20件）は、web-sys を使わないモジュールに分けて、`cargo test` で確かめる。DOM とイベントの配線は `#[cfg(target_arch = "wasm32")]` のモジュールに置き、`#[wasm_bindgen] pub fn run_keytest()` を出す。`static/keytest.html` は手書きで、`./pkg/cornix_web.js` を読み込んで `run_keytest()` を呼ぶ（`index.html` は、まだ作らない）。全画面と Keyboard Lock の呼び方は、web-sys に型が無ければ `js_sys::Reflect` で呼ぶ。通った方法を、design 6節の M-8 に書き戻す。`.gitignore` に `/static/pkg/` を足す
  - 足してよい依存: wasm-bindgen・web-sys・js-sys（`cornix_web`。target が wasm32 のときだけ）
  - 完了条件: Workflow のコマンド（wasm32 向けの clippy と、wasm-pack のビルドを含む）が全部通り、`static/pkg/` に `cornix_web.js` と `cornix_web_bg.wasm` が出る。ブラウザで開いて確かめるのは、次の【えだ】
- [ ] 【えだ】計測ページで、手元のキーの届き方を測る（①の実測。`docs/measurements.md` の①〜④）
  - メモ: 手順。
    1. `wasm-pack build cornix_web --target web --out-dir ../static/pkg --no-typescript`
    2. `python -m http.server 8000 --directory static` を動かしたまま、Chrome で `http://localhost:8000/keytest.html` を開く
    3. keyhac を動かしたまま、`docs/measurements.md` の①の「押すキー」を、上から順に、1つずつ押して離す。ページの下に出る Markdown の表を、①の「keyhac を動かしたまま」に貼る
    4. 「記録を消す」を押し、keyhac を止めて、3 と同じことをする。「keyhac を止めたとき」に貼る
    5. ②（止められるか）・③（全画面）・④（JIS の文字）を、表のとおりに試して、書く。ふだん使う状態（keyhac を動かしたまま）で測り、止めたときと違う所があれば、メモに書く。Ctrl+W・Ctrl+T・Ctrl+N は、③の全画面のときだけ押す（ふつうの窓では、タブが閉じたり開いたりする）
    6. ブラウザの版・IME・キーボードの型番も書く
- [ ] 【えだ】下段の対応表と、JIS の文字の表を決める（design 3.2節・3.4節。design 6節の M-1〜M-7）
  - メモ: Claude Code に「`docs/measurements.md` の①〜④を読んで、design 3.4節の決め方で、下段の対応表の案を出して」と頼む。えだが選んだら、Claude Code が次を行う（文書だけを変える。コミットは1つ）。(a) design 3.4節に、下段の表を足す。上の3段で、届き方が怪しかったキーは外す (b) design 3.2節の JIS の列を、④の実測に合わせる (c) B-43 の注意の文を、②③の実測に合わせる (d) design 6節の M-1〜M-7 を、分かったことで書き直す (e) 見本（design 3.5節）の MO(1)・MO(2)・Space・Enter・Bksp の位置に、手元のキーが当たらないときは、見本と対応表のどちらを直すかを、えだに聞く
- [ ] 【えだ】PR の差分を読んで、合流を頼む

## 2周目: ロジック（cornix_logic）

ゴール: `cargo test -p cornix_logic` で ②〜⑬ が全部通る。wasm-bindgen・web-sys に依存しない。

ブランチ: `stage2-logic`

- [ ] `keycode` を作り、②を満たす（design 3.1節）
  - メモ: 文字列（か整数）を受けて、種類を返す。読めない入力は無い（全部、どれかの種類になる）。未対応は、元の文字列を保つ。表は design 3.1節から手で写す。図に出す「名前」（Esc・Bksp など）も、ここで持つ
- [ ] 見本のキーマップ（テスト用の自作の .vil）を置き、`vil` の読める形を作り、③を満たす（B-09・design 3.5節）
  - メモ: design 3.5節の全文を、そのまま `cornix_logic/assets/sample.vil` に写す（1文字も変えない。改行は LF）。`sample` モジュールが `include_str!` で埋め込む。`vil` は、`serde_json::Value` で受けて、自分で形を確かめる（uid は読まない）。BOM と前後の空白は、読む前に取り除く。テスト用の .vil を組み立てる手伝いの関数（位置とキーコードを渡すと、.vil の文字列を返す物）を、テストの側に作る。公式の .vil を使うテストは、`local/cornix-default-keymap.vil` が無ければ、何もせずに通る形にする
  - 足してよい依存: serde_json（`cornix_logic`）
- [ ] `vil` の読めない形と、知らせの数を作り、④⑤を満たす（B-08〜B-10・design 3.6節）
  - メモ: エラーは自前の型（`VilError`）。種類と場所を持ち、表示用の文（`Display`）は design 3.6節の文と同じにする。最初に見つけた1つだけを返す
- [ ] `chars` を作り、⑨を満たす（B-37・design 3.2節）
  - メモ: 1周目の【えだ】の項目で直したあとの、design 3.2節の表から手で写す。配列（JIS・US）は、引数で受ける。`engine` より先に作るのは、`engine` の「押す」が、最初から文字まで返せるようにするため（あとから返す物を変えずに済む）
- [ ] `engine` の「押す」「離す」を作り、⑥⑦を満たす（B-25〜B-35・B-37〜B-39・B-41・design 3.7節）
  - メモ: 位置ごとの「押す」「離す」を受ける。「押す」は、配列を引数で受けて、決まったキーコード・欄への動き（文字を足す・1文字消す・なし）・「直前のキー」の文を返す。有効なレイヤーは、レイヤーごとの真偽で持つ（数えない）。修飾は、8つの真偽で持つ（数えない）。押したときに決まったキーコードを、位置ごとに覚える。「いま押したら決まるキーコード」を読む関数も、ここで作る（⑬で使う）。押し続けと全部離すは、次の次の項目（⑧）
- [ ] 点検: test-reviewer に点検させ、指摘を壊し方で確かめて直す（最大2回）
- [ ] `engine` の押し続け・二重の押す離す・全部離すを作り、⑧を満たす（B-22〜B-24）
  - メモ: 「押し続け」と「全部離す」を足す。二重の押す・離すの無視（B-22）は、「押す」「離す」の入口に足す（すでに通っているテストの結果は変わらない）
- [ ] `text` を作り、⑩を満たす（B-38・B-40）
- [ ] `hostmap` と `geometry` を作り、⑪⑫を満たす（design 3.3節・3.4節）
  - メモ: どちらも、design の表から手で写した定数。`hostmap` は、1周目で決めた下段の行まで入れる（design 3.4節の下段が「未定」のままなら、止まって聞く）
- [ ] `view` を作り、⑬を満たす（B-12〜B-19）
  - メモ: `engine` の状態・配列・表示するレイヤー（自動か番号）・手元のキーの名前を出すかを受けて、位置ごとの「図に出す中身」と、有効なレイヤーの番号を返す。DOM には触らない
- [ ] 点検: test-reviewer に点検させ、指摘を壊し方で確かめて直す（最大2回）
- [ ] 【えだ】PR の差分を読んで、合流を頼む

## 3周目: 画面（cornix_web）

ゴール: ブラウザでページを開いて、(1) .vil を選ぶと、その内容が図に出る (2) 手元のキーボードで打つと、対応するキーが光る (3) MO を押している間は、その面の表示に切り替わる (4) 出た文字が欄に並ぶ。⑭〜⑱が通る。

ブランチ: `stage3-web`

この周の実装の項目は、画面の配線だけを作る。見え方（`docs/test-items.md` の⑭〜⑱）は、えだがブラウザで見る項目なので、実装の項目では自動テストを書かない（判断を `cornix_logic` に足したときだけ、そこにテストを足す）。どの項目も、完了条件は「Workflow のコマンド（wasm32 向けの clippy と、wasm-pack のビルドを含む）が全部通る」。見え方の合格は、周の終わりの【えだ】の項目で確かめる。

- [ ] 配列の図を描く（見本のキーマップ。B-01〜B-04・B-11・B-12）
  - メモ: `static/index.html` を手書きで作り、`cornix_web::app`（wasm32 のときだけ）が、`view` の返す中身を描く（SVG を勧める）。入口は `run_simulator()`。keytest の入口（`run_keytest()`）は、そのまま残す。ページの中のパスは相対（B-54）。色と大きさは、実装で決めてよい。判断は `cornix_logic` に置く（足りない判断が見つかったら、`cornix_logic` に足して、テストも足す）
- [ ] 手元のキーで打てるようにする（B-13・B-14・B-17・B-19〜B-41）
  - メモ: keydown・keyup と、窓がフォーカスを失ったとき・ページが隠れたときを受ける。対応表にあるキーだけ、preventDefault する。ボタンを押したあとは、フォーカスをボタンから外す。修飾キーの札・「直前のキー」・欄・「全部消す」・「キーを全部離す」・有効なレイヤーの番号も、ここで出す
- [ ] .vil を選べるようにする（B-05〜B-08）
  - メモ: ファイルは、文字列として読む（FileReader の readAsText）。JavaScript の側で JSON にしない。読めたときの案内と知らせ、読めないときの理由を出す
- [ ] 切り替えを付ける（B-15・B-16・B-18・B-36）
  - メモ: 配列（JIS・US）、「手元のキーの名前」のチェック、レイヤーの列（「自動」と番号）
- [ ] 全画面と、注意の文を付ける（B-42〜B-45）
  - メモ: 1周目の計測ページで通った方法（design 6節の M-8）で、全画面と Keyboard Lock を呼ぶ。全画面を抜けたとき（fullscreenchange）に、Keyboard Lock を外して、全部離す
  - 足してよい依存: wasm-bindgen-futures（Keyboard Lock の結果を待つのに要るときだけ。要らなければ足さない）
- [ ] 点検: test-reviewer に点検させ、指摘を壊し方で確かめて直す（最大2回）
  - メモ: この周で `cornix_logic` に足した判断があれば、そこが中心
- [ ] 【えだ】ブラウザで⑭〜⑱を見て、結果を `docs/measurements.md` の⑤に書く
  - メモ: 通らなかった番号と、見えたことを書く。直しは、この下に項目を足す（1件1項目）
- [ ] 【えだ】PR の差分を読んで、合流を頼む

## 4周目: 公開する

ゴール: `https://eda3.github.io/cornix-simulator/` で開けて、⑲が通る。

ブランチ: `stage4-pages`

- [ ] README を、実物に合わせて直す
  - メモ: 「予定」の断り書きを外す。使い方・手元で動かす手順・対象のブラウザ・できること／できないことを、実物に合わせる。「非公式」と、座標の出どころ・「メーカーの許可は未確認」は残す。LICENSE は、えだが決めるまで作らない
  - 完了条件: README.md だけが変わっている
- [ ] GitHub Actions の workflow を作る（B-53）
  - メモ: `.github/workflows/pages.yml`。main への push と、手動（workflow_dispatch）で動く。流れは、checkout → `rustup target add wasm32-unknown-unknown` → wasm-pack を入れる（公式のリリースの `wasm-pack-v0.15.0-x86_64-unknown-linux-musl.tar.gz` を curl で取って展開する。版は固定）→ `cargo test --workspace` → `wasm-pack build cornix_web --target web --out-dir ../static/pkg --no-typescript` → `actions/configure-pages` → `actions/upload-pages-artifact`（path は `static`）→ `actions/deploy-pages`。使う Action は、公式の actions/* だけ（版は research 5.2節）。権限は `contents: read`・`pages: write`・`id-token: write`。テストが落ちたら、その先へ進まない
  - 完了条件: workflow のファイルがある。実際に通るかは、次の【えだ】
- [ ] 【えだ】Pages の設定をして、公開する
  - メモ: GitHub の Settings → Pages → Build and deployment の Source を「GitHub Actions」にする。PR の差分を読んで、合流を頼む。Actions の実行が成功するのを見る。失敗したら、ログの先頭20行を `docs/measurements.md` の⑥に貼って、Claude Code に直させる
- [ ] 【えだ】公開ページで、⑲を確かめる
  - メモ: 結果を `docs/measurements.md` の⑥に書く

最初の版のあとの候補は、design の7節にある（ここには、項目として置かない）。
