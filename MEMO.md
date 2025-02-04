`Result`

https://doc.rust-jp.rs/rust-by-example-ja/error/result.html

- Rust の標準ライブラリで定義された列挙型 (enum)。
- 通常は結果やエラーを扱うために使用する。

``` rust
enum Rusult<T, E> { // T: 成功時の値の型, E: 失敗時の値の型
    Ok(T),
    Err(E),
}
```

`Box`

https://doc.rust-jp.rs/rust-by-example-ja/std/box.html

- ヒープメモリにデータを格納するためのスマートポインタ。
- 実行時にサイズが決まる場合に型をエレガントに表現できる。

`dyn`

https://doc.rust-jp.rs/rust-by-example-ja/trait/dyn.html

- トレイトオブジェクトを返すときに `Box` で使用するキーワード。
- `Box<dyn XXX>` とすることでトレイト `XXX` を実装する任意の型を表せる。

`Result<(), Box<dyn Error>>`

- タスクが成功した場合は値がないことを示す `()` を返す。
- タスクが失敗した場合は `std::error::Error` トレイトを実装するエラーを返す。
  - IO エラーや解析エラーなど異なるタイプのエラーをひとつの型として扱える。
  - エラーの詳細情報を返すことができる。
  - プログラムが適切に終了し、エラーメッセージが表示されるようにできる。

``` rust
// 使用例
fn main() ->  Result<(), Box<dyn Error>> {
    let result = some_function_that_may_fail();
    result.map_err(|e| Box::new(e) as Box<dyn Error>)?; // 最後の ? でエラー移譲するとシンプルになる
    Ok(())
}
```

`VecDeque`

- 双方向キューを実装するためのデータ構造
- 前後の要素を追加・削除する操作が効率的に行える
- ランダムアクセスは効率的でない
