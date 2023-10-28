# Objective and links

- Objective
- Asana Task URL:

<!--
この変更が必要になった背景や変更の目的を書いてください。
Asana のリンクや、もしあれば Google docs　や Slack threads など関連するリンクを貼ってください。
-->

## What was changed

<!-- 具体的な変更内容について詳しく説明してください。スクリーンショットがある場合は貼ってください。 -->

## What was not changed

<!-- なんらかの理由で今回の変更のスコープから外すものがある場合は理由と対象を説明してください -->

## Deploy dependencies

<!--
このPRに依存している、もしくは依存されているプルリクエストを列挙してください。

CLI、Front、およびAPIサーバーは非同期にデプロイされることに注意してください。
常にサービスが正しい動作をすることを保証するため、どのプルリクエストが先にマージされなければいけないか、
順番を気にする必要がある場合があります。

1. 下位互換性のある方法でAPIを変更した場合、依存関係をあまり気にする必要はありません。
   しかし、例えばAPIがデプロイされる前に、ユーザーが更新されたページを開いてしまう可能性があります。
   これが問題になる可能性がある場合は、PRを分割して、先にAPIの変更をマージしてください。
2. APIと他のコンポーネントに後方互換性のない破壊的変更を加えたい場合は、その順序をよく考えてください。
   例えば、ウェブとCLIを新旧のAPIで動作するように変更し、APIの変更点をマージし、クリーンアップとして
   ウェブとCLIから後方互換性のあるコードを削除するという方法があります。
-->

## QA

<!--
各環境で行う動作確認項目をチェックボックスで列挙してください。
動作確認が不要な場合はその理由を書いてください。
- [ ] QA項目
-->

### local

### staging

## Review deadline

<!-- どれくらい急ぐのかを書いてください、特に何もないときは2,3日後を推奨します -->

## Others

<!-- 実装上の懸念点や注意点などあれば記載してください -->