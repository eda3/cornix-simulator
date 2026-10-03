# 調べた事実（research）

実装の前に調べた事実の置き場。設計（`docs/design.md`）の土台にする。調べた日は 2026-10-03。調べたのは Claude Code（えだの PC の上。Windows 11 Home）。

## 読み方

- 確度: [A] Claude Code が現物で確かめた／[B] 現物からの推論か記憶／[B・チャット側の報告] チャット側の Claude（別の環境）が確かめたと報告し、Claude Code は照合していない／[未確認]。えだから聞いたことは（えだ発言）と書く。
- 出どころは、0節の番号（S1〜）と、ファイルのパス・行で書く。行の番号は、0節に書いた版のもの。
- 他人のリポジトリ・公式のファームウェア・Web ページは、リポジトリの外の一時フォルダに取って読んだ（一時フォルダは残らない。取り直すときは、0節の URL と版を使う）。Web ページの本文は curl で取った。
- 公式の .vil と公式の画像は `local/` にある（git の管理の外）。公式の .vil についてここに書くのは、項目・数・種類と、設計に要る位置まで。レイヤーの中身の全部は、`local/` の現物で見る。
- cornix-studio の定義ファイル（S3）・公式ファームウェアの中の機種定義（S8）・ZMK の定義（S10）についてここに書くのは、「何が入っているか」まで。座標と角度の数値は書いていない（扱いは Q-10 でえだが決める）。
- 位置は (行,列) で書く。行0〜3が左手、行4〜7が右手。

## 0. 出どころの一覧

| 番号 | 物 | 場所 | 版・取得 |
|---|---|---|---|
| S1 | 公式「Cornix 日本語マニュアル」 | https://docs.channel.io/jezailfunderjp/ja/articles/Cornix-%E6%97%A5%E6%9C%AC%E8%AA%9E%E3%83%9E%E3%83%8B%E3%83%A5%E3%82%A2%E3%83%AB-c1160246 | 2026-10-03 取得。HTML 180,434 バイト |
| S2 | 公式「Cornix はじめてガイド」 | https://docs.channel.io/jezailfunderjp/ja/articles/Cornix-%E3%81%AF%E3%81%98%E3%82%81%E3%81%A6%E3%82%AC%E3%82%A4%E3%83%89-d375d34a | 2026-10-03 取得。HTML 104,168 バイト |
| S3 | cornix-studio（非公式の設定アプリ） | https://github.com/Ponkan230/cornix-studio | commit 7732b62（2026-08-18）。GPL-2.0-or-later |
| S4 | W3C「UI Events KeyboardEvent code Values」 | https://www.w3.org/TR/uievents-code/ | 22 April 2025 の版。2026-10-03 取得 |
| S5 | KBD.news のレビュー | https://kbd.news/Cornix-review-2715.html | 2025-08-29 公開。2026-10-03 に全文を取得 |
| S6 | 公式「Cornix ファームウェア」 | https://docs.channel.io/jezailfunderjp/ja/articles/Cornix-%E3%83%95%E3%82%A1%E3%83%BC%E3%83%A0%E3%82%A6%E3%82%A7%E3%82%A2-bf1534b6 | 2026-10-03 取得。HTML 74,948 バイト |
| S7 | RMK（キーボード用のファームウェア。Rust 製） | https://github.com/HaoboGu/rmk | タグ rmk-v0.8.3＝commit 8557c8f（2026-08-29）。比べた版: rmk-v0.7.8（2025-07-23）・rmk-v0.8.0（2025-11-25）・rmk-v0.8.1（2025-11-29）・rmk-v0.8.2（2025-12-18）・main f963675（2026-10-03） |
| S8 | 公式ファームウェア V1.12.zip（S6 の添付） | https://cf.channel.io/document/spaces/16010/articles/541501/revisions/1203467/usermedia/69a59709a945a0e6287f | 545,595 バイト。SHA-256 ef5f12056afc932e10dac85b1909defaf8bf2b334c51114afc38e422bc857c88 |
| S9 | vial-gui（Vial の本体） | https://github.com/vial-kb/vial-gui | commit aef8222（2026-05-25）。GPL（COPYING） |
| S10 | zmk-keyboard-cornix（Cornix 用の ZMK の定義。第三者の物） | https://github.com/hitsmaxft/zmk-keyboard-cornix | commit b2d5712（2026-08-15）。Apache-2.0（LICENSE） |
| S11 | MDN | https://github.com/mdn/content と https://github.com/mdn/browser-compat-data の main | 2026-10-03 取得 |
| S12 | Keyboard Lock の仕様と、Chrome の解説 | https://wicg.github.io/keyboard-lock/ ・ https://developer.chrome.com/docs/capabilities/web-apis/keyboard-lock | 2026-10-03 取得 |
| S13 | Chromium のソース | https://github.com/chromium/chromium の main | 2026-10-03 取得（そのときの先頭は 7274805） |
| S14 | Firefox のソース | https://github.com/mozilla-firefox/firefox の main | 2026-10-03 取得 |
| S15 | jetkvm/kvm の Issue #740（他人の観測の報告） | https://github.com/jetkvm/kvm/issues/740 | 2025-08-17 起票。コメントは 2025-12-10 まで |
| S16 | GitHub Docs（Pages） | https://github.com/github/docs の main、`content/pages/getting-started-with-github-pages/` | 2026-10-03 取得 |
| S17 | GitHub と crates.io の API | api.github.com ・ crates.io/api/v1 | 2026-10-03 取得 |
| S18 | えだの手元 | `C:\Users\eda3\workspace\edaberu_20260926` ・ `C:\Users\eda3\workspace\multiplay_minesweeper2_20260919` ・ `C:\Users\eda3\.claude\skills\next-todo\SKILL.md` ・ `C:\Users\eda3\.claude\agents\test-reviewer.md` | 2026-10-03 に読んだ |

`local/` に取った物（2026-10-03）:

| ファイル | 取った元 | バイト数 | SHA-256 |
|---|---|---|---|
| `local/cornix-default-keymap.vil` | S1 の「初期キーマップ」の添付。https://cf.channel.io/document/spaces/16010/articles/541498/revisions/942375/usermedia/6958e9940227a7198545 | 8,824 | f85dd13d58398ea53e29f3fbab88d07b1f86ba09982e7eaac5d36eb314a327ab |
| `local/cornix-outline.png`（外形図。1160×443） | S1 の画像。https://cf.channel.io/document/spaces/16010/usermedia/6958e857b32feab90e43 | 160,588 | 6669e2d4e91ce5fd04a44e64a279d90ee739e8945c46da75cd52f6486d470a85 |
| `local/cornix-vial-layer0.png`（Vial の画面写真。1088×959） | S2 の画像。https://cf.channel.io/document/spaces/16010/articles/590739/revisions/1091819/usermedia/6984be3b3437d382c2cf | 148,123 | a3ecc02f29b8004635e373ba4ad551cf753a11e0e3b4727c66b6aa72aa203e22 |

## 1. 前提にした決定（D-1〜D-12）

調査と質問一覧が前提にした決まり。準備の指示文はリポジトリに残らないので、ここに控える。

えだが決めたこと:
- D-1 リポジトリ名は cornix-simulator。
- D-2 言語は Rust。ブラウザ側は WASM。
- D-3 キーマップは、Vial の保存ファイル（.vil）をページで読み込んで使う。公式の初期キーマップと、Cornix の持ち主が Vial で保存した .vil の両方を読める形にする。
- D-4 GitHub Pages で公開する（静的なページだけで動かす）。
- D-5 進め方は、`docs/todo.md` を上から `/next-todo` で1項目ずつ進め、区切りで test-reviewer に点検させる形。

チャット側の案（段1の時点で、えだは未判断）:
- D-6 1周目のゴール: (1) .vil を選ぶと、その内容が画面の Cornix の配列に出る (2) 手元のキーボードで打つと、対応するキーが光る (3) MO のキーを押している間は、その面の表示に切り替わる (4) 出た文字が画面の欄に並ぶ。
- D-7 crate は2つ。ロジック（.vil の読み取り・手元のキーとの対応づけ・レイヤーの状態・文字への変換。ブラウザなしで動き、cargo test で確かめる）と、画面（WASM）。
- D-8 手元のキーと Cornix のキーの対応は「物理的な位置が同じキー」を原則にし、KeyboardEvent.code で見分ける。対応表は設定として外に出す。初期値は日本語配列（JIS）のキーボード向け。下段の当て方は、1周目の先頭の実測のあとで、えだに確かめて決める。
- D-9 ページと README に「非公式」と書く。
- D-10 1周目で動かすキーコード: 文字・数字・記号のキー、Enter・Space・Backspace・Tab・Esc・Delete・矢印、修飾キー（Shift・Ctrl・Alt・GUI）、MO(n)、KC_TRNS、KC_NO。
- D-11 1周目の範囲の外: LT・MT など長押しと短押しで役割が変わるキー、MO 以外のレイヤー切り替え（TG・TO・OSL など）、コンボ、タップダンス、マクロ、Bluetooth の切り替え（USERnn）、マウスと音量のキー、エンコーダー、練習文と計測、日本語入力（IME）。範囲の外のキーコードは、画面で「未対応」と分かる形で表示し、押したときの出力は空にする。長押しとコンボを1周目の範囲の外に置くことは、えだが確認した（えだ発言・2026-10-03 14:52「一発目はなしでも大丈夫」）。
- D-12 レイヤーの決まり（根拠は 3.4節）:
  - (1) キーは、押した瞬間に有効なレイヤーのうち、番号がいちばん大きいレイヤーのキーコードで決まる。
  - (2) そのキーコードが KC_TRNS のときは、その下の有効なレイヤーを見る。KC_NO のときは、そこで止まって、出力は空にする。
  - (3) 離したときは、押したときに決まったキーコードで処理する（押している間にレイヤーが変わっても、そのまま）。
  - (4) MO を2つ同時に押しているときも、(1) のとおり、番号の大きいレイヤーが先に見られる（tri_layer は使われていない前提）。

