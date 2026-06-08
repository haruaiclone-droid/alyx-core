# Events

EventPlan currently supports:

- `Click`, `Hover`
- Pointer (`Down`, `Up`, `Move`)
- Keyboard (`KeyDown`, `KeyUp`)
- Focus/Blur
- Scroll / Navigation

`ResolvedHitArea` is used for dispatching to handlers through `HandlerTable`.

Runtime dispatch helpers on `HeadlessRuntime`:

- `dispatch_pointer`, `dispatch_pointer_event`
- `dispatch_hover`, `dispatch_pointer_down`, `dispatch_pointer_up`, `dispatch_pointer_move`
- `dispatch_keydown`, `dispatch_keydown_with_key`, `dispatch_keyup`
- `dispatch_keydown_by_ids_with_key` for id-targeted key dispatch; Enter/Space fall back to click handlers when no explicit keydown handler exists.
- `dispatch_focus`, `dispatch_blur`, `dispatch_submit`
- Navigation event payloads (`navigate_back`, `navigate_forward`) are parsed by the web event bridge for future back/forward behavior.

`IrNode::HitArea` can be marked disabled.

- Disabled areas still render output nodes.
- Disabled areas do not emit runtime `ResolvedHitArea` handlers.
