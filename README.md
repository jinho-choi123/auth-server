Rust Auth server 개발환경 세팅하기 - using docker
=======
1. setup이라는 디렉토리를 만들고, 아래에 있는 파일들을 setup 디렉토리에 다운받는다.
2. gitlab(or github)에서 ssh을 통해 git clone할 것이기 때문에, ssh key를 만들어줄 것이다. setup 디렉토리에서 아래 명령을 실행하고, ssh key를 ./.ssh/id_ed25519 디렉토리에 저장해준다.
```
**save ssh key in ./.ssh/id_ed25519**
$ ssh-keygen -t ed25519
```
3. 만약, gitlab이 아닌 다른 git 서비스를 사용한다면 Dockerfile의 11번째 줄에서 gitlab.com 대신 다른 url을 적어준다.
4. Dockerfile의 12, 13번째줄에 email과 user name을 수정한다.
5. GitLab(or Github)으로 가서 Rust auth server repo를 만든다. 그리고 repo의 settings으로 가서, 2번에서 생성해준 ssh public key를 추가해준다. (https://docs.gitlab.com/ee/user/ssh.html)
6. **만약, git repo에 아무것도 없다면 에러가 발생할 것이다. 따라서 로컬에서 repo에 기본적인 프로젝트 세팅을 해주도록 하자. 
```
$ cargo new auth-server
$ cd auth-server
$ git init
$ git add .
$ git commit -m "Project initialization"
$ git remote add origin <git repo url>
$ git push origin main:dev
...
```
7. .env file에서는 mongoDB의 키, 인증서버에서의 hashing salt을 담고 있다. .env.example파일을 참고하여 .env file을 채우자. 이때 주의해야할 점은 MONGO_PASSWORD에 특수문자를 사용할 경우, MONGO_URL에서는 percent encoding을 해줘야 한다.
(https://www.mongodb.com/docs/manual/reference/connection-string/)
(https://www.url-encode-decode.com/)
8. 준비가 다 되었다면, Dockerfile을 통해 rustauth image을 만들어주자. 
```
$ docker build -t rustauth:latest -t rustauth:1.0
```
9. rustauth:latest이미지가 준비되었다면, docker-compose up -d를 통해 개발환경세팅을 마무리한다.