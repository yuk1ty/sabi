// shamelessly copied from https://github.com/bnjbvr/rouille/blob/principale/rouille_proc_macro/src/lib.rs
// Thank you to the authors!

use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "ダメ" => "Err",
        "よし" => "Ok",
        "文字列" => "String",
        "辞書" => "HashMap",
        "デフォルト" => "Default",
        "エラー" => "Error",
        "オプション" => "Option",
        "ある" => "Some",
        "ない" => "None",
        "結果" => "Result",
        "セルフ" => "Self",
        "書き出す" => "println",
        "止める" => "break",
        "非同期" => "async",
        "待つ" => "await",
        "ループ" => "loop",
        "ムーブ" => "move",
        "箱" => "crate",
        "として" => "as",
        "定数" => "const",
        "マッチ" => "match",
        "安全じゃない" => "unsafe",
        "中" => "in",
        "から" => "from",
        "動的" => "dyn",
        "空ける" => "unwrap",
        "デフォ" => "default",
        "入出力" => "io",
        "外部" => "extern",
        "偽" => "false",
        "関数" => "fn",
        "親" => "super",
        "実装" => "impl",
        "挿入" => "insert",
        "束" => "trait",
        "取得" => "get",
        "モジュール" => "mod",
        "可変" => "mut",
        "新" => "new",
        "条件" => "where",
        "繰り返し" => "for",
        "に対して" => "for",
        "取得または挿入" => "get_or_insert_with",
        "メイン" => "main",
        "公開" => "pub",
        "ない？" | "ない?" => None?,
        "返す" => "return",
        "もし" => "if",
        "でないならば" => "else",
        "自身" => "self",
        "束縛" => "let",
        "静的" => "static",
        "構造体" => "struct",
        "期待する" => "expect",
        "間" => "while",
        "使う" => "use",
        "真" => "true",
        "列挙" => "enum",
        "文字列に変換する" => "to_string",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn sabi(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
