# Alyx 設計ドキュメント

## 概要

Alyx は Rust 製の GUI ライブラリです。UI IR を中心に据えることで、記法やレンダラーを変えても資産を使い回せる設計を目指します。

## アーキテクチャ全体像

```
ユーザーコード
  ↓
Alyx Widgets（状態管理含む、公式実装、交換可能）
  ↓
Alyx IR（IrNode<Msg>、純粋なデータ）
  ↓
Alyx Compiler（Layout計算、ArcTo展開、Msg → HandlerId 変換）
  ↓              ↓
Alyx RP        Alyx EP
（描画命令）    （イベント情報）
  ↓              ↓
レンダラー      Winitなどのイベントシステム
（wgpu は公式、他はコミュニティ）
```

---

## Alyx IR

UIの意味を定義する中間表現です。

### 設計方針

- **プリミティブに絞る**: Button などの抽象的なウィジェットは持たない
- **純粋なデータ**: クロージャを持たず、`Msg: Clone + Send` により Clone / Send が可能
- **意味の記述に集中**: レイアウト計算・座標解決は Alyx Compiler の責務
- **ジェネリック**: イベントはメッセージ型 `Msg` でジェネリック

### ノード定義

```rust
enum IrNode<Msg> {
    Container(Container<Msg>),
    Text(Text),
    Image(Image),
    Video(Video),
    HitArea(HitArea<Msg>),
}
```

#### Container

```rust
struct Container<Msg> {
    children: Vec<IrNode<Msg>>,
    layout: Layout,
}
```

#### Text

```rust
struct Text {
    content: String,
    style: TextStyle,
}
```

#### Image

```rust
struct Image {
    src: ImageSource,
    style: ImageStyle,
}
```

#### Video

```rust
struct Video {
    src: VideoSource,
    style: VideoStyle,
}
```

#### HitArea

```rust
struct HitArea<Msg> {
    shape: Shape,
    child: Box<IrNode<Msg>>,
    on_click: Option<Msg>,
    on_hover: Option<Msg>,
}
```

複数の子が必要な場合は `Container` で包んで `child` に渡す。

### 共通型

#### Layout

```rust
enum Layout {
    Flex(FlexLayout),
}

struct FlexLayout {
    direction: FlexDirection,
    gap: f32,
    align: Align,
    justify: Justify,
}
```

#### Shape

```rust
enum Shape {
    Rect(Rect),
    Circle(Circle),
    Path(Vec<PathCommand>),
}

struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    border_radius: Option<f32>,
}

struct Circle {
    cx: f32,
    cy: f32,
    radius: f32,
}
```

#### PathCommand

```rust
enum PathCommand {
    MoveTo(Point),
    LineTo(Point),
    QuadraticBezierTo {
        control: Point,
        to: Point,
    },
    CubicBezierTo {
        control1: Point,
        control2: Point,
        to: Point,
    },
    ArcTo {
        radius: Point,
        x_rotation: f32,
        large_arc: bool,
        sweep: bool,
        to: Point,
    },
    Close,
}

struct Point {
    x: f32,
    y: f32,
}
```

`ArcTo` は IR では表現力のために保持し、Compiler が RP/EP への変換時に `CubicBezierTo` の列に展開する。

#### Style

```rust
struct TextStyle {
    font: Font,
    size: f32,
    color: Color,
}

struct ImageStyle {
    width: f32,
    height: f32,
}

struct VideoStyle {
    width: f32,
    height: f32,
}
```

#### Source

```rust
enum ImageSource {
    Path(PathBuf),
    Bytes(Vec<u8>),
    Url(String),
}

enum VideoSource {
    Path(PathBuf),
    Bytes(Vec<u8>),
    Url(String),
}
```

#### Color

```rust
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}
```

### イベント設計

クロージャではなくメッセージ型を採用。

```rust
// IR側: Msg を保持するだけ
on_click: Option<Msg>,

// ユーザー側: enum でメッセージを定義
enum MyMsg {
    ButtonClicked,
    ImageHovered,
}
```

- IR は純粋なデータのまま保たれる
- `Msg: Clone + Send` により Clone / Send が可能
- 副作用の実体はアプリ側のランタイムが管理
- 状態管理は Alyx Widgets 層（コミュニティ）に委ねる

---

## Alyx Compiler

IR を受け取り、RP と EP を同時に出力します。

### 責務

- IR → RP / EP の変換
- Flexbox レイアウト計算・座標解決
- `ArcTo` → `CubicBezierTo` の列への展開
- `Msg` → `HandlerId` への変換

### 出力

```rust
struct CompilerOutput {
    rp: RenderingPlan,
    ep: EventPlan,
}
```

当面は毎回全変換。MVP 完成後に差分更新に対応予定。

---

## Alyx RP（Rendering Plan）

座標解決済みの描画命令のみを持つ純粋なデータです。レンダラーは RP を受け取り、描くだけに集中できます。

```rust
struct RenderingPlan {
    nodes: Vec<RpNode>,
}

enum RpNode {
    Text(RpText),
    Image(RpImage),
    Video(RpVideo),
}

struct RpText {
    x: f32,
    y: f32,
    content: String,
    style: TextStyle, // IR と共有
}

struct RpImage {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    src: ImageSource, // IR と共有
}

struct RpVideo {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    src: VideoSource, // IR と共有
}
```

---

## Alyx EP（Event Plan）

ヒットテスト情報のみを持つ純粋なデータです。Winit などのイベントシステムが受け取り、ヒットテストに集中できます。

```rust
struct EventPlan {
    hit_areas: Vec<ResolvedHitArea>,
}

struct ResolvedHitArea {
    shape: ResolvedShape, // ArcTo展開済み
    handler_id: HandlerId,
    event_type: EventType,
}

enum EventType {
    Click,
    Hover,
}

// ArcTo を CubicBezierTo に展開済みの Shape
enum ResolvedShape {
    Rect(Rect),
    Circle(Circle),
    Path(Vec<ResolvedPathCommand>),
}

enum ResolvedPathCommand {
    MoveTo(Point),
    LineTo(Point),
    QuadraticBezierTo {
        control: Point,
        to: Point,
    },
    CubicBezierTo {
        control1: Point,
        control2: Point,
        to: Point,
    },
    Close,
}
```

---

## バージョニング方針

- v0.x: 破壊的変更自由（設計の安定化フェーズ）
- v1.0 宣言をもって安定版
- v1.0 以降: メジャーバージョンアップでのみ破壊的変更、3年に1度を目安
