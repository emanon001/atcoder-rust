# atcoder-rust

[AtCoder](https://atcoder.jp)の解答一覧。

## Usage

### コンテスト用のプロジェクトを作成

```
bin/new <contest_id>
```

ルートディレクト配下に`<contest_id>`というディレクトリが作成されます。
また、専用のブランチが作成され、対象ブランチにいるときはコンテストに参加中の扱いになります。
後述するGitフックを使用することで、コンテスト参加中のリモートブランチへのpushを防ぐことが可能です。

### コードスニペットを作成

```
competitive/bin/gen-code-snippets
```

### コードスニペットを任意コンテストのプロジェクトにコピー

```
bin/cp-code-snippets <contest_id>
```

`bin/new`の中でコードスニペットのコピーも実行されます。
コードスニペットを最新の内容に更新したい場合などに実行してください。

### コンテスト参加中にリモートブランチへのpushを防ぐ

```
$ cd .git/hooks
$ ln -s ../../bin/git-hooks/pre-push
```