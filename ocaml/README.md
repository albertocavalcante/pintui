# pintui-ocaml

OCaml implementation of the pintui design system.

**Status:** Planned

## Planned API

```ocaml
open Pintui

let () =
  Messages.info "Starting deployment...";

  let spinner = Progress.spinner "Connecting to server" in
  (* ... do work ... *)
  Progress.finish_success spinner "Connected";

  Layout.header "Configuration";
  Layout.kv "Environment" "production";
  Layout.kv "Region" "us-east-1";

  Messages.success "Deployment complete!"
```

### With local open

```ocaml
let () =
  let open Pintui in
  info "Starting deployment...";
  header "Configuration";
  kv "Environment" "production";
  success "Deployment complete!"
```

### With Fun.protect for cleanup

```ocaml
open Pintui

let () =
  Messages.info "Starting deployment...";

  Progress.with_spinner "Connecting to server" (fun () ->
    connect_to_server ()
  );

  Messages.success "Deployment complete!"
```

### Formatting

```ocaml
open Pintui.Format

let () =
  print_endline (human_size 1048576);     (* "1.0 MB" *)
  print_endline (human_duration 5500);    (* "5.5s" *)
  print_endline (pluralize 3 "file")      (* "3 files" *)
```

## Planned Distribution

### opam

```bash
opam install pintui
```

### dune

```lisp
(library
 (name myapp)
 (libraries pintui))
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