## 2. R-1 公式の .vil

### 2.1 取得と照合

- 2026-10-03 15:15、S1 の直リンクから curl で `local/cornix-default-keymap.vil` に取った。HTTP 200・application/json・8,824 バイト・SHA-256 は f85dd13d…（0節の表）。準備の指示文の値（F-1）と一致 [A]。
- 直リンクは S1 の HTML の中にある [A]。S6 にも同じ名前の添付があり、取ると 8,824 バイトで SHA-256 も同じ [A]。
- 先に `.gitignore`（`local/` と `target/`）を作ってから取った。`git check-ignore` で、`local/` が管理の外になっているのを確かめた [A]。
- 取ったあと、15:17:12 に、手元の写しの末尾に CRLF（2バイト）が足されて 8,826 バイトになっていた。Claude Code がしたのは読み取りだけ。配布元を取り直すと 8,824 バイトのままだったので、手元の写しを取り直した物に戻した。その後は変わっていない（15:52 に再確認）[A]。足した物は [未確認]。そのとき Zed が動いていた [A・tasklist]。Zed で開いて保存すると末尾に改行が足される、という可能性がある [B]。
- 設計への帰結: 末尾に空白や改行が足された .vil も読める形にする（JSON の読み取りは、ふつう末尾の空白を許す。serde_json で確かめた [A・5.1節]）。公式の .vil を使うテストは、ハッシュの一致を条件にしない。

### 2.2 中身（項目・数・種類）

- 1行（改行なし）。ASCII だけ。BOM なし。JSON [A]。
- 最上位の項目は13個。順に version・uid・layout・encoder_layout・layout_options・macro・vial_protocol・via_protocol・tap_dance・combo・key_override・alt_repeat_key・settings [A]。この順番と、1行で区切りが「, 」「: 」になる書き方は、vial-gui の保存の処理（S9 `src/main/python/protocol/keyboard_comm.py` 369〜407行。`json.dumps`）と合う [A]。
- version は 1、layout_options は 0、vial_protocol は 6、via_protocol は 9 [A]。
- uid は 16882930253541522617（16進で ea4c379db209bcb9）。符号つき64ビットの上限 9223372036854775807 を超え、符号なし64ビットには収まる [A]。
- layout は 10レイヤー × 8行 × 7列 ＝ 560個の値。文字列が 500個、整数が 60個で、整数は全部 -1 [A]。
- -1 の位置は、どのレイヤーも (0,6) (1,6) (3,6) (4,6) (6,6) (7,6) の6か所。キーのある位置は 50 [A]。-1 は、vial-gui が「機種定義に無い位置」に入れる値（S9 同ファイル 382行）[A]。
- キーコードの文字列は 69種類 [A]。内訳:
  - KC_NO（410個）
  - 英字 26種類（KC_A〜KC_Z）、数字 10種類（KC_0〜KC_9）
  - 記号 8種類: KC_MINUS・KC_EQUAL・KC_BSLASH・KC_SCOLON・KC_QUOTE・KC_COMMA・KC_DOT・KC_SLASH
  - KC_ENTER・KC_ESCAPE・KC_BSPACE・KC_TAB・KC_SPACE（2個）・KC_DELETE、矢印 4種類（KC_UP・KC_DOWN・KC_LEFT・KC_RIGHT）
  - KC_CAPSLOCK
  - 修飾キー 4種類: KC_LSHIFT・KC_LCTRL・KC_LALT・KC_LGUI（右側の修飾キーは無い）
  - MO(1)・MO(2)・MO(3)・MO(4)
  - USER00・USER01・USER02（各2個）、KC_MUTE（10個）、KC_BTN3（10個）
- KC_TRNS は 0個。レイヤーを切り替えるキーコードは MO(1)〜MO(4) の4つだけ [A]。
- encoder_layout は 10レイヤー × 2個 × 2方向。全レイヤーで [KC_VOLD, KC_VOLU] と [KC_WH_U, KC_WH_D] [A]。
- macro は空の配列が 32個。tap_dance は ["KC_NO","KC_NO","KC_NO","KC_NO",250] が 32個。combo は KC_NO 5つの配列が 32個。key_override と alt_repeat_key は空の配列 [A]。
- settings は {"2":50,"6":1000,"7":250,"18":20,"19":20,"22":1,"23":0,"26":1,"27":120} [A]。番号の意味は、vial-gui の定義（S9 `src/main/resources/base/qmk_settings.json`）で、2＝コンボの待ち時間、6＝ワンショットの時間、7＝Tapping Term、18＝Tap Code Delay、19＝Tap Hold Caps Delay、22＝Permissive Hold、23＝Hold On Other Key Press、26＝Chordal Hold、27＝Flow Tap [A]。どれも長押し・コンボ・ワンショットの設定で、1周目の範囲の外（D-11）。

### 2.3 設計に要る位置

- 行0〜3が左手、行4〜7が右手。右手は列0が外側。Vial の画面写真（Layer 0）の右手の並びと、.vil の行4〜7 を突き合わせて確かめた（行4は、列0 が Backspace の位置、列5 が Y の位置）[A]。
- レイヤー0: MO(1) は (3,3)、MO(3) は (3,4)、MO(2) は (7,3)、MO(4) は (7,4)。左 Shift は (2,0)。KC_CAPSLOCK は (1,0)。左手の下段の外側3つ (3,0)(3,1)(3,2) は Ctrl・GUI・Alt。Space は (3,5) と (7,5) [A]。
- レイヤー1: 上の段（行0 と行4）に Esc・数字・Delete。記号は (1,4) (1,5) (5,4) (5,5) の4か所。そのほかは KC_NO。MO(1) の位置 (3,3) と、左 Shift の位置 (2,0) も KC_NO [A]。
- レイヤー2 とレイヤー3（中身は同じ）: (1,0) USER00、(2,0) USER01、(3,0) USER02。そのほかは KC_NO [A]。
- レイヤー4〜9: (2,6) と (5,6) のほかは、全部 KC_NO [A]。
- (2,6) は全レイヤーで KC_MUTE、(5,6) は全レイヤーで KC_BTN3 [A]。

### 2.4 初期キーマップで打てる記号と、打てない記号

- 記号のキーコードは 2.2節の8種類だけで、KC_LBRACKET・KC_RBRACKET・KC_GRAVE・KC_RO・KC_JYEN は無い [A]。
- PC の配列が JIS のとき、`` @ ` [ { \ _ ¥ | `` の8文字が打てない。US のとき、`` [ ] { } ` ~ `` の6文字が打てない [B・配列の知識と 2.2節の種類から。文字の対応は 4.4節]。
- つまり、初期キーマップのままでは記号が足りず、実際に使う人は割り当てを足す。公式ガイド（S2）も、割り当てを足す手順を案内している（3.6節）[A]。

## 3. R-2 キーの数と並び、cornix-studio、レイヤーの動き

### 3.1 外形図と Vial の画面（目で見た）

- 外形図（`local/cornix-outline.png`）: 片手が「3段×6列＝18キー」＋「外側3列の下にもう1段＝3キー」＋「親指3キー」の 24キー。左右で 48キー。左右とも、内側の3段目の高さに、丸い部品が1つ [A・図を見て数えた]。
- 外形図には、親指キーのあたりに色の付いた点が4つ（左に赤と青、右に緑と橙）ある。S1 の本文に「左右それぞれに 2 つの RGB インジケーターを搭載」とあり [A]、点はインジケーターの位置と読める [B]。
- Vial の画面写真（`local/cornix-vial-layer0.png`）: 48キーに加えて、左の内側に「Mute」、右の内側に「Mouse 3」のキー。右端にエンコーダーが2つ（Vol −／Vol ＋、Mouse Wheel Up／Mouse Wheel Down）。レイヤーの番号は 0〜9 [A・画像を見た]。
- 「行列の50の位置＝48キー＋丸い部品の押し込み2つ（(2,6) と (5,6)）」「エンコーダー2つ＝丸い部品の回転」と読める [B]。裏づけ:
  - S5 の仕様の欄に「Layout: 48 keys (extended 6x3+3)」「Dimensions: 142x91x15(case)/21(keycaps)/24(encoder)mm」[A]。
  - S10 の `boards/jzf/cornix/cornix_sensors.dtsi` に、回転式のエンコーダー（`alps,ec11`）が左右に1つずつ。`cornix-layouts.dtsi` の行列の図では、3段目だけ片手7つ（内側に1つ多い）[A]。
  - S8 の左手側・右手側の両方のファイルに、`rmk/src/input_device/rotary_encoder.rs` の文字列 [A]。
  - S3 の README 68行に「Cornix LP V1.12の50キーと左右エンコーダー4方向を実機どおりに表示」[A]。
  - 丸い部品は「回せて、押せる」（えだ発言・2026-10-03 14:47。だれが実機で確かめたかは [未確認]）。
- S1 と S2 の本文に、エンコーダー・ノブ・ダイヤル・ホイール（encoder・knob・dial・wheel）の語は出てこない [A・取った本文を語で検索]。S6 には、v1.13 の説明に「Mouse Wheel Up」が1回出る [A]。

### 3.2 cornix-studio の定義ファイル（何が入っているか）

