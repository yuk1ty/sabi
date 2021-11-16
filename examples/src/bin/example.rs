sabi::sabi! {
    外部 箱 sabi;


使う std::collections::HashMap as 辞書;

束 値束{
    関数 書く(&自身, 目次: 文字列, 内容: 文字列);
    関数 取得(&自身, 目次: 文字列) -> 結果<オプション<&文字列>, 文字列>;
}

静的 可変 静的可変辞書: オプション<辞書<文字列, 文字列>> = ない;

構造体 もの;

実装 値束 に対して もの {
    関数 書く(&自身, 目次: 文字列, 内容: 文字列){
         束縛 辞書データ = 安全じゃない { 静的可変辞書.取得または挿入(デフォルト::デフォ) };
              辞書データ.挿入(目次, 内容);
    }
    関数 取得(&自身, 目次: 文字列) -> 結果<オプション<&文字列>, 文字列> {
        もし 束縛 ある(辞書データ) = 安全じゃない { 静的可変辞書.as_ref() } {
             よし(辞書データ.取得(&目次))
        } でないならば {
              ダメ("辞書を取ってくる".into())
        }
    }
}
公開(箱) 関数 かもしれない(i: u32) -> オプション<結果<u32, 文字列>> {
    もし i % 2 == 1 {
        もし i == 42 {
                ある(ダメ(文字列::から("クソッ")))
        } でないならば {
                ある(よし(33))
        }
    } でないならば {
                ない
    }
}

非同期 関数 例(){}
非同期 関数 例2(){
    例().待つ;
}

関数 メイン() {
    束縛 可変 x = 31;
    マッチ x {
        42 => {
            書き出す!("寿司")
        }
        _ => 書き出す!("こんな感じです"),
}

    繰り返し i 中 0..10 {
        束縛 値 = ループ {
            止める i;
        };
        間 x < 値 {
            x += 1;
        }

        x = もし 束縛 ある(返り値) = かもしれない(i) {
            返り値.空ける()
        } でないならば {
            12
        };
    }
}
}
