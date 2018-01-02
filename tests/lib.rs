extern crate char_stream;

use char_stream::CharStream;

#[test]
fn test_nth() {
    // [言語処理100本ノック 2015](http://www.cl.ecei.tohoku.ac.jp/nlp100/)
    //   第1章.01 「パタトクカシーー」という文字列の1,3,5,7文字目を取り出して連結した文字列を得よ．
    let input = "パタトクカシーー";
    let mut stream = CharStream::from(input);
    let c1 = stream.nth(0).unwrap();
    let c3 = stream.nth(1).unwrap();
    let c5 = stream.nth(1).unwrap();
    let c7 = stream.nth(1).unwrap();
    let result = format!("{}{}{}{}", c1, c3, c5, c7);
    assert_eq!("パトカー", result);
}