- `studio/src/fixtures/cornix-v1.12.json`（2,387 バイト）。Vial 互換の機種定義 [A]。
- 最上位の項目: name（"Cornix"）・vendorId・productId・matrix（rows 8・cols 7）・customKeycodes・lighting（"none"）・layouts [A]。
- layouts.keymap は KLE（Keyboard Layout Editor）の形式。キーごとに、行列の位置（"行,列"）と、x・y・回転（r・rx・ry）を持つ。layouts.labels は [["Firmware Version","V1.12"]] [A]。
- 行列の位置は 50個（左25・右25）で、公式の .vil の「-1 でない位置」50個と完全に一致 [A・照合した]。ほかに、エンコーダーの項目が4つ（2個×2方向）[A]。
- 回転が付いているのは、親指の内側寄りの2キーずつ（(3,4) (3,5) (7,4) (7,5)）。親指のいちばん外側の (3,3) と (7,3) には付いていない。(2,6) と (5,6) は、左右の間にある [A]。
- customKeycodes は8個: BT0・BT1・BT2・NEXT_BT・PREV_BT・CLR_BT・SWITCH・CLR_PEER（それぞれ name・title・shortName を持つ）[A]。
- ライセンス: リポジトリ全体は GPL-2.0-or-later（COPYING、README 163〜170行）。THIRD_PARTY_NOTICES.md の 46〜50行は、この定義を「オフラインのデモと互換性テストのための、Vial 互換の機種定義」と説明し、ファームウェア・鍵・実行コードは含まないとしている。定義そのものの権利者と、どこから取ったかは書かれていない [A]。`studio/src/kle.ts` の先頭のコメントに「vial-kb/vial-gui の KLE パーサーを一部元にしている」[A]。
- .vil の読み取り（`studio/src/main.ts` 2536行〜の keycode 関数と、2789行〜の分岐）は、layout の値を「0〜0xffff の整数か -1」として読む。公式の .vil は文字列なので、読み取りは現物の .vil に合わせて自分で書く [A・コードを読んだ。動かしてはいない]。

### 3.3 公式ファームウェア V1.12 の中身（追加で分かったこと）

- V1.12.zip の中は cornix-left.uf2（877,568 バイト）と cornix-right.uf2（569,344 バイト）。どちらも zip の中の日付は 2026-03-02 [A]。
- ソースのパスの文字列が、`/Users/haobogu/Projects/keyboard/fix/cornix_panic/rmk/src/...` の形で入っている（左手側に27個、右手側に10個）[A・uf2 をつなぎ直して文字列を拾った]。パスの中の利用者名は、RMK のリポジトリの持ち主（HaoboGu）と同じつづり。その人の手元の作業ツリーからビルドした、と読める [B]。
- `rmk/src/keymap.rs` は左右の両方にある。`rmk/src/keyboard.rs` と `rmk/src/host/via/vial.rs` は左手側だけにある [A]。
- 依存クレートの版の文字列: embassy-nrf-0.8.0・embassy-sync-0.7.2・embassy-time-0.5.0・embassy-usb-0.5.1・heapless-0.9.1・trouble-host-0.5.1・sequential-storage-6.0.1・bt-hci-0.6.0 [A]。この組み合わせは、S7 のタグ rmk-v0.8.0〜rmk-v0.8.3 の `rmk/Cargo.toml` と合い、rmk-v0.7.8 と main とは合わない [A]。
- 出てくる RMK のファイルは、rmk-v0.8.0〜rmk-v0.8.3 には全部ある。rmk-v0.7.8 には11個が無く、main には3個が無い [A]。
- zip の中の日付（2026-03-02）は、rmk-v0.8.2（2025-12-18）と rmk-v0.8.3（2026-08-29）の間。つまり、V1.12 は「0.8 系の、タグではない作業ツリー」から作られている [B]。正確なコミットと、独自の変更の有無は [未確認]。
- 左手側のファイルに、Vial の機種定義が xz で圧縮されて入っている（展開すると 2,149 バイトの JSON）。項目は S3 の定義と同じ。キーの置き方（54項目の位置と回転）は、S3 の定義と全部同じ [A・展開して比べた]。違うのは、name（"HID Keyboard"）と、customKeycodes の title・shortName の文言だけ。つまり、S3 の定義の座標は、公式ファームウェアが Vial に渡している物と同じ値 [A]。
- 文字列「Jezail FunderCornix」がある [A]。

### 3.4 レイヤーの動き（D-12 の根拠）

RMK のソース（S7 のタグ rmk-v0.8.3）を読んだ結果。D-12 の (1)(2)(3) は、このソースの動きと一致する [A]。

- `rmk/src/keymap.rs` 219〜281行 `get_action_with_layer_cache`:
  - 離したとき（220〜225行）: 押したときに覚えたレイヤーを取り出し（293〜315行）、そのレイヤーのキーを返す。
  - 押したとき（232〜250行）: 番号の大きいレイヤーから順に見る。「有効なレイヤー」か「既定のレイヤー」のときだけ、そのキーを見る（233行）。キーが Transparent なら次へ進む（236〜238行）。それ以外（No を含む）なら、そのレイヤーを覚えて（241行）、そのキーを返す（243行）。既定のレイヤーまで来たら、そこで終わる（246〜249行）。
  - どのレイヤーでも決まらなかったとき（全部 Transparent）は、No を返す（280行）。
- `rmk/src/keyboard.rs` 796行: No と Transparent は、何もしない。
- 同 1131行と 1484〜1491行: MO（`Action::LayerOn`）は、押したときにレイヤーを有効にし、離したときに無効にする。
- `rmk/src/keymap.rs` 363〜386行: レイヤーの状態は、レイヤーごとの真偽1つ。数えてはいない。だから、同じレイヤーの MO を2つ押して1つ離すと、そのレイヤーは無効になる [A・ソース]。レイヤーの数以上の番号は、警告を出して何もしない（364〜370行）。
- `rmk/src/keyboard.rs` 2078〜2091行: 修飾キーも、押すとビットを立て、離すとビットを消す（数えてはいない）[A]。
- `rmk/src/host/via/keycode_convert.rs` 91〜210行 `from_via_keycode`: Vial から届くキーコードの番号を、RMK の動きに変える。0x0000＝No、0x0001＝Transparent、0x5220〜0x523F＝MO（レイヤーは下位4ビット。124〜128行）、0x7E00〜0x7E0F＝User（200〜204行）。対応していない番号は No になる（TT は 149〜153行、そのほかは 205〜208行）[A]。
- 版をまたいだ比較: `get_action_with_layer_cache` の本文は、rmk-v0.7.8・v0.8.0・v0.8.1・v0.8.2・v0.8.3 で1文字も違わない [A・関数の本文のハッシュを比べた]。main（f963675）は書き方が変わっているが、見る順番と、Transparent・No の扱いは同じ。足されたのは「全部 Transparent だったときも、既定のレイヤーを覚える」1行で、外から見える結果は同じ [A・差分を読んだ]。
- tri_layer（2つのレイヤーが両方有効なとき、3つ目のレイヤーを有効にする設定）:
  - 設定の既定値は「なし」（`rmk/src/config/mod.rs` 43〜46行。`Option` で、`Default` は `None`）[A]。
  - 設定されていると、レイヤーを有効・無効にするたびに、3つ目のレイヤーの状態が「1つ目 かつ 2つ目」で上書きされる（`rmk/src/keymap.rs` 348〜353行）[A]。だから、3つ目に当たるレイヤーは、MO だけでは有効にできなくなる。
  - 公式の案内は、MO(1)・MO(2)・MO(3) がそれぞれ単独で働くと書いている（S2「MO(1)を押し続けている間Layer1に切り替えます」「MO(2) を押し続けている間、Layer 2 に切り替わります」、S1「MO(3) に割り当てられたキーを押しながらCaps / Shift / Ctrl を押すことで、Bluetooth デバイスを切り替える」）[A]。この案内が正しく、ファームウェアが上のソースのとおりなら、レイヤー1・2・3 は tri_layer の3つ目ではない [B]。
  - Cornix のファームウェアが tri_layer を使っているかは [未確認]。初期キーマップでは、使っていてもいなくても、1周目の範囲のキーの結果は変わらない（レイヤー2〜9 は、範囲の外のキーのほかは全部 KC_NO のため）[B]。
- `TriLayerLower`・`TriLayerUpper` という別のキーコードもある（`rmk/src/keyboard.rs` 1410〜1417行）。初期キーマップには出てこない [A]。
- 公式ガイド（S2）が「Layer 1 に Shift キー を割り当てたうえで」と案内していることは、「上のレイヤーの KC_NO が、下のレイヤーのキーを隠す」動きと合う [B]。
- 実機での動きは [未確認]（実機に触った人の話は取れていない）。
- 初期キーマップで、この決まりが結果を分ける場面（位置は [A]。結果は、上のソースどおりに動く場合）:
  - 場面1: MO(1) を押している間、(3,3) はレイヤー1では KC_NO。離すときにレイヤー1のキーコードを見ると、レイヤー1から戻れなくなる。押したときに覚えたレイヤー0の MO(1) で処理するので、戻れる。
  - 場面2: 左 Shift → MO(1) → (0,1) の順に押すと、Shift の効いた KC_1 が出る。MO(1) → (2,0) → (0,1) の順に押すと、(2,0) はレイヤー1では KC_NO なので、Shift の効いていない KC_1 が出る。

### 3.5 .vil に出てくるキーコードの文字列（vial-gui の決まり）

- 文字列にするのは `src/main/python/keycodes/keycodes.py` 108〜124行の `serialize` [A]。
  - ふつうのキーコードは、決まった名前（例: KC_BSPACE、MO(1)）。別名（例: KC_BSPC）は書かれない。
  - 上位バイトが「マスク」に当たるキーコードは、外側の名前の中に内側の名前を入れた形（例: LSFT(KC_2)、LT1(KC_SPACE)）。
  - どれにも当たらない番号は、16進の文字列（例: "0x7c79"）。
