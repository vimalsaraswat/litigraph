export default function HomePage() {
  return (
    <main className="min-h-screen flex items-center justify-center bg-secondary">
      <div className="w-full max-w-xl px-8">
        <div className="space-y-2">
          <p className="text-sm uppercase tracking-[0.3em] text-muted-foreground">
            Litigraph
          </p>

          <h1 className="text-4xl font-semibold tracking-tight">
            Visual intelligence,
            <br />
            built for complexity.
          </h1>

          <p className="max-w-md text-muted-foreground leading-relaxed">
            An offline-first workspace for building connected knowledge,
            investigations, and relationship graphs.
          </p>
        </div>
      </div>
    </main>
  );
}
