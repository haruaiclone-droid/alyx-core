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

Almost all developers write this layer.

- **Widgets**

- **IR**

- **RP / EP**

- **Executor**

- **Runtime**

## Design Principles

  1. **Layer-independent**: Alyx separates syntax (how UI is written), widgets, UI IR, compiler, renderers, event handlers, and runtime. Each layer can be replaced, extended, or developed independently.
  2. **Transform-based pipeline**: Alyx turns UI into lower-level representations step by step: syntax → widgets → UI IR → RP/EP → renderer and event handler.
  3. **Runtime-oriented design**: Alyx Core is designed to be used by runtimes that manage state, events, updates, compilation, rendering, and platform integration.
