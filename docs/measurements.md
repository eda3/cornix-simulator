# 実測の記録

えだが自分の PC で測った値と、見えたことを書く場所です。測った値だけを書きます（推測は書きません。書く場合は［推測］と付けます）。値は、測った日と一緒に書きます。

表の枠は、先に作ってあります。空の欄は、まだ測っていない所です。code や key の欄は、計測ページ（`keytest.html`）に出た値を、そのまま書きます。

## ① キーの届き方（1周目。`docs/design.md` 6節の M-1〜M-3）

- 測った日: 2026-10-03（出どころ: えだの申告）
- OS: Microsoft Windows 11 Home、バージョン 25H2（10.0.26200）、ビルド番号 26200.9550（出どころ: `Get-CimInstance Win32_OperatingSystem` の Caption・Version・BuildNumber と、`Get-ItemProperty 'HKLM:\SOFTWARE\Microsoft\Windows NT\CurrentVersion'` の DisplayVersion・UBR。2026-10-03 19:02 に読んだ）
- ブラウザと版: Brave 154.1.96.61（Chromium 系。測ったのが Brave であることは、えだの申告）（出どころ: `(Get-Item "$env:LOCALAPPDATA\BraveSoftware\Brave-Browser\Application\brave.exe").VersionInfo.ProductVersion`。動いていた brave のプロセスの ProductVersion も同じ。2026-10-03 19:02 に読んだ）
- IME（Microsoft IME・Google 日本語入力 など）: Google 日本語入力（測ったときに選んでいた物。えだの申告）。有効な入力方式は2つで、並びは Microsoft IME、Google 日本語入力（Google Japanese Input）の順（出どころ: `Get-WinUserLanguageList` の InputMethodTips と、`HKLM:\SOFTWARE\Microsoft\CTF\TIP\{CLSID}\LanguageProfile\0x00000411\{GUID}` の Description。2026-10-03 19:02 に読んだ）
- キーボード（型番。Windows 用か Mac 用か）: PnP の名前は「REALFORCE 114 JP」（`USB\VID_0853&PID_0149`。その下に「HID キーボード デバイス」が2つ）。型番は、PC からは読めなかった（出どころ: `Get-PnpDevice -Class Keyboard -PresentOnly` と、親の USB 機器の `Get-PnpDeviceProperty -KeyName DEVPKEY_Device_BusReportedDeviceDesc`。2026-10-03 19:02 に読んだ）。えだの申告: 「REALFORCE Mac 用 JIS。Windows で使用。下段の刻印は左から Capslock・option・command・英数・space・かな・command・option・control・fn」。キーボードの種類の機器は、ほかに2つある（「TB550 2.4GHz Receiver」`USB\VID_047D&PID_80FD` と、「USB Gaming Mouse」`USB\VID_04D9&PID_FC4D`。どちらも、その下に「HID キーボード デバイス」が1つ）
- keyhac: 止めて測る（えだの決定・2026-10-03）。Scancode Map（レジストリにある、キーの入れ替え）は3つ: スキャンコード 0x0038（AltLeft）のキー → 0xE05B（MetaLeft）、0xE05B（MetaLeft）のキー → 0x0038（AltLeft）、0x003A（CapsLock）のキー → 0x0001（Escape）。値は `00 00 00 00 00 00 00 00 04 00 00 00 5B E0 38 00 38 00 5B E0 01 00 3A 00 00 00 00 00`（出どころ: `Get-ItemProperty 'HKLM:\SYSTEM\CurrentControlSet\Control\Keyboard Layout' -Name 'Scancode Map'`。2026-10-03 19:02 に読んだ。解読は Claude Code。スキャンコードと code の対応は、MDN の Windows の表を 2026-10-03 に見た [A]。どちらが元のキーかは、Scancode Map の書式の知識 [B]）。えだの申告: 「KeySwap で左下（刻印 Capslock）→ Esc」。keyhac.exe は、測ったときは終了していた（えだの申告）。プロセスは、2026-10-03 19:02:59 には動いていて（PID 34752）、19:04:38 と 19:05:07 には動いていなかった（出どころ: `Get-Process` と `Get-CimInstance Win32_Process`）。測ったあとで起動し、そのあと終了した（えだの申告）

