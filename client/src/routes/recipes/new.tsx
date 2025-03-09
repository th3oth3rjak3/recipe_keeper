import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/recipes/new")({
	component: RouteComponent,
});

function RouteComponent() {
	return <div>Hello "/recipes/new"!</div>;
}
