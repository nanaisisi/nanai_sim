#概要 牧歌的シミュレーション

#使用コマンド

<code>cargo update</code><br />
<code>cargo fmt</code><br />
<code>cargo clippy</code><br />
<code></code>cargo run --features bevy/dynamic_linking</code><br />

#主要機能

bot用<br /> 農業<br /> その他<br /> スクエアタイル<br />
1秒以上演算ターン制<br /> 永続継続対応<br /> 他マップ設計<br />
ブロック内マップ(補助)<br />

#システム

クロスプラットフォーム<br /> ローカルネットでの高いデータ移行性<br />
botが高度に処理<br />
loneboth_ai用のコードをバージョン表記し、配置。適応データはlonebothに格納<br />

#グラフィック

白黒<br />

#ターン詳細 1秒以上、ターン数を保存<br />
開始時とバージョン更新時間、現在時刻(変化する変数)。各時点ターン数を提供。<br />
元号リストを開始時の元号から提供。<br />
