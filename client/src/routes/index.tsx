import { Group, Stack } from "@mantine/core";
import { notifications } from "@mantine/notifications";
import { IconAlertSquareRoundedFilled } from "@tabler/icons-react";
import { useQuery } from "@tanstack/react-query";
import { createFileRoute } from "@tanstack/react-router";
import ky from "ky";
import { useEffect, useState } from "react";
import { RecipeCard } from "../components/recipe-card";
import SearchRecipesComponent, {
	type SearchParameters,
} from "../components/search-recipe";
import type { RecipeBase } from "../models/recipes";

export const Route = createFileRoute("/")({
	component: SearchPage,
});

function SearchPage() {
	const [params, setParams] = useState<SearchParameters>({
		query: "",
		includeIngredients: false,
		includeInstructions: false,
	});

	const [enabled, setEnabled] = useState(false);

	const fetchRecipes = async () => {
		const recipes = await ky
			.get(
				`http://localhost:8000/api/recipes?query=${params.query}&include_ingredients=${params.includeIngredients}&include_instructions=${params.includeInstructions}`,
			)
			.json<RecipeBase[]>();

		return recipes;
	};

	const { data, error, isFetching, refetch } = useQuery({
		queryKey: ["recipes", params],
		queryFn: fetchRecipes,
		enabled: enabled, // Prevents auto-fetching
	});

	// 🔥 Ensure fetching happens AFTER params are updated
	useEffect(() => {
		if (params.query) {
			refetch();
		}
	}, [params, refetch]);

	useEffect(() => {
		if (error) {
			notifications.show({
				title: "Error!",
				message: error.message,
				color: "red",
				autoClose: false,
				withCloseButton: true,
				icon: <IconAlertSquareRoundedFilled />,
				withBorder: true,
			});
		}
	}, [error]);

	const handleSearch = (params: SearchParameters) => {
		setEnabled(true);
		setParams(params);
	};

	return (
		//
		<Group style={{ width: "100%" }}>
			<Stack align="center" style={{ width: "100%" }}>
				<SearchRecipesComponent onSearch={handleSearch} />
				{data?.map((recipe) => (
					<RecipeCard key={recipe.id} recipe={recipe} />
				))}
			</Stack>
		</Group>
	);
}