押すキー（1つずつ押して、離す。計測ページの下に出る Markdown の表を、そのまま貼ってよい）:

1. 下段の全部（左端から右端へ）
2. 上の3段の端と記号: Tab・英数・左 Shift・右 Shift・@・[・;・:・]・,・.・/・ろ
3. 半角/全角・かな（カタカナ ひらがな）を、2回ずつ
4. 文字のキーの代表: Q・A・Z
5. Enter・Backspace・Space
6. 数字の段の右の3つ: -・^・¥

### keyhac を止めたとき

①下段の全部（左端から右端へ）

| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |
|---|---|---|---|---|---|---|---|
| Capslock | Escape | Escape | 0 | 1 | 1 | 0 | KeySwap で Esc |
| option | MetaLeft | Meta | 1 | 1 | 0 | 0 | |
| command | AltLeft | Alt | 1 | 1 | 1 | 0 | |
| 英数 | NonConvert | NonConvert | 0 | 1 | 1 | 0 | |
| space | Space | （空白） | 0 | 1 | 1 | 0 | |
| かな | Convert | Convert | 0 | 0 | 1 | 0 | |
| command（右） | ContextMenu | ContextMenu | 0 | 1 | 1 | 0 | |
| option（右） | AltRight | Alt | 2 | 1 | 1 | 0 | |
| control（右） | ControlRight | Control | 2 | 1 | 1 | 0 | |

②上の3段の端と記号: Tab・英数・左 Shift・右 Shift・@・[・;・:・]・,・.・/・ろ

| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |
|---|---|---|---|---|---|---|---|
| | Tab | Tab | 0 | 1 | 1 | 0 | |
| | KeyQ | q | 0 | 1 | 1 | 0 | |
| | KeyW | w | 0 | 1 | 1 | 0 | |
| | KeyE | e | 0 | 1 | 1 | 0 | |
| | KeyR | r | 0 | 1 | 1 | 0 | |
| | KeyT | t | 0 | 1 | 1 | 0 | |
| | KeyY | y | 0 | 1 | 1 | 0 | |
| | KeyU | u | 0 | 1 | 1 | 0 | |
| | KeyI | i | 0 | 1 | 1 | 0 | |
| | KeyO | o | 0 | 1 | 1 | 0 | |
| | KeyP | p | 0 | 1 | 1 | 0 | |
| | BracketLeft | @ | 0 | 1 | 1 | 0 | |
| | BracketRight | [ | 0 | 1 | 1 | 0 | |
| | ControlLeft | Control | 1 | 1 | 1 | 0 | |
| | KeyA | a | 0 | 1 | 1 | 0 | |
| | KeyS | s | 0 | 1 | 1 | 0 | |
| | KeyD | d | 0 | 1 | 1 | 0 | |
| | KeyF | f | 0 | 1 | 1 | 0 | |
| | KeyG | g | 0 | 1 | 1 | 0 | |
| | KeyH | h | 0 | 1 | 1 | 0 | |
| | KeyJ | j | 0 | 1 | 1 | 0 | |
| | KeyK | k | 0 | 1 | 1 | 0 | |
| | KeyL | l | 0 | 1 | 1 | 0 | |
| | Semicolon | ; | 0 | 1 | 1 | 0 | |
| | Quote | : | 0 | 1 | 1 | 0 | |
| | Backslash | ] | 0 | 1 | 1 | 0 | |
| | ShiftLeft | Shift | 1 | 1 | 1 | 0 | |
| | KeyZ | z | 0 | 1 | 1 | 0 | |
| | KeyX | x | 0 | 1 | 1 | 0 | |
| | KeyC | c | 0 | 1 | 1 | 0 | |
| | KeyV | v | 0 | 1 | 1 | 0 | |
| | KeyB | b | 0 | 1 | 1 | 0 | |
| | KeyN | n | 0 | 1 | 1 | 0 | |
| | KeyM | m | 0 | 1 | 1 | 0 | |
| | Comma | , | 0 | 1 | 1 | 0 | |
| | Period | . | 0 | 1 | 1 | 0 | |
| | Slash | / | 0 | 1 | 1 | 0 | |
| | IntlRo | \ | 0 | 1 | 1 | 0 | |
| | ShiftRight | Shift | 2 | 1 | 1 | 0 | |

