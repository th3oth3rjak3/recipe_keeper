import {
	Accordion,
	AccordionControl,
	AccordionItem,
	AccordionPanel,
	ActionIcon,
	Button,
	Card,
	CardSection,
	Divider,
	Group,
	Stack,
	Text,
} from "@mantine/core";
import { modals } from "@mantine/modals";
import { IconCheck, IconEdit, IconTrash, IconX } from "@tabler/icons-react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { useNavigate } from "@tanstack/react-router";
import ky from "ky";
import { useMemo, useState } from "react";
import type { Recipe, UpdateRecipe } from "../models/recipes";
import { RecipeEditor } from "./recipe-editor";

export interface RecipeDetailsProps {
	recipe: Recipe;
	onRecipeEdited: () => void;
}

export function RecipeDetails(props: RecipeDetailsProps) {
	const [finishedIngredients, setFinishedIngredients] = useState<number[]>([]);
	const [finishedInstructions, setFinishedInstructions] = useState<number[]>(
		[],
	);
	const [openSections, setOpenSections] = useState<string[]>([]);
	const [editModalOpen, setEditModalOpen] = useState(false);

	const queryClient = useQueryClient();
	const navigate = useNavigate();

	const { mutate: deleteRecipe } = useMutation(
		{
			mutationKey: ["delete-recipe"],
			mutationFn: async (id: number) => {
				await ky.delete(`http://localhost:8000/api/recipes/${id}`);
			},
			onSuccess: () => {
				navigate({ to: "/" });
			},
		},
		queryClient,
	);

	const { mutate: updateRecipe } = useMutation(
		{
			mutationKey: ["update-recipe"],
			mutationFn: async (recipe: UpdateRecipe) => {
				await ky.put(`http://localhost:8000/api/recipes/${recipe.id}`, {
					json: recipe,
				});
			},
			onSuccess: () => props.onRecipeEdited(),
		},
		queryClient,
	);

	const handleDeleteRequest = (recipe: Recipe) => {
		modals.openConfirmModal({
			title: "Are you sure?",
			children: (
				<Text size="md">{`Are you sure you want to delete '${recipe.name}'?`}</Text>
			),
			labels: { confirm: "Delete", cancel: "Cancel" },
			onCancel: () => {},
			onConfirm: () => deleteRecipe(recipe.id),
		});
	};

	const handleEditRequest = (recipe: UpdateRecipe) => {
		setEditModalOpen(false);

		recipe.ingredients = recipe.ingredients.map((ingredient) => ({
			...ingredient,
			id: ingredient.id && ingredient.id > 0 ? ingredient.id : null,
		}));

		recipe.instructions = recipe.instructions.map((instruction) => ({
			...instruction,
			id: instruction.id && instruction.id > 0 ? instruction.id : null,
		}));

		updateRecipe(recipe);
	};

	const ingredients = useMemo(() => {
		const toggleIngredient = (id: number) => {
			if (finishedIngredients.includes(id)) {
				setFinishedIngredients(
					finishedIngredients.filter((existing) => existing !== id),
				);
			} else {
				setFinishedIngredients([...finishedIngredients, id]);
			}
		};

		if (props.recipe.ingredients.length === 0) {
			return (
				<AccordionItem value={"ingredients"}>
					<AccordionControl>
						<Text>{"Ingredients"}</Text>
					</AccordionControl>
					<AccordionPanel>
						<Stack>
							<Divider />
							<Text>{"No Ingredients"}</Text>
						</Stack>
					</AccordionPanel>
				</AccordionItem>
			);
		}

		return (
			<AccordionItem value={"ingredients"}>
				<AccordionControl>
					<Text>{"Ingredients"}</Text>
				</AccordionControl>
				<AccordionPanel>
					<Stack>
						<Divider />
						{props.recipe.ingredients.map((ingredient) => (
							<Group key={ingredient.id}>
								<ActionIcon onClick={() => toggleIngredient(ingredient.id)}>
									{finishedIngredients.includes(ingredient.id) ? (
										<IconX />
									) : (
										<IconCheck />
									)}
								</ActionIcon>
								<Text
									style={
										finishedIngredients.includes(ingredient.id)
											? { textDecoration: "line-through" }
											: {}
									}
									key={ingredient.id}
								>
									{ingredient.description}
								</Text>
							</Group>
						))}
					</Stack>
				</AccordionPanel>
			</AccordionItem>
		);
	}, [props.recipe, finishedIngredients]);

	const instructions = useMemo(() => {
		const toggleInstruction = (id: number) => {
			if (finishedInstructions.includes(id)) {
				setFinishedInstructions(
					finishedInstructions.filter((existing) => existing !== id),
				);
			} else {
				setFinishedInstructions([...finishedInstructions, id]);
			}
		};

		if (props.recipe.instructions.length === 0) {
			return (
				<AccordionItem value={"instructions"}>
					<AccordionControl>
						<Text>{"Instructions"}</Text>
					</AccordionControl>
					<AccordionPanel>
						<Stack>
							<Divider />
							<Text>{"No Instructions"}</Text>
						</Stack>
					</AccordionPanel>
				</AccordionItem>
			);
		}

		return (
			<AccordionItem value={"instructions"}>
				<AccordionControl>
					<Text>{"Instructions"}</Text>
				</AccordionControl>
				<AccordionPanel>
					<Stack>
						<Divider />
						{props.recipe.instructions.map((instruction) => (
							<Group key={instruction.id}>
								<ActionIcon onClick={() => toggleInstruction(instruction.id)}>
									{finishedInstructions.includes(instruction.id) ? (
										<IconX />
									) : (
										<IconCheck />
									)}
								</ActionIcon>
								<Text
									style={
										finishedInstructions.includes(instruction.id)
											? { textDecoration: "line-through" }
											: {}
									}
									key={instruction.id}
								>
									{instruction.description}
								</Text>
							</Group>
						))}
					</Stack>
				</AccordionPanel>
			</AccordionItem>
		);
	}, [props.recipe, finishedInstructions]);

	const header = useMemo(() => {
		return (
			<AccordionItem value="header">
				<AccordionControl>
					<Text>{"About"}</Text>
				</AccordionControl>
				<AccordionPanel>
					<Stack>
						<Divider />
						{props.recipe.author && (
							<Text>{`Author: ${props.recipe.author}`}</Text>
						)}
						{props.recipe.difficulty && (
							<Text>{`Difficulty: ${props.recipe.difficulty}`}</Text>
						)}
						{props.recipe.estimatedDuration && (
							<Text>{`Time: ${props.recipe.estimatedDuration}`}</Text>
						)}
						{props.recipe.description && (
							<Text>{props.recipe.description}</Text>
						)}
					</Stack>
				</AccordionPanel>
			</AccordionItem>
		);
	}, [props.recipe]);

	return (
		<>
			<Card maw={800} w="75%" miw={300} withBorder>
				<CardSection>
					<Stack p="md">
						<Text fz="h2" lh="md" ta="center">
							{props.recipe.name}
						</Text>
						<Divider />
						<Accordion
							variant="separated"
							radius="md"
							multiple={true}
							value={openSections}
							onChange={setOpenSections}
						>
							{header}
							{ingredients}
							{instructions}
						</Accordion>
						<Divider />
						<Group justify="center">
							<Button
								leftSection={<IconEdit />}
								onClick={() => setEditModalOpen(true)}
							>
								Edit
							</Button>
							<Button
								leftSection={<IconTrash />}
								onClick={() => handleDeleteRequest(props.recipe)}
							>
								Delete
							</Button>
						</Group>
					</Stack>
				</CardSection>
			</Card>
			<RecipeEditor
				recipe={props.recipe}
				open={editModalOpen}
				onSave={handleEditRequest}
				closeModal={() => setEditModalOpen(false)}
			/>
		</>
	);
}
