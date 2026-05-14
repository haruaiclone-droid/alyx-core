# Alyx Overview

This document describes the overall vision of Alyx project.

## Features

  - **Layer-independent**: Alyx separates syntax (how UI is written), widgets, UI IR, compiler, renderers, event handlers, and runtime. Each layer can be replaced, extended, or developed independently.
  - **Transform-based pipeline**: Alyx turns UI into lower-level representations step by step: syntax → widgets → UI IR → RP/EP → renderer and event handler.
  - **Runtime-oriented design**: Alyx Core is designed to be used by runtimes that manage state, events, updates, compilation, rendering, and platform integration.