③半角/全角・かな（カタカナ ひらがな）を、2回ずつ

| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |
|---|---|---|---|---|---|---|---|
| | Backquote | Hankaku Zenkaku | 0 | 0 | 2 | 0 | |
| | NonConvert | NonConvert | 0 | 2 | 2 | 0 | |
| | Convert | Convert | 0 | 0 | 2 | 0 | |

④文字のキーの代表: Q・A・Z

| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |
|---|---|---|---|---|---|---|---|
| | KeyQ | q | 0 | 1 | 1 | 0 | |
| | KeyA | a | 0 | 1 | 1 | 0 | |
| | KeyZ | z | 0 | 1 | 1 | 0 | |

⑤Enter・Backspace・Space

| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |
|---|---|---|---|---|---|---|---|
| | Enter | Enter | 0 | 1 | 1 | 0 | |
| | Backspace | Backspace | 0 | 1 | 1 | 0 | |
| | Space | （空白） | 0 | 1 | 1 | 0 | |

⑥数字の段の右の3つ: -・^・¥

| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |
|---|---|---|---|---|---|---|---|
| | Minus | - | 0 | 1 | 1 | 0 | |
| | Equal | ^ | 0 | 1 | 1 | 0 | |
| | IntlYen | \ | 0 | 1 | 1 | 0 | |

### 見えたこと

- keyup が来なかったキー（keydown と keyup の回数が合わないキー）: keyup が来なかった（keydown より keyup が少ない）のは、MetaLeft（①の表で keydown 1・keyup 0）。keydown が来なかった（keydown より keyup が多い）のは、Convert（①の表で keydown 0・keyup 1。③の表で keydown 0・keyup 2）と、Backquote（③の表で keydown 0・keyup 2）
- code が空で届いたキー: なし
- まったく届かなかったキー: fn（押したが、何も出なかった。えだの申告）。カタカナひらがな のキーは、このキーボードに無い（えだの申告）
- 文字のキーを押し続けたとき、repeat が数えられたか: 表からは分からない（6つの表の repeat は、どの行も 0。押し続けを試したかどうかは [未確認]）
- 素の JIS との差: 左下＝Escape、A の左＝ControlLeft（刻印 control）、カタカナひらがな 無し、無変換・変換の刻印は 英数・かな、左の option/command が Meta/Alt、右の command が ContextMenu（えだの申告と表から）

## ② 止められるか（1周目。design 6節の M-5）

計測ページの「preventDefault する」を入れたまま、ふつうの窓で試す。keyhac を止めて測る。Ctrl+W・Ctrl+T・Ctrl+N は、ここでは押さない。

- 測った日: 2026-10-03（Brave。keyhac は止めた状態。「preventDefault する」は入り。えだの申告）

| 操作 | ブラウザや Windows が動いたか（動いたときは、その内容） | ページに届いた code |
|---|---|---|
| Tab | 動かなかった | Tab |
| Space | 動かなかった | Space |
| ↑・↓ | 動かなかった | ArrowUp・ArrowDown |
| Alt を単独で押して、離す | 動かなかった | AltLeft |
| Alt を押したまま D | 動かなかった | AltLeft・KeyD |
| Alt を押したまま F | 動かなかった | AltLeft・KeyF |
| F5 | 動かなかった | F5 |
| F10 | 動かなかった | F10 |
| F12 | 動かなかった | F12 |
| Ctrl+L | 動かなかった | ControlLeft・KeyL |
| Ctrl+F | 動かなかった | ControlLeft・KeyF |
| Ctrl+P | 動かなかった | ControlLeft・KeyP |
| Win を単独で押して、離す | 動いた（スタートメニューが開いた） | MetaLeft（keydown のみ。keyup は届かず） |
| アプリケーションキー | 動かなかった（メニューは出なかった）。手元では、右の command（刻印）のキー。2026-10-05 に測った | ContextMenu（keydown・keyup とも届いた） |

