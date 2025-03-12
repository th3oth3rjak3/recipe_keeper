import { Button, Loader, Stack } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import {
	IconAlertSquareRoundedFilled,
	IconArrowLeft,
} from "@tabler/icons-react";
import { useQuery } from "@tanstack/react-query";
import { Link, createFileRoute } from "@tanstack/react-router";
import ky from "ky";
import { RecipeDetails } from "../../components/recipe-details";
import type { Recipe } from "../../models/recipes";

async function fetchRecipeById(recipeId: string): Promise<Recipe> {
	return await ky.get(`/api/recipes/${recipeId}`).json<Recipe>();
}

export const Route = createFileRoute("/recipes/$recipeId")({
	component: RecipeDetailsComponent,
});

function RecipeDetailsComponent() {
	const params = Route.useParams();

	const {
		data: recipe,
		error,
		isLoading,
		isError,
		refetch,
	} = useQuery({
		queryKey: ["get-recipe-by-id", params.recipeId],
		queryFn: () => fetchRecipeById(params.recipeId),
	});

	if (isLoading) {
		return (
			//
			<Stack w="100%" mih="50vh" align="center" justify="center">
				<Loader size="xl" />
			</Stack>
		);
	}

	if (isError || recipe === undefined) {
		const message = isError ? error.message : "Unknown error occurred.";
		notifications.show({
			title: "Error!",
			message: message,
			icon: <IconAlertSquareRoundedFilled />,
			withCloseButton: true,
			autoClose: false,
			color: "red",
		});
		return (
			<Stack align="center" w="100%">
				<Button component={Link} to="/" leftSection={<IconArrowLeft />}>
					Go Back Home
				</Button>
			</Stack>
		);
	}

	return (
		<Stack align="center" w="100%">
			<RecipeDetails recipe={recipe} onRecipeEdited={refetch} />
		</Stack>
	);
}
