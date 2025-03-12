import {
	Button,
	Card,
	CardSection,
	Divider,
	SimpleGrid,
	Stack,
	Text,
} from "@mantine/core";
import { Link } from "@tanstack/react-router";
import { useMemo } from "react";
import type { RecipeBase } from "../models/recipes";

export interface RecipeCardProps {
	recipe: RecipeBase;
}

export function RecipeCard(props: RecipeCardProps) {
	const description = useMemo(
		() => props.recipe.description && <Text>{props.recipe.description}</Text>,
		[props.recipe],
	);

	const author = useMemo(
		() =>
			props.recipe.author && <Text>{`Author: ${props.recipe.author}`}</Text>,
		[props.recipe],
	);

	const difficulty = useMemo(
		() =>
			props.recipe.difficulty && (
				<Text>{`Difficulty: ${props.recipe.difficulty}`}</Text>
			),
		[props.recipe],
	);

	const estimatedDuration = useMemo(
		() =>
			props.recipe.estimatedDuration && (
				<Text>{`Time: ${props.recipe.estimatedDuration}`}</Text>
			),
		[props.recipe],
	);

	return (
		<Card maw={500} w="75%" miw={400} withBorder>
			<CardSection>
				<Stack p="md">
					<Button
						component={Link}
						variant="filled"
						size="lg"
						to={`/recipes/${props.recipe.id}`}
					>
						{props.recipe.name}
					</Button>
					<Divider />
					<SimpleGrid cols={2} spacing="xs">
						{author}
						{estimatedDuration}
						{difficulty}
					</SimpleGrid>
					{description}
				</Stack>
			</CardSection>
		</Card>
	);
}
