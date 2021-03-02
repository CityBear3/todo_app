# todo_app

1.このリポジトリをcloneして，プロジェクトルートディレクトリに移動します．
```
git clone https://github.com/CityBear3/todo_app.git/ todo

cd todo
```

2.`mkcert`を使用して証明書を発行します．
あらかじめ，
```
brew install mkcert
```
でインストールしてください．
```
mkcert -install
```
を実行したのち，
```
mkcert localhost
```
を実行します．

3.Dockerfileをbuildします．
```
docker build -t [repository  name]
```
4.コンテナを起動します．
```
docker run -d -p [your server's port]:[container's port] [repository name]
```
