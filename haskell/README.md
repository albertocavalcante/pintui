# pintui-haskell

Haskell implementation of the pintui design system.

**Status:** Planned

## Planned API

```haskell
import Pintui

main :: IO ()
main = do
    info "Starting deployment..."

    spinner <- newSpinner "Connecting to server"
    -- ... do work ...
    finishSuccess spinner "Connected"

    header "Configuration"
    kv "Environment" "production"
    kv "Region" "us-east-1"

    success "Deployment complete!"
```

### With bracket for cleanup

```haskell
import Pintui

main :: IO ()
main = do
    info "Starting deployment..."

    withSpinner "Connecting to server" $ \spinner -> do
        connectToServer
        finishSuccess spinner "Connected"

    success "Deployment complete!"
```

### Qualified imports

```haskell
import qualified Pintui.Messages as Msg
import qualified Pintui.Layout as Layout
import qualified Pintui.Progress as Progress

main :: IO ()
main = do
    Msg.info "Starting deployment..."
    Layout.header "Configuration"
    Layout.kv "Environment" "production"
    Msg.success "Done!"
```

### Formatting

```haskell
import Pintui.Format

main :: IO ()
main = do
    putStrLn $ humanSize 1048576      -- "1.0 MB"
    putStrLn $ humanDuration 5500     -- "5.5s"
    putStrLn $ pluralize 3 "file"     -- "3 files"
```

## Planned Distribution

### Hackage

```bash
cabal install pintui
```

### package.yaml (hpack)

```yaml
dependencies:
  - pintui >= 0.1
```

### cabal

```cabal
build-depends:
    pintui >= 0.1 && < 0.2
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
