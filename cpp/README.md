# pintui-cpp

C++ implementation of the pintui design system.

**Status:** Planned

## Planned API

```cpp
#include <pintui/pintui.hpp>

using namespace pintui;

int main() {
    messages::info("Starting deployment...");

    auto spinner = progress::spinner("Connecting to server");
    // ... do work ...
    spinner.success("Connected");

    layout::header("Configuration");
    layout::kv("Environment", "production");
    layout::kv("Region", "us-east-1");

    messages::success("Deployment complete!");

    return 0;
}
```

### With RAII Spinner

```cpp
#include <pintui/pintui.hpp>

using namespace pintui;

int main() {
    messages::info("Starting deployment...");

    {
        auto spinner = progress::scoped_spinner("Connecting to server");
        // ... do work ...
        // automatically finishes on scope exit
    }

    messages::success("Deployment complete!");
    return 0;
}
```

## Planned Distribution

- CMake FetchContent
- vcpkg
- Conan

```cmake
include(FetchContent)
FetchContent_Declare(
    pintui
    GIT_REPOSITORY https://github.com/albertocavalcante/pintui.git
    GIT_TAG main
    SOURCE_SUBDIR cpp
)
FetchContent_MakeAvailable(pintui)

target_link_libraries(myapp PRIVATE pintui::pintui)
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
