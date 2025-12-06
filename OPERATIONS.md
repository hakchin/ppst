# OPERATIONS.md

PPST Academy 웹사이트 실행 및 운영 가이드입니다.

## 사전 요구사항

### 필수 설치

```bash
# Rust 툴체인 (stable)
rustup default stable

# cargo-leptos 빌드 도구
cargo install cargo-leptos

# Tailwind CSS CLI (standalone binary)
# macOS
brew install tailwindcss

# 또는 직접 다운로드
# https://github.com/tailwindlabs/tailwindcss/releases
```

### WASM 타겟 추가

```bash
rustup target add wasm32-unknown-unknown
```

## 개발 환경 실행

### Cargo Leptos 사용 (권장)

개발 시에는 `cargo-leptos`를 사용하여 핫 리로드와 함께 실행합니다:

```bash
# 개발 서버 실행 (http://0.0.0.0:3000)
cargo leptos watch
```

이 명령어는 다음을 자동으로 처리합니다:
- Rust 서버 코드 컴파일 (SSR feature)
- WASM 클라이언트 번들 빌드 (hydrate feature)
- Tailwind CSS 컴파일
- 파일 변경 감지 및 자동 리로드

### 포트 변경

```bash
# 환경변수로 포트 변경
LEPTOS_SITE_ADDR=127.0.0.1:8080 cargo leptos watch

# 또는 Cargo.toml의 site-addr 수정
```

## 프로덕션 빌드

### 빌드 생성

```bash
# 릴리스 빌드 (최적화됨)
cargo leptos build --release
```

빌드 결과물:
- `target/release/ppst-academy` - 서버 바이너리
- `target/site/` - 정적 파일 (WASM, CSS, 에셋)

### 바이너리 직접 실행

빌드된 바이너리를 직접 실행할 수 있습니다:

```bash
# 외부 접속 허용 (0.0.0.0)
LEPTOS_SITE_ADDR=0.0.0.0:3000 \
LEPTOS_SITE_ROOT=target/site \
./target/release/ppst-academy

# 로컬 전용 (기본값)
LEPTOS_SITE_ROOT=target/site \
./target/release/ppst-academy
```

> **참고**: `Cargo.toml`의 `site-addr` 설정은 개발 환경(`cargo leptos watch`)에만 적용됩니다.
> 배포된 바이너리는 `Cargo.toml`을 읽지 않으므로 **환경변수로 설정해야 합니다**.

### 배포용 파일 구조

프로덕션 배포 시 필요한 파일:

```
배포_디렉토리/
├── ppst-academy          # 서버 바이너리
└── site/                 # 정적 파일 디렉토리
    ├── pkg/              # WASM 번들
    │   ├── ppst-academy.js
    │   └── ppst-academy_bg.wasm
    ├── output.css        # 컴파일된 CSS
    └── (public 폴더 내용)
```

## 환경변수

| 변수명 | 기본값 | 설명 |
|--------|--------|------|
| `LEPTOS_SITE_ADDR` | `0.0.0.0:3000` | 서버 바인딩 주소 |
| `LEPTOS_SITE_ROOT` | `target/site` | 정적 파일 루트 경로 |
| `LEPTOS_RELOAD_PORT` | `3001` | 핫 리로드 WebSocket 포트 |
| `RUST_LOG` | - | 로깅 레벨 (예: `info`, `debug`) |

## 서비스 운영

### Systemd 서비스 등록 (Linux)

`/etc/systemd/system/ppst-academy.service`:

```ini
[Unit]
Description=PPST Academy Website
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/ppst-academy
ExecStart=/opt/ppst-academy/ppst-academy
Environment=LEPTOS_SITE_ADDR=0.0.0.0:3000
Environment=LEPTOS_SITE_ROOT=/opt/ppst-academy/site
Environment=RUST_LOG=info
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

> **필수**: `LEPTOS_SITE_ADDR`와 `LEPTOS_SITE_ROOT` 환경변수는 반드시 설정해야 합니다.

서비스 관리:

```bash
# 서비스 등록 및 시작
sudo systemctl daemon-reload
sudo systemctl enable ppst-academy
sudo systemctl start ppst-academy

