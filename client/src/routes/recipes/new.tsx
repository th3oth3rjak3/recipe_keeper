import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/recipes/new")({
	component: RouteComponent,
});

// TODO: work on adding the new recipe page and components.

function RouteComponent() {
	return <div>Hello "/recipes/new"!</div>;
}
