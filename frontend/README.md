# event-net frontend

This is the frontend application for the event-net project, built with SvelteKit.

## Setup

1. Copy `.env.example` to `.env` and configure environment variables:

```bash
cp .env.example .env
```

2. The following environment variables are available:

```
# API connection
PUBLIC_API_URL=http://localhost:8000  # URL of the backend API
```

3. Install dependencies:

```bash
npm install
```

## Development

Run the development server:

```bash
npm run dev
```

## Building

To create a production build:

```bash
npm run build
```

You can preview the production build with:

```bash
npm run preview
```

## Project Structure

```
frontend/
├── src/
│   ├── lib/          # Reusable components and utilities
│   ├── routes/       # SvelteKit routes
│   ├── styles/       # CSS styles
│   └── app.html      # Main HTML template
├── static/           # Static assets
├── .env              # Environment variables (not in git)
├── .env.example      # Example environment variables
└── package.json      # Project dependencies and scripts
```

## Technologies

- [SvelteKit](https://kit.svelte.dev/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Flowbite](https://flowbite.com/)

## Integration with Backend

This frontend connects to the Rust API backend which interacts with a Neo4j database. Make sure the backend is running before starting the frontend application.
