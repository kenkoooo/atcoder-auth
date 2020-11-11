# AtCoder Authentication API [beta]

AtCoder ユーザーの本人確認を行うための API です。

|API|役割|
|:----|:----|
|/api/authorize|ユーザーの本人確認をするための認証用コードを発行します。|
|/api/confirm|認証用コードを使って本人確認を行います。本人確認に成功すると `token` が手に入ります。|
|/api/verify|`token` を使ってログインします。|

# POST /api/authorize

本人確認を行いたいユーザーの `user_id` を送ります。
認証用コード `verification_code` と `secret` が返ってきます。

## Request
```json
{
  "user_id": "(user_id to authorize)"
}
```

## Response
```json
{
  "verification_code": "(temporary verification code)"
}
```
## Example

```sh
curl -X POST https://atcoder-auth.kenkoooo.com/api/authorize \
    -H 'Content-Type:application/json' \
    -d '{"user_id":"kenkoooo"}' 

# {"verification_code":"pGqFZ4GPbACxWGCsWbBapcyT0mYg4Z"}
```

# POST /api/confirm

`/api/authorize` で手に入れた認証用コード `verification_code` を AtCoder の "所属" 欄に設定したら、 `user_id` と先ほど手に入れた `secret` を送ります。
所属欄に正しく設定されていて本人確認できれば、`token` が返ってきます。
`token` が返ってきたら所属欄をもとに戻しても構いません。
以降は `/api/verify` で `token` を使って本人確認できます。

## Request
```json
{
  "user_id": "(user_id to confirm)",
  "secret": "(secret)"
}
```

## Response
```json
{
  "token": "(token string)"
}
```
## Example
```sh
curl -X POST https://atcoder-auth.kenkoooo.com/api/confirm \
    -H 'Content-Type:application/json' \
    -d '{"user_id":"kenkoooo"}' 

# {"token":"IzMNAMm5tNd1Kv90GR1orIcJQd93bx"}
```

# POST /api/verify

`/api/confirm` で手に入れた `token` を使って本人確認を行います。
`user_id` と `token` の組が正しければステータスコード200が返ってきます。

## Request
```json
{
  "user_id": "(user_id to confirm)",
  "token": "(token string of the user)"
}
```

## Response
```json
"Ok"
```

## Example
```sh
curl -X POST https://atcoder-auth.kenkoooo.com/api/verify \
    -H 'Content-Type:application/json' \
    -d '{"user_id":"kenkoooo", "token":"IzMNAMm5tNd1Kv90GR1orIcJQd93bx"}' 

# "Ok"
```
