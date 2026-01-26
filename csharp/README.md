# Pintui.NET

C# / .NET implementation of the pintui design system.

**Status:** Planned

## Planned API

```csharp
using Pintui;

// Messages
Messages.Info("Starting deployment...");
Messages.Success("Done!");
Messages.Warn("Config not found, using defaults");
Messages.Error("Connection failed");

// Progress
using var spinner = Progress.Spinner("Connecting to server");
// ... do work ...
spinner.Success("Connected");

// Layout
Layout.Header("Configuration");
Layout.Kv("Environment", "production");
Layout.Kv("Region", "us-east-1");
```

### With async/await

```csharp
using Pintui;

Messages.Info("Starting deployment...");

await Progress.WithSpinnerAsync("Connecting to server", async () =>
{
    await ConnectAsync();
});

Messages.Success("Deployment complete!");
```

### Static using

```csharp
using static Pintui.Messages;
using static Pintui.Layout;
using static Pintui.Progress;

Info("Starting deployment...");
Header("Configuration");
Kv("Environment", "production");
Success("Done!");
```

## Planned Distribution

### NuGet

```bash
dotnet add package Pintui
```

```xml
<PackageReference Include="Pintui" Version="0.1.0" />
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