出どころ（計測ページが出した表）:

| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |
|---|---|---|---|---|---|---|---|
| | Tab | Tab | 0 | 1 | 1 | 0 | |
| | Space | （空白） | 0 | 1 | 1 | 0 | |
| | ArrowUp | ArrowUp | 0 | 1 | 1 | 0 | |
| | ArrowDown | ArrowDown | 0 | 1 | 1 | 0 | |
| | AltLeft | Alt | 1 | 3 | 3 | 23 | |
| | KeyD | d | 0 | 1 | 1 | 0 | |
| | KeyF | f | 0 | 2 | 2 | 0 | |
| | F5 | F5 | 0 | 1 | 1 | 0 | |
| | F10 | F10 | 0 | 1 | 1 | 0 | |
| | F12 | F12 | 0 | 1 | 1 | 0 | |
| | ControlLeft | Control | 1 | 4 | 3 | 1 | |
| | KeyL | l | 0 | 1 | 1 | 0 | |
| | KeyP | p | 0 | 1 | 1 | 0 | |
| | MetaLeft | Meta | 1 | 1 | 0 | 0 | |

- アプリケーションキーの行は、2026-10-05 に測った（Brave。keyhac は止めた状態。「preventDefault する」は入り。開発者ツールの Console からは、何も足していない。えだの申告）。計測ページを開いていない所で同じキーを押すと、メニューが出た（えだの申告）。Brave の版は 154.1.96.61（出どころ: `(Get-Item "$env:LOCALAPPDATA\BraveSoftware\Brave-Browser\Application\brave.exe").VersionInfo.ProductVersion`。動いていた brave のプロセスの ProductVersion も同じ。2026-10-05 14:26 に読んだ）

出どころ（2026-10-05。計測ページが出した表）:

| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |
|---|---|---|---|---|---|---|---|
| | ContextMenu | ContextMenu | 0 | 1 | 1 | 0 | |

## ③ 全画面＋Keyboard Lock（1周目。design 6節の M-6）

計測ページの「全画面＋Keyboard Lock」を押してから試す。

- 測った日: 2026-10-03（keyhac は止めた状態。えだの申告）
- 測ったブラウザ: Edge 154.0.4258.53（測ったのが Edge であることは、えだの申告。版の出どころ: `(Get-Item "${env:ProgramFiles(x86)}\Microsoft\Edge\Application\msedge.exe").VersionInfo.ProductVersion`。動いていた msedge のプロセスの ProductVersion も同じ。2026-10-03 19:37 に読んだ）。Brave 154.1.96.61 でも試した（えだの申告）
- 許可の確認が出たか: Edge＝出なかった。Brave＝出なかった（ブラウザの全画面の帯「全画面を終了するには Esc を押します」だけ）
- Keyboard Lock が掛かったと、ページに出たか: Edge＝「Keyboard Lock が掛かりました」。Brave＝「Keyboard Lock は掛けていません」のまま（ボタンの文は「全画面をやめる」に変わった）。`ad7ac1e` のあとは、Brave でも「Keyboard Lock を掛けられませんでした（このブラウザには navigator.keyboard がありません）」と出た（えだの申告・2026-10-03 19:55）
- 全画面の抜け方: 2026-10-03 の Edge は、マウスを画面の上へ動かすと出るボタンで抜けた（えだの申告。2026-10-05 に聞いた）。2026-10-05 にも、全画面をマウスで抜けた（えだの申告）。2026-10-03 の記録の時点では、まだ全画面のままだった（別のディスプレイの窓で作業中）

下の表は、Edge で、全画面＋Keyboard Lock のとき。ページにフォーカスがある状態で押した（えだの申告）。

