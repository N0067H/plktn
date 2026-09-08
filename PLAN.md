# plktn 계획서

plktn은 `plankton`을 줄인 이름의 Rust 기반 Docker CLI다.

Docker 데몬의 Unix domain socket에 연결해 컨테이너와 이미지를 조회하고, 여러 컨테이너를 동시에 제어하며, 로그를 실시간으로 스트리밍하는 작은 도구를 목표로 한다.

## 목적

이 프로젝트의 목적은 Rust로 Docker API 클라이언트를 만들면서 비동기 I/O, 동시성, 스트림 처리, CLI 설계를 익히는 것이다.

Docker CLI 전체를 다시 만드는 것이 아니라, 실제로 자주 쓰는 작은 기능을 직접 구현한다.

## 기술 선택

- **bollard:** 비동기 Docker API 클라이언트
- **tokio:** 비동기 런타임
- **clap v4:** CLI 서브커맨드와 옵션 파싱
- **futures-util:** stream 처리와 여러 future 조합
- **anyhow:** 초기 에러 처리
- **comfy-table 또는 tabled:** 터미널 테이블 출력

## MVP 범위

1. `plktn ps`
   - 실행 중인 컨테이너 목록 출력
   - ID, 이름, 이미지, 상태 표시

2. `plktn ps -a`
   - 중지된 컨테이너를 포함한 전체 컨테이너 목록 출력

3. `plktn images`
   - 로컬 이미지 목록 출력
   - repository, tag, image ID, size 표시

4. `plktn stop <container>...`
   - 여러 컨테이너를 동시에 정지
   - 각 컨테이너별 성공/실패 결과 출력

5. `plktn logs -f <container>`
   - 특정 컨테이너 로그를 실시간 스트리밍

## 구현 단계

### 1단계: CLI 뼈대

`clap`으로 아래 명령을 먼저 정의한다.

```text
plktn ps
plktn ps -a
plktn images
plktn stop <container>...
plktn logs -f <container>
```

`main.rs`는 CLI 파싱과 실행 진입점만 담당하게 둔다.

### 2단계: Docker 연결과 조회

`bollard`를 사용해 기본 Docker socket에 연결한다.

```text
unix:///var/run/docker.sock
```

먼저 `ps`, `ps -a`, `images`를 구현한다. 조회 결과는 테이블로 출력한다.

### 3단계: 병렬 stop

`plktn stop <container>...`에서 여러 컨테이너 중지 요청을 동시에 실행한다.

`join_all` 또는 `FuturesUnordered`를 사용하고, 일부 실패가 있어도 나머지 결과를 끝까지 출력한다.

```text
CONTAINER      RESULT
api-server     stopped
postgres       stopped
redis          failed: permission denied
```

### 4단계: 로그 스트리밍

`plktn logs -f <container>`를 구현한다.

Docker 로그 stream을 `StreamExt`로 소비하고, 들어오는 로그를 즉시 터미널에 출력한다.

## 추천 구조

```text
src/
  main.rs
  cli.rs
  docker.rs
  output.rs
```

- `cli`: 명령과 옵션 정의
- `docker`: Docker 연결과 API 호출
- `output`: 테이블과 로그 출력

## 수동 검증

```text
cargo run -- ps
cargo run -- ps -a
cargo run -- images
cargo run -- stop <container>
cargo run -- logs -f <container>
```

## 개발 시 주의사항

- Docker socket 권한 문제는 알아보기 쉬운 에러 메시지로 처리한다.
- Docker CLI 전체 호환성은 목표로 삼지 않는다.
- 처음부터 추상화를 많이 만들지 말고, 기능이 늘어날 때 모듈을 분리한다.