- 読むとき（127〜142行の `deserialize`）は、整数もそのまま受ける。古い .vil やほかの道具が、整数のキーコードを書いている可能性がある [B]。
- 名前は `keycodes.py` の `K("名前", …)` の並び、番号は `keycodes_v6.py` にある [A]。MO(n)・DF(n)・TG(n)・TT(n)・OSL(n) は n が 0〜31、マクロ M0〜、タップダンス TD(0)〜（`keycodes_v6.py` 591〜600行）[A]。
- USERnn は、機種定義の customKeycodes の n 番目（`keycodes.py` 833〜841行）。だから USER00・USER01・USER02 は BT0・BT1・BT2 [A]。S1 の「MO(3) を押しながら Caps / Shift / Ctrl」と、レイヤー3の (1,0) (2,0) (3,0) の位置も合う [A]。
- 日本語配列に関わる名前: KC_RO（0x87）・KC_KANA（0x88）・KC_JYEN（0x89）・KC_HENK（変換。0x8A）・KC_MHEN（無変換。0x8B）・KC_LANG1（0x90）・KC_LANG2（0x91）（`keycodes.py` 322〜331行、`keycodes_v6.py`）[A]。RMK の側にも同じ番号がある（S7 `rmk-types/src/keycode.rs` 274〜284行）[A]。S3 の `studio/src/keycodes.ts` 231〜232行では、0x8a が「変換」、0x8b が「無変換」[A]。実機と Windows で届くかは [未確認]。
- Vial の「Keyboard Layout」で Japanese を選ぶと変わるのは、画面の刻印だけ（`keymaps.py` 54行、`keymap/japanese.py`）。キーコードの番号は変わらない [A]。
- 実機なしで編集する道: Vial のデスクトップ版の File メニューに「Load dummy JSON...」がある（`main_window.py` 168〜183行。Web 版には無い）。機種定義の JSON を読んで、実機の代わりの「dummy」を作る（`protocol/dummy_keyboard.py`。レイヤーは4、uid は 0）[A・ソース]。これで保存した .vil が読めるか、どんな形になるかは [未確認・動かしていない]。

### 3.6 公式の案内（S1・S2・S6 の本文）

- S1: 「Vialアプリ版「File → Load saved layout」 を選択することで、キーマップ設定を読み込むことができます」。保存は「File → Save current layout」[A]。
- S2: 「MO(1)を押し続けている間Layer1に切り替えます」。記号の方法①は「Layer 1 に Shift キー を割り当てたうえで、Layer 1 の状態でShift を押しながら、一番上の数字キーを押す」。方法②は「Layer 1 の空いているキーに記号を直接割り当てる」（例は「@」）[A]。
- S2: 「もし 「@」の代わりに「"」が入力される場合 は、PC 側のキーボード配列が JIS に設定されている 可能性があります。その場合は、Vial 側でも JIS レイアウトを正しく設定する必要があります」。選んだあと「「@」の位置が別のキーに切り替わるため、改めて割り当て直す必要があります」[A]。出る文字は、PC 側の配列（JIS か US か）で変わる。
- S2: 「ISO/JISタブでのLANG１とLANG２を指定したキーに割り当てば、日本語と英語の入力を切り替えます」[A]。
- S2 は、Vial の資料として CornixHub（https://cornixhub.com/vial-guide 。「Share your keymaps」）を紹介している [A・リンクがある。中身は読んでいない]。
- S6: ファームウェアの zip は V1.9〜V1.12。v1.13（更新日 2026-08-06）には zip が無く、「9月1日更新：複数の不具合を確認ており、修正版が出た後また更新します」[A]。
- S6 の更新履歴: v1.12「特定の操作でLayer切り替えによる接続が切れ問題を修正しました」。v1.11「Vial 上で Permissive Hold およびHold on Other Key Press の設定を 直接変更可能」。v1.10「Vial でキーマップをインポートする際、Combo および Tap Dance が読み込めない問題を修正」「エクスポート時のファームウェアが v1.9 以上である必要があります」。v1.9「v1.8 では Keyboard ID の変更を行ったため、v1.8 でエクスポートしたキーマップファイルは読み込めません。恐れ入りますが、v1.8 以前にエクスポートしたファイルをご使用ください」[A・原文のまま。どの版の .vil が読めないのかは、この文面からは決まらない]。
- 設計への帰結: ファームウェアの版で uid が変わりうるので、シミュレータは uid の値で .vil を拒まない [B]。
- S1・S2・S6 の本文に、ライセンス・再配布・著作についての記載は無い [A・語で検索]。公式の .vil と画像を配り直してよいかは [未確認] のまま。

### 3.7 Cornix 用の ZMK の定義（S10。第三者の物）

- リポジトリのライセンスは Apache-2.0（LICENSE）[A]。
- `boards/jzf/cornix/cornix-layouts.dtsi` に、ZMK の「物理レイアウト」（`zmk,physical-layout`。名前は LAYOUT_50）があり、キーの位置の項目が 50個 [A]。このファイルにライセンスの見出しは無い（ほかの一部のファイルには MIT の見出しがある）[A]。
- `boards/jzf/cornix/metadata/` に、QMK 形式の配列の JSON もある [A・ファイル名を見た。中は読んでいない]。
- 数値が実物の寸法どおりかは [未確認]（メーカーの物ではない）。

## 4. R-3 ブラウザのキーイベント

### 4.1 手元のキーの見分け方（KeyboardEvent.code）

- code は、物理的な位置で決まる名前を返す。日本語配列（106）で足されるキーは、IntlYen（¥）・Backslash（]）・IntlRo（ろ）・NonConvert（無変換）・Convert（変換）・KanaMode（カタカナ/ひらがな/ローマ字）。半角/全角は Backquote。Mac 用の日本語キーボードの かな は Lang1、英数 は Lang2 [A・S4 の 2.1.7節と code の一覧]。
- Windows のスキャンコードと code の対応（S11 `files/en-us/web/api/ui_events/keyboard_event_code_values/index.md` の Windows の表。Firefox と Chrome で同じ）: 0x001A＝BracketLeft、0x001B＝BracketRight、0x0027＝Semicolon、0x0028＝Quote、0x0029＝Backquote、0x002B＝Backslash、0x003A＝CapsLock、0x0070＝KanaMode、0x0073＝IntlRo、0x0079＝Convert、0x007B＝NonConvert、0x007D＝IntlYen、0xE05B＝MetaLeft、0xE05C＝MetaRight、0xE05D＝ContextMenu、0xE038＝AltRight、0xE01D＝ControlRight [A]。
- だから、日本語配列のキーボードでは、@＝BracketLeft、[＝BracketRight、;＝Semicolon、:＝Quote、]＝Backslash、英数＝CapsLock になる [B・スキャンコードと刻印の対応は一般的な知識。えだの PC での値は実測で確かめる]。
- 位置で当てると、文字の3段は左右6列ずつそろう。左手は Tab Q W E R T／英数 A S D F G／Shift Z X C V B。右手は Y U I O P @／H J K L ; :／N M , . / ろ [B]。
- 下段は足りない。Cornix は片手6キー（左右で12）。日本語配列のキーボードは、左から Ctrl・Win・Alt・無変換・Space・変換・かな・Alt・(Win)・アプリケーション・Ctrl の10〜11キーで、Space は1つ [B・チャット側の報告]。えだのキーボードは REALFORCE の日本語配列・テンキーレス（えだ発言）。下段の正確な並びと、Windows 用か Mac 用かは [未確認]。

### 4.2 届き方が怪しいキー（他人の観測。えだの PC では未確認）

- Windows 11（日本語）＋ Chrome 139 ＋ JIS 106 での観測（S15）[B・他人の報告]:
  - 半角/全角: 押すたびに「前の状態の keyup」と「今の状態の keydown」が続けて届く（code はどちらも Backquote。key は Zenkaku と Hankaku が入れ替わる）。離したときには keyup が来ない。素朴に数えると、押されたままに見える。
  - 右 Shift: Microsoft IME のとき、key が "Shift"、code が空文字で届く。Google 日本語入力と、Microsoft IME（Legacy）では ShiftRight で届く（2025-12-10 のコメントの要約。原文は英語）。
  - 半角/全角: Microsoft IME が keydown を取ってしまい、keyup だけが届くことがある（同じコメントの要約）。
  - ろ: code は IntlRo で、keydown と keyup が届く。
- 入力欄にフォーカスがあると、半角/全角 は IME が取って、ページにイベントが届かない。入力欄の外なら届く、という 2021年の観測もある（https://let.blog.jp/article/27890813 ）[B・他人の報告]。
- 英数（CapsLock）と かな（KanaMode）で keyup が来るかは [未確認]。
- keyhac が動いているとき（えだの PC では常駐している [A・2026-10-03 15:18 に keyhac.exe が動いていた]。無変換と変換を使っている（えだ発言））、この2つがブラウザに届かないか、別のキーとして届く可能性がある [未確認]。
- 押している間、ブラウザは keydown を repeat 付きで繰り返し送る。レイヤーの処理では、これを「押し直し」として扱わない [B]。
- ウィンドウがフォーカスを失うと（Alt+Tab・Win キーなど）、keyup が届かないまま終わる [B]。

### 4.3 ページの側で止められないキーと、止める手段