| 操作 | ブラウザや Windows が動いたか（動いたときは、その内容） | ページに届いた code |
|---|---|---|
| Win を単独で押して、離す | 動かなかった（スタートメニューは開かなかった） | MetaLeft（keydown・keyup とも届いた） |
| Win を押したまま D | 動かなかった（デスクトップに切り替わらなかった） | MetaLeft・KeyD |
| Alt を押したまま Tab | 動かなかった（窓は切り替わらなかった） | AltLeft・Tab |
| Ctrl+W | 動かなかった（タブは閉じなかった。表が残った） | ControlLeft・KeyW |
| Ctrl+T | 動かなかった（新しいタブは開かなかった） | ControlLeft・KeyT |
| Ctrl+N | 動かなかった（新しい窓は開かなかった） | ControlLeft・KeyN |
| Esc を短く押す | 全画面は解けなかった | Escape |
| Esc を2秒押す | 全画面は解けなかった（2秒では。Edge は Esc の長押しで抜けられる、はえだの知識） | Escape |

出どころ（Edge。計測ページが出した表）:

| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |
|---|---|---|---|---|---|---|---|
| | MetaLeft | Meta | 1 | 2 | 2 | 0 | |
| | KeyD | d | 0 | 1 | 1 | 0 | |
| | AltLeft | Alt | 1 | 1 | 1 | 3 | |
| | Tab | Tab | 0 | 1 | 1 | 0 | |
| | ControlLeft | Control | 1 | 3 | 3 | 0 | |
| | KeyW | w | 0 | 1 | 1 | 0 | |
| | KeyT | t | 0 | 1 | 1 | 0 | |
| | KeyN | n | 0 | 1 | 1 | 0 | |
| | Escape | Escape | 0 | 3 | 3 | 0 | |

## ④ JIS の文字（1周目。design 6節の M-4）

キーを、そのままと、Shift を押しながらで押して、計測ページの key の値を書く。design 3.2節の JIS の列と見比べる。

- 測った日: 2026-10-05（Brave。keyhac は止めた状態。えだの申告）。Brave の版は 154.1.96.61（出どころ: `(Get-Item "$env:LOCALAPPDATA\BraveSoftware\Brave-Browser\Application\brave.exe").VersionInfo.ProductVersion`。動いていた brave のプロセスの ProductVersion も同じ。2026-10-05 14:09 に読んだ）

