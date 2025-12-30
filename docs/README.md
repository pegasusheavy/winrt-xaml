# WinRT-XAML Documentation Website

This is the official documentation website for the WinRT-XAML library, built with Angular.

## Features

- **Comprehensive Guide**: Complete overview of WinRT-XAML capabilities
- **Quick Start**: Get up and running in minutes
- **API Reference**: Detailed documentation for all controls
- **Examples Gallery**: Browse all example applications
- **Architecture Diagram**: Understand how WinRT-XAML works under the hood
- **Responsive Design**: Works on desktop and mobile

## Development

### Prerequisites

- Node.js (v18 or later)
- pnpm (recommended) or npm

### Install Dependencies

```bash
cd docs
pnpm install
```

### Run Development Server

```bash
pnpm start
```

Navigate to `http://localhost:4200/`. The application will automatically reload if you change any source files.

### Build for Production

```bash
pnpm run build
```

Build artifacts will be stored in the `dist/` directory.

### Deploy to GitHub Pages

```bash
pnpm run deploy
```

This builds the site and deploys it to the `gh-pages` branch.

## Structure

```
docs/
├── src/
│   ├── app/
│   │   ├── app.html       # Main template
│   │   ├── app.css        # Styles
│   │   └── app.ts         # Component logic
│   ├── index.html         # Entry HTML
│   └── styles.css         # Global styles
├── public/
│   └── favicon.ico        # Site icon
└── package.json           # Dependencies & scripts
```

## Contributing

To update the documentation:

1. Edit files in `src/app/`
2. Test locally with `pnpm start`
3. Build and verify with `pnpm run build`
4. Deploy with `pnpm run deploy`

## License

This documentation is part of the WinRT-XAML project and is licensed under MIT or Apache-2.0.