# 상태 확인
sudo systemctl status ppst-academy

# 로그 확인
sudo journalctl -u ppst-academy -f
```

### Docker 실행 (선택사항)

```dockerfile
FROM rust:1.84-slim AS builder
WORKDIR /app
COPY . .
RUN cargo install cargo-leptos
RUN rustup target add wasm32-unknown-unknown
RUN cargo leptos build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/ppst-academy .
COPY --from=builder /app/target/site ./site
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_SITE_ROOT=/app/site
EXPOSE 3000
CMD ["./ppst-academy"]
```

```bash
# 빌드 및 실행
docker build -t ppst-academy .
docker run -p 3000:3000 ppst-academy
```

## 상태 확인

### 헬스 체크

```bash
# 서버 응답 확인
curl -I http://127.0.0.1:3000

# 또는
curl http://127.0.0.1:3000/
```

### 프로세스 확인

```bash
# 실행 중인 프로세스 확인
pgrep -a ppst-academy

# 포트 사용 확인
lsof -i :3000
```

## 문제 해결

### 포트 충돌

`cargo leptos watch`는 두 개의 포트를 사용합니다:
- **3000**: 메인 웹 서버
- **3001**: Hot reload용 WebSocket 서버

이전 프로세스가 비정상 종료되면 포트가 점유된 채로 남아 새 프로세스 시작 시 아래와 같은 오류가 발생합니다:

```
Reload TCP port 0.0.0.0:3001 already in use
```

**해결 방법**:

```bash
# 한 줄로 프로세스 종료 (권장)
lsof -t -i :3000 -i :3001 | xargs kill -9 2>/dev/null

# 포트가 비었는지 확인 후 재시작
lsof -i :3000 -i :3001 || cargo leptos watch
```

또는 단계별로:

```bash
# 1. 포트 사용 중인 프로세스 확인
lsof -i :3000 -i :3001

# 2. PID 확인 후 종료
kill -9 <PID>

# 3. 서버 재시작
cargo leptos watch
```

> **팁**: `lsof -t`는 PID만 출력하므로 파이프와 함께 사용하기 좋습니다.

**예방**: 개발 서버를 종료할 때는 `Ctrl+C`로 정상 종료하세요.

### WASM 빌드 실패

```bash
# WASM 타겟 재설치
rustup target remove wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown

# cargo-leptos 재설치
cargo install cargo-leptos --force
```

### Tailwind CSS 미적용

```bash
# 수동으로 CSS 빌드
tailwindcss -i input.css -o style/output.css

# 감시 모드로 실행
tailwindcss -i input.css -o style/output.css --watch
```

### macOS Release 빌드 실패 (linker 오류)

macOS에서 release 빌드 시 `ld: Assertion failed: (name.size() <= maxLength)` 오류가 발생할 수 있습니다.
이는 Leptos의 복잡한 generic 타입으로 인해 심볼 이름이 linker 제한을 초과하기 때문입니다.

**해결책**: `.cargo/config.toml`에 심볼 맹글링 설정 추가 (이미 적용됨)

```toml
[build]
rustflags = ["-Csymbol-mangling-version=v0"]
```

이 설정이 없으면 release 빌드가 실패할 수 있습니다.

## 외부 접속 허용

현재 `Cargo.toml`에서 `site-addr = "0.0.0.0:3000"`으로 설정되어 있어 외부에서도 접근 가능합니다.

로컬 전용으로 변경하려면:

```bash
# 로컬만 허용
LEPTOS_SITE_ADDR=127.0.0.1:3000 ./target/release/ppst-academy
```

또는 `Cargo.toml`의 `site-addr`를 수정:

```toml
[package.metadata.leptos]
site-addr = "127.0.0.1:3000"
```

> **보안 주의**: `0.0.0.0`으로 바인딩하면 네트워크의 모든 호스트에서 접근 가능합니다.
> 프로덕션에서는 리버스 프록시(nginx, Caddy 등)를 앞에 두는 것을 권장합니다.
