import type { RecipeBase } from "../models/recipes";

export interface RecipeCardProps {
	recipe: RecipeBase;
}

export function RecipeCard(props: RecipeCardProps) {
	return (
		// TODO: finish the recipe card component
		<li>{props.recipe.name}</li>
	);
}