- Chrome が、ふつうの窓でページに渡さないショートカット（S13 `chrome/browser/ui/browser_command_controller.cc` 473行〜の `IsReservedCommandOrKey` と、`chrome/browser/ui/accelerator_table.cc`）[A・ソース]:
  - Ctrl+W・Ctrl+F4（タブを閉じる）、Ctrl+Shift+W・Alt+F4（窓を閉じる）、Ctrl+T（新しいタブ）、Ctrl+N（新しい窓）、Ctrl+Shift+N（シークレット）、Ctrl+Shift+T（閉じたタブを戻す）、Ctrl+Tab・Ctrl+Shift+Tab・Ctrl+PageDown・Ctrl+PageUp（タブの切り替え）。
  - 全画面のときは、F11（全画面の切り替え）と終了のほかは、全部ページに渡す。
  - アプリの窓（インストールしたページなど）では、予約は無い。
- 上の一覧に無いショートカット（Ctrl+L・Ctrl+F・Ctrl+P・F5・F10・Alt 単独でメニューに移る動き、など）は、ページが先に受け取る。preventDefault で止められる見込み [B。1周目の実測で確かめる]。Tab（フォーカスの移動）・Space と矢印（スクロール）も同じ [B]。
- Firefox が、ページに渡さないショートカット（S14 `browser/base/content/browser-sets.inc.xhtml` の `reserved="true"`）: 新しい窓・新しいタブ・タブを閉じる・窓を閉じる・プライベートウィンドウ・終了 [A・ソース]。キーは Ctrl+N・Ctrl+T・Ctrl+W・Ctrl+Shift+W・Ctrl+Shift+P・Ctrl+Shift+Q [B・文字は別のファイルにあり、記憶で書いた]。
- Windows が先に取るキー（Win キー・Win+何か・Alt+Tab・Alt+Esc・Ctrl+Esc など）は、ページの preventDefault では止まらない [B]。
- Keyboard Lock（`navigator.keyboard.lock()`）[A・S11 と S12]:
  - ページが JavaScript で全画面（`requestFullscreen()`）にしている間だけ働く。F11 の全画面では働かない。
  - 対応は Chrome 68 以降と、Edge・Opera。Firefox と Safari は対応していない。
  - HTTPS（か localhost）のページでだけ使える。利用者の操作のあとでだけ呼べる。Chrome 130（2024年9月）からは、許可の確認が出る。
  - 抜けるのは、Esc の長押し（2秒）。
  - Ctrl+Alt+Del のような OS の決まった組み合わせは、止められない。
  - Windows の Chrome は、全画面で窓にフォーカスがある間だけ、低レベルのフックで Ctrl・Alt・Win を先に受ける（S13 `ui/events/win/modifier_keyboard_hook_win.cc` 40〜73行のコメント）[A・ソース]。Win キーや Alt+Tab が実際にページに届くかは [B。実測で確かめる]。
- `navigator.keyboard.getLayoutMap()` は、いまの配列で code から文字を引ける。Chrome 69 以降だけ [A・S11]。
- keyhac のように、OS の側でキーを先に取る常駐ソフトが止めたキーは、ページからは何もできない [B]。
- 手段のまとめ: (a) 対応表にあるキーの keydown・keyup で preventDefault する (b) 「全画面で試す」ボタンで、全画面＋Keyboard Lock にする (c) ふつうの窓のときは、Ctrl+W や Win キーが効いてしまうことを画面に書く (d) 止められないキーは、対応表で別のキーに当てる (e) フォーカスを失ったら、押されているキーを全部離した扱いにする。どれを採るかは Q-7。

### 4.4 キーコードから文字への対応（JIS と US）

HID の番号ごとの文字。左が素、右が Shift。JIS の側は、1周目の実測（えだの PC で event.key を見る）で確かめる [B・配列の知識]。

