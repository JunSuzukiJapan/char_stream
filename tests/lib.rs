extern crate char_stream;

use char_stream::CharStream;

#[test]
fn test_rev() {
    // [言語処理100本ノック 2015](http://www.cl.ecei.tohoku.ac.jp/nlp100/)
    // 00. 文字列の逆順
    // 文字列"stressed"の文字を逆に（末尾から先頭に向かって）並べた文字列を得よ．
    let input = "stressed";
    let stream = CharStream::from(input);
    let rev_stream = stream.wend_iter().rev();
    let result: String = rev_stream.collect();
    assert_eq!("desserts", result);
}

#[test]
fn test_nth() {
    // [言語処理100本ノック 2015](http://www.cl.ecei.tohoku.ac.jp/nlp100/)
    // 01. 「パタトクカシーー」
    //「パタトクカシーー」という文字列の1,3,5,7文字目を取り出して連結した文字列を得よ．
    let input = "パタトクカシーー";
    let mut stream = CharStream::from(input);
    let c1 = stream.nth(0).unwrap();
    let c3 = stream.nth(1).unwrap();
    let c5 = stream.nth(1).unwrap();
    let c7 = stream.nth(1).unwrap();
    let result = format!("{}{}{}{}", c1, c3, c5, c7);
    assert_eq!("パトカー", result);
}

#[test]
fn test_zip() {
    // [言語処理100本ノック 2015](http://www.cl.ecei.tohoku.ac.jp/nlp100/)
    // 02. 「パトカー」＋「タクシー」＝「パタトクカシーー」
    // 「パトカー」＋「タクシー」の文字を先頭から交互に連結して文字列「パタトクカシーー」を得よ．
    let input1 = "パトカー";
    let input2 = "タクシー";
    let stream1 = CharStream::from(input1);
    let stream2 = CharStream::from(input2);
    let mut ziped_srream = stream1.zip(stream2);
    let mut result = String::new();
    while let Some((c1, c2)) = ziped_srream.next() {
        result.push(c1);
        result.push(c2);
    }

    assert_eq!("パタトクカシーー", result);
}