# @pintui/core

TypeScript implementation of the pintui design system.

**Status:** Planned

## Planned API

```typescript
import { messages, layout, progress } from '@pintui/core';

messages.info('Starting deployment...');

const spinner = progress.spinner('Connecting to server');
// ... do work ...
spinner.success('Connected');

layout.header('Configuration');
layout.kv('Environment', 'production');
layout.kv('Region', 'us-east-1');

messages.success('Deployment complete!');
```

## Contributing

See the main [README](../README.md) for contribution guidelines.