| キーコード | US | JIS |
|---|---|---|
| KC_1 | 1 ! | 1 ! |
| KC_2 | 2 @ | 2 " |
| KC_3・KC_4・KC_5 | 3 # ・ 4 $ ・ 5 % | 同じ |
| KC_6 | 6 ^ | 6 & |
| KC_7 | 7 & | 7 ' |
| KC_8 | 8 * | 8 ( |
| KC_9 | 9 ( | 9 ) |
| KC_0 | 0 ) | 0 （Shift は文字なし） |
| KC_MINUS | - _ | - = |
| KC_EQUAL | = + | ^ ~ |
| KC_LBRACKET | [ { | @ ` |
| KC_RBRACKET | ] } | [ { |
| KC_BSLASH | \ ｜ | ] } |
| KC_SCOLON | ; : | ; + |
| KC_QUOTE | ' " | : * |
| KC_GRAVE | ` ~ | 半角/全角（文字なし） |
| KC_COMMA・KC_DOT・KC_SLASH | , < ・ . > ・ / ? | 同じ |
| KC_RO | 文字なし | \ _ |
| KC_JYEN | 文字なし | ¥ ｜ |

（表の中の「｜」は、縦棒の記号 U+007C のこと。Markdown の表の区切りと重なるので、全角で書いた。）

- 裏づけの一部: vial-gui の日本語配列の刻印（S9 `keymap/japanese.py`）で、KC_LBRACKET が「@ と `」、KC_RBRACKET が「[ と {」、KC_2 の Shift が「"」、KC_6 が「&」、KC_7 が「'」[A・この5つを見た。表は写していない]。
- 文字が JIS と US で分かれるのは、KC_2・KC_6・KC_7・KC_8・KC_9・KC_0 の Shift 側、KC_MINUS の Shift 側、KC_EQUAL、KC_LBRACKET、KC_RBRACKET、KC_BSLASH、KC_SCOLON の Shift 側、KC_QUOTE、KC_GRAVE、KC_RO、KC_JYEN [B]。

### 4.5 手元でページを開く方法

- ES モジュールと WASM は、`file://` では読み込めない。手元で開くには、静的なファイルを配る HTTP サーバーが要る [B]。この PC には Python 3.12.10 があるので、`python -m http.server` で足りる [A・python があること。B・これで動くこと]。
- `http://localhost` は HTTPS と同じ扱いなので、Keyboard Lock も手元で試せる [B]。

## 5. R-4 Rust＋WASM のページを GitHub Pages に出す方法

### 5.1 道具（この PC で確かめた）

- この PC: rustc 1.98.0・cargo 1.98.0・wasm-pack 0.15.0・wasm32-unknown-unknown ターゲット・Python 3.12.10・gh 2.102.0 が入っている [A]。
- リポジトリの外の一時フォルダに、試し用の crate を2つ（ロジックと画面）作って確かめた [A]:
  - 画面の側は、マインスイーパー2号機と同じ形（crate-type が ["cdylib","rlib"]。wasm-bindgen と web-sys は target が wasm32 のときだけの依存。画面のコードは `#[cfg(target_arch = "wasm32")]` のモジュール）。
  - `cargo test --workspace` が通る（画面の crate も、ふつうのターゲットでビルドできる）。
  - `cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery` は、ルートが `[workspace]` だけの Cargo.toml なら、`--workspace` を付けなくても全部の crate を見る。
  - 上の2つは、wasm32 のときだけのコードを見ない。見るには `cargo clippy --workspace --target wasm32-unknown-unknown -- -D warnings -D clippy::pedantic -D clippy::nursery`。これも通った。
  - `wasm-pack build <画面の crate> --target web --out-dir ../static/pkg --no-typescript` が通る（約15秒）。`--out-dir` は、crate のフォルダから見た相対パス。出てくるのは `<crate名>.js`・`<crate名>_bg.wasm`・`package.json`・`.gitignore`（中身は `*`）。
  - wasm-bindgen 0.2.129・web-sys 0.3.106 でビルドできた。
  - serde_json 1.0.151 で、公式の .vil と同じ形が読める。uid は `u64` なら読め、`i64` では誤りになる。文字列と -1 が混ざる値は `serde_json::Value` で受けられる。末尾に CRLF を足しても読める。
- マインスイーパー2号機（S18。手元の先頭は ae71b1f。origin/main は 2dfaff8）: `client/Cargo.toml` は上と同じ形（wasm-bindgen 0.2.128・web-sys 0.3.105）。`client/src/lib.rs` 7行が `#[cfg(target_arch = "wasm32")] mod app;`。`static/index.html` が `import init from "./pkg/client.js"` で読み込む。`/static/pkg/` は .gitignore。wasm-pack のコマンド行は、どのファイルにも書かれていない。`.github/` は無い [A]。
- wasm-pack のリポジトリは wasm-bindgen/wasm-pack に移っている（アーカイブではない）。最新は v0.15.0（2026-05-15）。リリースに Linux 用（x86_64-unknown-linux-musl）の実行ファイルがある [A・S17]。
- crates.io の最新の安定版（2026-10-03）: wasm-bindgen 0.2.129・web-sys 0.3.106・js-sys 0.3.106・serde 1.0.229・serde_json 1.0.151・wasm-bindgen-test 0.3.79 [A・S17]。

### 5.2 GitHub Pages の決まり

- リポジトリ eda3/cornix-simulator は公開（public）[A・S17]。Pages は、GitHub Free でも公開リポジトリなら使える [A・S16 `data/reusables/gated-features/pages.md`]。
- 公開元は2通り: 「ブランチから（フォルダは / か /docs）」か「GitHub Actions」[A・S16 `configuring-a-publishing-source-for-your-github-pages-site.md`]。
- GitHub Actions で出すときの流れ: checkout → ビルド → `actions/upload-pages-artifact` → `actions/deploy-pages`。ジョブに `pages: write` と `id-token: write` の権限が要る。環境は `github-pages` [A・S16 `using-custom-workflows-with-github-pages.md`]。
- Actions の最新（2026-10-03）: actions/checkout v7.0.1・actions/configure-pages v6.0.0・actions/upload-pages-artifact v5.0.0・actions/deploy-pages v5.0.1 [A・S17]。
- 制限: 公開するサイトは 1 GB まで。帯域は月 100 GB（目安）。デプロイは10分で打ち切り [A・S16 `github-pages-limits.md`]。
- 公開先の URL は https://eda3.github.io/cornix-simulator/ になる見込み。ページの中のパスは相対（`./pkg/…`）で書く [B]。

### 5.3 公開までの手順の選択肢

- a. GitHub Actions で公開する。main に入ると、Actions が cargo test と wasm-pack のビルドをして、Pages に出す。wasm-pack は、公式のリリースの実行ファイルを取って入れられる（第三者の Action は要らない）。えだは最初に1回、Settings → Pages の Source を「GitHub Actions」にする。workflow が実際に通るかは [未確認]（公開の周で確かめる）。
- b. 手元でビルドして、gh-pages ブランチに置く。Pages の公開元は「ブランチ（gh-pages の /）」。wasm-pack が出す `pkg/.gitignore`（中身は `*`）を消すか、`git add -f` が要る。ビルドの結果が git の履歴に入る。
- c. 手元でビルドした物を main に入れて、ブランチから公開する。フォルダは / か /docs しか選べない。/docs は設計の文書の置き場と重なる。/ にすると、リポジトリの全部が公開ページになる。
- d. wasm-pack 以外の道具（Trunk など）。第一候補ではないので、調べていない。
- どれにするかは Q-12。

## 6. R-5 /next-todo と test-reviewer が前提にしている形

### 6.1 /next-todo（`~/.claude/skills/next-todo/SKILL.md`）[A・全文を読んだ]

- `docs/todo.md` のチェックが付いていない項目を、上から順に1つずつ進める。【えだ】で始まる項目に来たら、止まって報告する。
- 1つの項目は「計画 → 作成 → 検証 → 判定」を1周として、えだの入力を待たずに回す。上限は1項目につき5周。
- 検証のコマンドは3つ: `cargo test --workspace`（doc-tests を含む）・`cargo clippy -- -D warnings -D clippy::pedantic -D clippy::nursery`・`cargo fmt --check`。
- 3つが通り、項目の完了条件を満たしたら、チェックを入れて、チェックの変更も含めて1回コミットする（push はえだ）。
- 項目に丸数字（⑦など）があるときは、`docs/test-items.md` のその番号の中身を確かめるテストを書き、テスト名に番号を入れる（例: `item7_...`）。
- 「点検」の項目: test-reviewer に「点検してください」の1行だけを渡す。「弱い」「ない」とされた項目は、書かれた壊し方を一時的に入れて `cargo test --workspace` を回し、結果を記録して元に戻す。点検は最大2回。
- 守ること: `docs/todo.md` の順番と文言は変えず、チェックを入れるだけ。前の項目で作った物の名前や動きは変えず、足すだけ。CLAUDE.md の「止まって聞く場面」では止まる。ただし、項目の「足してよい依存」に書かれたクレートは、止まらずに足してよい。
- 止まったときの報告: 止まった場所と理由／項目ごとのコミットのハッシュ／テスト名と丸数字の対応表／点検役の返答（原文）と壊し方ごとの結果／置いた前提。

### 6.2 test-reviewer（`~/.claude/agents/test-reviewer.md`）[A・全文を読んだ]

- 使える道具は Read・Grep・Glob。読むだけで、直さない。
- `docs/test-items.md` のテスト項目を読み、項目ごとに「対応するテスト名／判定（確かめている・弱い・ない）／根拠のコードの引用（ファイル名と行番号つき）／理由1行」を返す。「弱い」のときは、ルールが壊れていてもテストが通ってしまう壊し方を1つ書く。
- 目安として挙げているのは「真ん中だけを試していて角や辺を試していない」「1つのシードでしか試していない」「期待値を実装と同じ関数で作っている」。

### 6.3 edaberu の文書の形（S18。手元は stage3-core ブランチの 1321300）[A・読んだ]

- `docs/todo.md`: 見出しは「# ToDo」。冒頭に進め方（上から `/next-todo`、1項目＝1ループ＋1コミット、【えだ】の項目はえだがチェック）。周は「## N周目: 題」と「ゴール:」。項目は `- [ ]` で、下に「メモ:」「足してよい依存:」「完了条件:」。区切りに「点検: test-reviewer に点検させ、指摘を壊し方で確かめて直す（最大2回）」。周の終わりに【えだ】の項目。項目の文に、`docs/test-items.md` の丸数字が入る。
- `docs/test-items.md`: 冒頭に、このファイルの役割と、番号の指し先と、「自動テストの書き方（点検役はここを見て「弱い」を判定する）」（期待値は表から手で写す／境界を入れる／外の物は偽物か注入で置き換える／点検の指摘は実装を1か所壊して確かめる）。そのあとに、周ごとの見出しと、①〜の項目。
- `docs/measurements.md`: 冒頭に「測った数字と、見えたことを書く場所。推測は書かない」。①〜の見出しごとに、測った日と、項目の枠（「項目名：」の形）が先に作ってある。
- `CLAUDE.md`: Project／文書の役割／設計の約束／Code Style／Workflow／テスト／止まって聞く場面。このリポジトリの CLAUDE.md と同じ並び。
- ブランチ: 周ごとのブランチ（stage2-bot・stage3-core）と PR。main の先頭は PR の合流（d299ecc）[A]。CLAUDE.md に「ブランチへの push と PR の作成（`gh pr create`）は Claude Code が行ってよい。main への合流（`gh pr merge`）とブランチの削除はえだが行う」。
- マインスイーパー2号機の `docs/todo.md` と `docs/test-items.md` も同じ形（丸数字・メモ・足してよい依存・完了条件・点検・【えだ】）。【えだ】の項目に「wasm-pack と wasm32-unknown-unknown ターゲットを入れる」があり、チェックが入っている [A]。

### 6.4 段2で気を付ける点（食い違いを含む）

- このリポジトリの CLAUDE.md は、段1の時点で「`/next-todo` は上から順に1項目だけ進める」と書いていて、SKILL.md の本文（【えだ】の項目に来るまで、1つずつ続ける）と食い違っていた [A]。2026-10-03 のえだの指示で、CLAUDE.md を SKILL.md に合わせて直した。止めたい所には、【えだ】で始まる項目を置く。
- CLAUDE.md の clippy は `--workspace` 付き、SKILL.md は無し。ルートが `[workspace]` だけの Cargo.toml なら、結果は同じ [A・5.1節]。
- SKILL.md の3つのコマンドは、wasm32 のときだけのコードを見ない [A・5.1節]。画面の項目の「完了条件」に、wasm32 向けの clippy と wasm-pack のビルドを書く。
- 画面の crate は、ふつうのターゲットでも `cargo test --workspace` が通る形にする（マインスイーパー2号機と同じ形）。
- crate の名前は、標準ライブラリと重ならないものにする（CLAUDE.md）。
- test-reviewer が読むのは `docs/test-items.md` なので、「自動テストの書き方」は、その冒頭に置く。

## 7. 準備の指示文の事実（F-1〜F-7）との照合

- 一致（Claude Code が現物で確かめた）: F-1 の全項目。F-2 の外形図・Vial の画面・本文の語の検索。F-3 の全項目。F-4 の引用（下の2点を除く）。F-5 のソースの動き・依存の版・ファームウェアの中の文字列。F-6 のうち code の名前（S4）。F-7 のうち edaberu とマインスイーパー2号機の中身。
- 違い・足りなかった点:
  - F-3 layouts.labels: 指示文は `["Firmware Version","V1.12"]`。現物は `[["Firmware Version","V1.12"]]`（配列の中に配列）。
  - F-4「@」の代わりに出る文字: 指示文は「別の文字」。現物は「"」と書いてある。
  - F-4 v1.8 の Keyboard ID: 指示文は「それより前に保存した .vil は読み込めない」。現物は 3.6節の原文のとおりで、どの版の .vil が読めないのかは決まらない。
  - F-5 ファームウェアの中の文字列: 指示文は「左右のファイルに keymap.rs・keyboard.rs・host/via/vial.rs など」。現物は、右手側には keyboard.rs と host/via/vial.rs が無い（keymap.rs はある）。
  - F-5 main との比較: 指示文は「同じ動き」。現物は、関数の書き方が変わっていて、1行足されている。外から見える結果は同じ（3.4節）。
  - F-5 RMK の版: 指示文は「どの版かは未確認」。現物から、0.8 系の作業ツリーで、タグ rmk-v0.8.3（2026-08-29）より前の日付（2026-03-02）だと分かった（3.3節）。
- 確度が上がった点: 右手は列0が外側（[B] → [A]）。USER00〜USER02 が BT0〜BT2（[B] → [A]）。settings の番号の意味（[未確認] → [A]）。マインスイーパー2号機の手元の場所（[未確認] → [A]）。この PC に wasm-pack と wasm32 のターゲットがある（[B] → [A]）。
- 照合していない（[B・チャット側の報告] のまま）: F-6 の「日本語配列のキーボードの下段の並び」。F-7 の「マインスイーパー2号機は main に直接積んでいた」（手元の main が origin より1つ先なのは見た）。

## 8. 未確認の一覧と、確かめる場所

段1（2026-10-03）の時点の一覧。そのあとの一覧は、`docs/design.md` の6節にある。

| 番号 | 未確認の点 | 確かめる場所 |
|---|---|---|
| U-1 | 実機でのレイヤーの動き（RMK のソースどおりか） | 確かめる手段が無い。持ち主に聞けたら、ここを書き直す |
| U-2 | ファームウェアの正確なコミットと、独自の変更の有無 | 同上 |
| U-3 | Cornix のファームウェアが tri_layer を使っているか | 同上（初期キーマップの範囲では結果が変わらない） |
| U-4 | えだのキーボードの下段の並びと、Windows 用か Mac 用か | 1周目の先頭の実測（`docs/measurements.md`） |
| U-5 | keyhac が動いているときの、無変換・変換などの届き方 | 同上（keyhac を動かしたままと、止めたときの両方） |
| | → 2026-10-03 えだの決定で、シミュレータは keyhac を止めて使うことに | |
| U-6 | 右 Shift の code、半角/全角・英数・かな の keydown と keyup、使っている IME | 同上 |
| U-7 | preventDefault で止まるキー（Tab・Alt 単独・F キーなど）と、全画面＋Keyboard Lock で Win キーが届くか | 同上 |
| U-8 | JIS の文字の表（4.4節。Shift+0 など） | 同上（event.key を見る） |
| U-9 | Vial の dummy で保存した .vil の形 | えだが試すとき |
| U-10 | 公式の .vil と画像を配り直してよいか | 未確認のまま（`local/` に置き、git に入れない） |
| U-11 | 機種定義の座標の権利者 | Q-10 でえだが決める |
| U-12 | GitHub Actions の workflow が実際に通るか | 公開の周 |
| U-13 | 丸い部品が「回せて、押せる」ことを、実機で確かめた人 | 未確認のまま |
| U-14 | Cornix を買ったあと、PC の配列を JIS のままにするか、US にするか | Q-2 |
| U-15 | `local/` の .vil の末尾に CRLF を足した物（2.1節） | えだに聞く |

## 9. 決まっていない振る舞い（質問一覧）

えだの回答（2026-10-03）は、`docs/design.md` の 5.1節にある。Q-10 と Q-11 はライセンスの判断を含むので、推奨を付けていない。

**Q-1（a）: .vil を読み込む前（初めて開いたとき）に、画面に何を出しますか。**
- 事実: 公式の .vil は git の管理の外に置くので、公開したページには、最初はキーマップが無い。公式のページに、配り直しについての記載は無い [A・3.6節]。
- 選択肢: a. 「.vil を選んでください」の案内と、入手先（公式マニュアル）へのリンクだけを出す ／ b. 自作の見本キーマップ（公式の写しではない、最小のもの）で、開いてすぐ打てる状態にする。案内とリンクも出す ／ c. b に加えて、読み込んだ .vil をブラウザに覚えておき、次に開いたときはそれを出す
- 推奨: b（開いてすぐ試せるため。見本は KC_TRNS も入れた形にして、テスト用の自作 .vil と同じ物を使い、全文を `docs/design.md` に書く。c は2周目以降の候補）

**Q-2（b）: キーコードから文字への変換を、JIS と US のどちらで行いますか。切り替えを付けますか。**
- 事実: 出る文字は PC 側の配列で決まる [A・3.6節]。JIS と US で分かれるキーは 4.4節 [B]。初期キーマップのままでは、JIS で `` @ ` [ { \ _ ¥ | `` が、US で `` [ ] { } ` ~ `` が打てない [B・2.4節]。えだのキーボードは日本語配列（えだ発言）。Cornix を買ったあとの PC の配列は [未確認]。
- 選択肢: a. JIS だけ ／ b. US だけ ／ c. 切り替えを付ける（初期値は JIS）／ d. 切り替えを付ける（初期値は US）
- 推奨: c（えだの PC のままで出る文字が最初に見え、US にした場合とも見比べられるため。ロジックは「配列」を引数で受けるだけで、表が2つになる）

**Q-3（c）: 1周目で動かすキーコードの線引き（D-10・D-11）は、どれにしますか。**
- 事実: 初期キーマップの KC_CAPSLOCK（(1,0)）は、D-10 にも D-11 にも入っていない [A]。公式ガイドは、記号を「Shift 付きのキーコード（例: @）を直接割り当てる」方法でも案内している [A]。vial-gui は、それを LSFT(KC_2) の形や、KC_AT のような名前で .vil に書く [A・ソース]。持ち主の .vil には、F1〜F12・Home・End・変換・無変換・LANG1・LANG2 も出うる [B]。
- 選択肢: a. D-10 のまま（KC_CAPSLOCK も「未対応」）／ b. D-10 に「名前の分かる、文字を出さないキー」（KC_CAPSLOCK・F1〜F12・Home・End・PageUp・PageDown・Insert・変換・無変換・LANG1・LANG2 など）を足す。押すと光り、キーに名前が出る。出た文字の欄は変えない。CapsLock の状態は持たない ／ c. b に加えて、Shift 付きの記号のキーコード（KC_EXLM・KC_AT … と、LSFT(kc)・RSFT(kc) の形）も動かす
- 推奨: c（初期キーマップのキーが「未対応」に見えるのを避けられる。Shift 付きの記号は、公式ガイドが案内している使い方で、えだが自分のキーマップを試すときに要る。作りは「Shift を押した扱いで、中のキーを文字に変える」だけ）

**Q-4（c）: 範囲の外のキーコードは、画面でどう見せますか。**
- 事実: 初期キーマップの範囲の外は、USER00〜USER02（Vial の画面では BT0〜BT2）・KC_MUTE・KC_BTN3 と、エンコーダーの4つ [A]。持ち主の .vil では、LT・MT・TD・コンボ・マクロが使われうる [B]。vial-gui は、知らない番号を "0x…" の文字列で書く [A・3.5節]。
- 選択肢: a. キーコードの文字列をそのまま小さく出し（例: USER00、LT1(KC_SPACE)）、色を薄くして「未対応」と分かる形にする。読み込んだときに「未対応のキーが n 個。コンボ n 個・タップダンス n 個・マクロ n 個は動きません」と1行で知らせる ／ b. a と同じ見た目で、読み込んだときの知らせは出さない ／ c. 一律に「未対応」とだけ書く
- 推奨: a（何が割り当ててあるかが読めて、動かない理由も分かるため。動いているように見えたまま結果がずれるのを防ぐ）

**Q-5（d）: 修飾キー（Shift・Ctrl・Alt・GUI）を押している間の見せ方と、Ctrl・Alt・GUI と文字の組み合わせは、どう出しますか。**
- 事実: キーボードは修飾キーと文字キーを別々に送り、文字にするのは PC の側。Shift は文字を変える（Q-2）。Ctrl・Alt・GUI と文字の組み合わせは、PC では文字にならず、ショートカットとして働く [B]。
- 選択肢: a. 押している修飾キーを、欄の上に札（Shift・Ctrl・Alt・GUI）で出す。Ctrl・Alt・GUI が入った組み合わせは、欄には足さず、「直前のキー」の1行に Ctrl+C の形で出す。Esc・矢印・Delete・F1 など、文字にならないキーも、同じ1行に出す ／ b. a と同じだが、組み合わせを欄の中に [Ctrl+C] の形で並べる ／ c. 修飾キーは光るだけ。組み合わせは何も出さない
- 推奨: a（欄は「文字として出たもの」だけになり、ショートカットが送られたことも1行で確かめられるため）

**Q-6（e）: 手元のキーが足りない下段は、どういう方針で当てますか。対応表を画面で変えられるようにしますか。**
- 事実: Cornix の下段は、片手が「外側の3キー＋親指の3キー」で、左右で12キー [A]。日本語配列のキーボードの下段は10〜11キーで、Space は1つ [B・チャット側の報告]。keyhac が無変換と変換を使っているので、この2つが届くかは [未確認]。半角/全角と右 Shift は、届き方が怪しい [B・4.2節]。初期値の表そのものは、1周目の実測のあとで決める（D-8）。
  - → 2026-10-03 えだの決定で、シミュレータは keyhac を止めて使うことに
- 選択肢（方針）: a. 親指を先に当てる（Space・無変換・変換など、親指で押せるキーを Cornix の親指キーに当てる。外側の3キーは Ctrl・Win・Alt の側から当て、足りない位置は、手元で余っているキーに当てる。それでも足りない位置は「手元のキー無し」と表示する）／ b. 外側を先に当てる（Ctrl・Win・Alt を外側の3キーに当て、親指は残りのキーで当てる）／ c. 足りない位置は、画面のクリックで押す
- 選択肢（変え方）: 1. 1周目は、表を1か所のデータとして持つだけ（変えるときは、その表を直す）／ 2. 画面で1キーずつ変えられて、ブラウザに覚える
- 推奨: a と 1（MO は親指にあり、「レイヤーでやっていけるか」を確かめるには、親指の位置が実物に近いほうがよいため。画面での変更は、表が決まってからの候補にする）

**Q-7（f）: ブラウザや Windows が先に取るキーは、どう扱いますか。対象のブラウザはどれにしますか。**
- 事実: 4.3節。Chrome は、ふつうの窓では Ctrl+W・Ctrl+T・Ctrl+N などをページに渡さず、全画面では渡す [A]。Win キーなどは、全画面＋Keyboard Lock のときだけページに届く見込みで、対応は Chrome と Edge だけ [A・対応表。B・実際に届くか]。Firefox は Keyboard Lock が無く、Ctrl+W なども渡さない [A]。
- 選択肢: a. 対応表にあるキーは全部 preventDefault する。「全画面で試す」ボタンを付け、全画面＋Keyboard Lock で、Win キーや Ctrl+W も Cornix のキーとして受ける。ふつうの窓のときは「Ctrl+W でタブが閉じます。Win キーでスタートが開きます」と注意を出す。対象は Chrome と Edge ／ b. a から全画面のボタンを外す（preventDefault と注意書きだけ）／ c. Win・Ctrl・Alt は対応表に入れず、別のキーに当てる
- 推奨: a（修飾キーを実物と同じ位置で試せるため。Firefox は「確かめていない」と README に書く）

**Q-8（g）: 左右の内側にある丸い部品を、画面に出しますか。**
- 事実: 行列の (2,6) と (5,6) と、エンコーダー2つがある [A]。回転式のエンコーダーだという裏づけは 3.1節 [A・レビューと ZMK の定義。B・実機]。「回せて、押せる」（えだ発言。だれが実機で確かめたかは [未確認]）。初期キーマップでは、ミュート／中クリックと、音量／ホイールで、1周目の範囲の外（D-11）。
- 選択肢: a. 部品の形だけを描いて、「未対応」と表示する ／ b. 描かずに、48キーだけを出す
- 推奨: a（実物の見た目に近く、キーの数え間違いを防げるため）

**Q-9（h）: 出た文字の欄での Backspace・Enter・Delete・矢印の動きと、全部消す操作は、どうしますか。**
- 事実: 欄は、手元のキーをそのまま受ける入力欄ではなく、シミュレータが決めた文字を足していく表示になる（入力欄にすると、手元のキーの文字がそのまま入り、IME も働くため）[B]。キーは全部 Cornix のキーに当たるので、画面の操作はマウスになる。
- 選択肢: a. 末尾に足すだけの欄にする。Backspace は末尾の1文字を消す。Enter は改行。Tab はタブ文字。Delete と矢印は欄を変えない（Q-5 の「直前のキー」に名前が出る）。全部消すのは、画面のボタン ／ b. カーソルを持つ欄にする（矢印で動き、Delete は右の1文字を消す）
- 推奨: a（確かめたいのは配列とレイヤーで、欄の編集ではないため。b はロジックとテストが増える）

**Q-10（i）: キーの座標（段差・角度）を、どこから取りますか。**
- 事実: キーの数と並びは、外形図と .vil から分かる [A]。座標の数値を持っているのは3か所: cornix-studio の定義ファイル（GPL-2.0-or-later のリポジトリ。定義の権利者は書かれていない）[A]、公式ファームウェアの中の機種定義（cornix-studio の物と置き方が全部同じ。ライセンスの記載は無い）[A・3.3節]、第三者の ZMK の定義（Apache-2.0。実物の寸法どおりかは未確認）[A・3.7節]。
- 選択肢と、選んだときの結果:
  - a. 自分で描く（数と並びは事実どおり。段差と角度は自分で決めた目安の値にして、画面に「寸法は目安」と書く）→ ライセンスは、あとで自由に決められる。あとから b・c・d に変えることもできる
  - b. cornix-studio の定義ファイルの数値を使う → このリポジトリを GPL-2.0-or-later にするか、定義の権利者（メーカーの側と読める [B]）に確かめるかを、えだが決めることになる
  - c. 定義ファイルも、利用者がページで読み込む形にする（リポジトリには置かない）→ 初めて開いた人は、配列の図を見るのにもファイルが要る
  - d. ZMK の定義（Apache-2.0）の数値を使う → Apache-2.0 の条件（ライセンス文を添える・出どころと変えた点を書く）を満たせば、このリポジトリのライセンスは自由に選べる。数値が実物どおりかは分からない
- 推奨: 付けない（ライセンスの判断を含むため。決めるのはえだ）

**Q-11（i）: キーコードの名前の表（"KC_BSPACE" など）を、どう作りますか。**
- 事実: .vil のキーコードは文字列で、公式の .vil に出てくるのは 69種類 [A]。1周目で動かすキーには、公式の .vil に出てこない名前（KC_LBRACKET・KC_TRNS・KC_RSHIFT など）も要る。名前を決めているのは vial-gui の `keycodes.py`（GPL）[A・3.5節]。名前の元は QMK（GPL-2.0-or-later）、番号は USB HID の仕様の値 [B・記憶]。
- 選択肢と、選んだときの結果:
  - a. 名前を「.vil という書式の語彙」として使い、表（名前 → 番号 → 文字）は自分で書く。vial-gui のファイルは写さない → 「名前の一覧は著作物に当たらない」という立場を取ることになる。ライセンスは、あとで自由に決められる
  - b. vial-gui の `keycodes.py` と `keycodes_v6.py` を元に、表を作る（写す・機械的に変える）→ このリポジトリは GPL-2.0-or-later にすることになる。全部の名前が最初からそろう
- 推奨: 付けない（ライセンスの判断を含むため。決めるのはえだ）

**Q-12（j）: 公開までの手順は、どれにしますか。**
- 事実: 5.2節と 5.3節。リポジトリは公開で、Pages が使える [A]。wasm-pack は、この PC でビルドが通った [A]。マインスイーパー2号機は Actions を使っていない [A]。
- 選択肢: a. GitHub Actions で公開する（main に入ると自動で公開。えだは最初に1回、Pages の Source を「GitHub Actions」にする）／ b. えだが手元でビルドして、gh-pages ブランチに置く ／ c. 手元でビルドした物を main に入れて、ブランチから公開する
- 推奨: a（「main に入れる」と「公開する」が1つの操作になり、ビルドの結果を git に入れずに済むため）

**Q-13（k）: 周ごとにブランチと PR を使いますか。main に直接積みますか。**
- 事実: edaberu は、周ごとのブランチと PR [A]。マインスイーパー2号機は main に直接 [B・チャット側の報告]。このリポジトリの CLAUDE.md は「main への push と合流は、えだが差分を読んでから行う」。Q-12 が a なら、main に入った時点で公開される。
  - → 2026-10-03 17:11 の commit dc91588 で、commit と push は Claude Code の担当に変更。今の文は CLAUDE.md の Workflow が正
- 選択肢: a. 周ごとにブランチと PR（Claude Code がブランチへ push して PR を作り、えだが読んで合流する）／ b. main に直接積む（えだが push する）
- 推奨: a（Q-12 の a と組むと、周の途中の物は公開されず、えだが合流したときだけ公開されるため。CLAUDE.md の Workflow に、edaberu と同じ1行を足すことになるので、足す前に報告する）

**Q-14: 初期キーマップ以外は、どうやって試しますか。**
- 事実: 初期キーマップのままでは記号が足りない [B・2.4節]。えだは実機が無いので、Vial で .vil を保存できない。Vial のデスクトップ版には、機種定義の JSON を読んで実機なしで編集する「Load dummy JSON」がある [A・ソース]。保存した .vil は、レイヤーが4つ・uid が 0 になる見込み [B・動かしていない]。.vil は JSON なので、テキストエディタでも直せる [A]。
- 選択肢: a. 1周目は「.vil を読む」だけにする。自分のキーマップは、.vil をエディタで直すか、Vial の dummy で作って読み込む（そのために、レイヤーの数は 10 に決め打ちせず、uid の値は見ない）／ b. ページの中で、キーの割り当てを変えられるようにする（.vil の書き出しも要る）
- 推奨: a（1周目のゴール（D-6）のまま進められるため。b は、a で足りないと分かってから周を足す）

**Q-15: キーの刻印は、どう出しますか。**
- 事実: Vial の画面は、キーコードを US の刻印で出す。Keyboard Layout を Japanese にすると、刻印だけが JIS に変わる [A・3.5節]。下段は、手元のキーと「同じ位置」にならない（Q-6）。
- 選択肢（文字）: a. Q-2 で選んだ配列での文字を出す（下に素の文字、上に Shift 側の文字。英字は大文字1つ）。文字でないキーは、短い名前（Tab・Bksp・MO(1) など）／ b. Vial と同じ US の刻印に固定する
- 選択肢（手元のキーの名前）: 1. Cornix のキーの隅に、手元のキーの名前（例: 無変換）を小さく出す。画面で出す・消すを切り替えられる（初期値は出す）／ 2. 出さない
- 推奨: a と 1（出る文字と刻印が一致し、どのキーで押すのかが図で分かるため）

**Q-16: MO を押している間の表示と、レイヤーを眺める方法は、どうしますか。**
- 事実: キーは1つずつ、有効なレイヤーを上から見て決まる（D-12）。MO を2つ押していると、有効なレイヤーは3つになる。手元のキーが足りないと、押せない MO が出る（Q-6）。
- 選択肢（押している間）: a. キーごとに「いま押したら決まるキーコード」を出す（KC_TRNS のキーは、下のレイヤーのキーコードを薄く出す。KC_NO は空）。有効なレイヤーの番号も出す ／ b. いちばん上の有効なレイヤーを、そのまま出す（KC_TRNS は ▽）
- 選択肢（眺める）: 1. レイヤーの番号のボタンを付け、押さなくても、各レイヤーの中身を見られる（打った結果は変えない）／ 2. 付けない
- 推奨: a と 1（「押すと何が出るか」が図と一致し、押せない MO の先のレイヤーも確かめられるため）

**Q-17: キーを押し続けたときの繰り返しを、再現しますか。**
- 事実: 実機では、キーを押し続けると、PC の側が文字を繰り返す。ブラウザは、押し続けの間、keydown を repeat 付きで送ってくる [B]。レイヤーの処理では、repeat を「押し直し」にしない（D-12 の (3)）。
- 選択肢: a. 再現する（押し続けると、押したときに決まったキーコードの文字が続けて出る。Backspace も続けて消える）／ b. 再現しない（1回押すと、1回だけ）
- 推奨: a（Backspace の押しっぱなしなど、ふだんの打ち方のまま試せるため。レイヤーと修飾キーの状態は、repeat では変えない）

## 10. 調べ方の記録

- WebSearch は2回、WebFetch は0回（上限は合わせて10回）。検索で見つけた物は、本文を curl で取り直して読んだ（S15 と 4.2節のブログ）。
- git clone（浅い clone）: S3・S7（タグと main を追加で fetch）・S9・S10。
- curl: S1・S2・S4・S5・S6・S8・S11・S12・S13・S14・S15・S16・S17 と、`local/` の3ファイル。
- S5 が「Some documentation」として挙げている https://www.elimkeys.com/en/doc/cornix/ は、404 だった（2026-10-03）。