| 手元の刻印 | code | key（そのまま） | key（Shift） | design 3.2節と同じか |
|---|---|---|---|---|
| 1 | Digit1 | 1 | ! | 同じ |
| 2 | Digit2 | 2 | " | 同じ |
| 6 | Digit6 | 6 | & | 同じ |
| 7 | Digit7 | 7 | ' | 同じ |
| 8 | Digit8 | 8 | ( | 同じ |
| 9 | Digit9 | 9 | ) | 同じ |
| 0 | Digit0 | 0 | 0 | key は違う（design の Shift は「なし」、key は `0`）。メモ帳では、何も打たれなかった |
| - | Minus | - | = | 同じ |
| ^ | Equal | ^ | ~ | 同じ |
| ¥ | IntlYen | \ | \| | 同じ |
| @ | BracketLeft | @ | ` | 同じ |
| [ | BracketRight | [ | { | 同じ |
| ; | Semicolon | ; | + | 同じ |
| : | Quote | : | * | 同じ |
| ] | Backslash | ] | } | 同じ |
| , | Comma | , | < | 同じ |
| . | Period | . | > | 同じ |
| / | Slash | / | ? | 同じ |
| ろ | IntlRo | \ | _ | 同じ |
| 半角/全角 | Backquote | Hankaku（keydown 0・keyup 1） | Zenkaku（keydown 0・keyup 1） | 文字は出ていない（design も「なし」） |

- メモ帳での確かめ（2026-10-05。えだの申告）: IME を切って Shift+0 を押すと、何も打たれなかった。計測ページの key は `0` なので、このキーの Shift では、key の値と、打たれる文字が同じでない
- 半角/全角のキー: 2回とも、keydown は届かず、keyup だけ届いた。key は、そのままの回が Hankaku、Shift の回が Zenkaku。①の「③半角/全角・かな」の表では、Shift なしの2回で、Hankaku と Zenkaku の両方が出ている。値が Shift で変わったのか、押すたびに入れ替わるのかは、この測り方では分からない

出どころ（計測ページが出した表。そのまま押した回）:

| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |
|---|---|---|---|---|---|---|---|
| | Digit1 | 1 | 0 | 1 | 1 | 0 | |
| | Digit2 | 2 | 0 | 1 | 1 | 0 | |
| | Digit3 | 3 | 0 | 1 | 1 | 0 | |
| | Digit4 | 4 | 0 | 1 | 1 | 0 | |
| | Digit5 | 5 | 0 | 1 | 1 | 0 | |
| | Digit6 | 6 | 0 | 1 | 1 | 0 | |
| | Digit7 | 7 | 0 | 1 | 1 | 0 | |
| | Digit8 | 8 | 0 | 1 | 1 | 0 | |
| | Digit9 | 9 | 0 | 1 | 1 | 0 | |
| | Digit0 | 0 | 0 | 1 | 1 | 0 | |
| | Minus | - | 0 | 1 | 1 | 0 | |
| | Equal | ^ | 0 | 1 | 1 | 0 | |
| | IntlYen | \ | 0 | 1 | 1 | 0 | |
| | BracketLeft | @ | 0 | 1 | 1 | 0 | |
| | BracketRight | [ | 0 | 1 | 1 | 0 | |
| | Semicolon | ; | 0 | 1 | 1 | 0 | |
| | Quote | : | 0 | 1 | 1 | 0 | |
| | Backslash | ] | 0 | 1 | 1 | 0 | |
| | Comma | , | 0 | 1 | 1 | 0 | |
| | Period | . | 0 | 1 | 1 | 0 | |
| | Slash | / | 0 | 1 | 1 | 0 | |
| | IntlRo | \ | 0 | 1 | 1 | 0 | |
| | Backquote | Hankaku | 0 | 0 | 1 | 0 | |

出どころ（計測ページが出した表。Shift を押しながら押した回）:

| 手元の刻印 | code | key | location | keydown | keyup | repeat | メモ |
|---|---|---|---|---|---|---|---|
| | ShiftLeft | Shift | 1 | 1 | 1 | 64 | |
| | Digit1 | ! | 0 | 1 | 1 | 0 | |
| | Digit2 | " | 0 | 1 | 1 | 0 | |
| | Digit3 | # | 0 | 1 | 1 | 0 | |
| | Digit4 | $ | 0 | 1 | 1 | 0 | |
| | Digit5 | % | 0 | 1 | 1 | 0 | |
| | Digit6 | & | 0 | 1 | 1 | 0 | |
| | Digit7 | ' | 0 | 1 | 1 | 0 | |
| | Digit8 | ( | 0 | 1 | 1 | 0 | |
| | Digit9 | ) | 0 | 1 | 1 | 0 | |
| | Digit0 | 0 | 0 | 1 | 1 | 0 | |
| | Minus | = | 0 | 1 | 1 | 0 | |
| | Equal | ~ | 0 | 1 | 1 | 0 | |
| | IntlYen | \| | 0 | 1 | 1 | 0 | |
| | BracketLeft | ` | 0 | 1 | 1 | 0 | |
| | BracketRight | { | 0 | 1 | 1 | 0 | |
| | Semicolon | + | 0 | 1 | 1 | 0 | |
| | Quote | * | 0 | 1 | 1 | 0 | |
| | Backslash | } | 0 | 1 | 1 | 0 | |
| | Comma | < | 0 | 1 | 1 | 0 | |
| | Period | > | 0 | 1 | 1 | 0 | |
| | Slash | ? | 0 | 1 | 1 | 0 | |
| | IntlRo | _ | 0 | 1 | 1 | 0 | |
| | Backquote | Zenkaku | 0 | 0 | 1 | 0 | |

## ⑤ ブラウザで見る項目（3周目。`docs/test-items.md` の⑭〜⑱）

| 番号 | 日 | 結果 | 見えたこと |
|---|---|---|---|
| ⑭ | | | |
| ⑮ | | | |
| ⑯ | | | |
| ⑰ | | | |
| ⑱ | | | |

## ⑥ 公開（4周目。`docs/test-items.md` の⑲）

- Pages の Source を「GitHub Actions」にした日:
- Actions の実行の結果（失敗したときは、ログの先頭20行）:

| 番号 | 日 | 結果 | 見えたこと |
|---|---|---|---|
| ⑲ | | | |
