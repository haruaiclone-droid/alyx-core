# Alyx Overview

This document describes the overall vision of Alyx project.

## Motivation

Many GUI frameworks tightly couple multiple elements:

- how UI is written
- how widgets are structured
- how layout is calculated
- how rendering is performed
- how events are routed
- how the runtime is managed

This kind of architecture can be effective for building highly optimized applications. When the framework controls the entire stack, it can make strong assumptions and optimize across layers.

However, Alyx does not treat maximum performance as the only goal of UI architecture. For many applications, flexibility, maintainability, and developer experience can be more important than raw performance.

Alyx is based on the idea that these elements should be separated into independent layers. Each layer should be understandable on its own, developed independently, and replaceable without forcing the rest of the system to be rewritten.

## Layer Responsibilities

Alyx divides UI development into these layers:

- **Syntax**

    Almost all developers write this layer. This layer should be easy to write, read, and maintain.

    Syntax may be provided by Rust APIs, macros, builders, DSLs, or other user-facing interfaces. It should focus on developer experience rather than rendering or event handling details.

    Almost all syntaxes are convert into widgets.

- **Widgets**

    Widgets provides reuseable UI building blocks.

    A widget describes UI behavior and structure at a higher level than IR. Widgets may manage composition, parameters, styling, and message mapping, but they should not directly depend on a specific renderer or platform event system.

    Widgets are convert into IR.

- **IR**
    IR is the intermediate representation of UI.

    It represents the meaning and structure of UI as data. IR should be independent from syntax, widgets, renderers, event handlers, and runtimes.

    IR is converted into RP and RP by a compiler.

- **RP / EP**

    `RP` stands for `RenderingPlan`. `RP` stands for `EventPlan`.

    `RenderingPlan` describes what should be rendered.
    `EventPlan` describes how events should be interpreted and routed.

    These plans are lower-level than IR and are intended to be consumed by executors.

- **Executor**

    Executors can be divided into `RendererPlanExecutor` and `EventPlanExecutor`. Each executor process each plans.

    They should be executable in Windows, MacOS, Linux, iOS, Android, and Web.

- **Runtime**

    The runtime manages the whole application.

    It owns the application loop, state updates, message flow, recompilation timing, rendering timing, and platform integration.

    In Alyx, the runtime is different from an executor:

    ```text
    Runtime = manages the application
    Executor = processes a plan
    ```

## First-party Crates

The following crates are maintained as part of the Alyx project:

- [`alyx-core`](https://github.com/naoshinn/alyx-core): provides the core entry point and re-exports `alyx-compiler`, `alyx-ir`, and `alyx-plan`

The following first-party crates are planned:

- `alyx-wgpu-renderer`
- `alyx-winit-event-handler`
- `alyx-widgets`

## Design Principles

  1. **Layer-independent**: Alyx separates syntax (how UI is written), widgets, UI IR, compiler, renderers, event handlers, and runtime. Each layer can be replaced, extended, or developed independently.
  2. **Transform-based pipeline**: Alyx turns UI into lower-level representations step by step: syntax → widgets → UI IR → RP/EP → renderer and event handler.
  3. **Runtime-oriented design**: Alyx Core is designed to be used by runtimes that manage state, events, updates, compilation, rendering, and platform integration.
