import {
	ActionIcon,
	Button,
	Divider,
	Grid,
	Group,
	Modal,
	NumberInput,
	Select,
	Stack,
	Text,
	TextInput,
	Textarea,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { IconPlus, IconTrash } from "@tabler/icons-react";
import { valibotResolver } from "mantine-form-valibot-resolver";
import { useEffect } from "react";
import { parse } from "valibot";
import {
	type Recipe,
	RecipeSchema,
	type UpdateRecipe,
	UpdateRecipeSchema,
} from "../models/recipes";

export interface RecipeEditorProps {
	recipe: Recipe;
	onSave: (recipe: UpdateRecipe) => void;
	open: boolean;
	closeModal: () => void;
}

export function RecipeEditor({
	recipe,
	onSave,
	open,
	closeModal,
}: RecipeEditorProps) {
	const form = useForm({
		validate: valibotResolver(RecipeSchema), // Use Valibot for validation
		initialValues: {
			id: recipe.id,
			name: recipe.name,
			author: recipe.author,
			description: recipe.description,
			difficulty: recipe.difficulty,
			estimatedDuration: recipe.estimatedDuration,
			ingredients: recipe.ingredients,
			instructions: recipe.instructions,
		},
	});

	useEffect(() => {
		form.setInitialValues({ ...recipe });
	}, [form, recipe]);

	const handleSubmit = (values: UpdateRecipe) => {
		try {
			parse(UpdateRecipeSchema, values); // Ensure values are valid
			onSave(values);
		} catch (error) {
			console.error("Validation error:", error);
			form.reset();
		}
	};

	const handleClose = () => {
		closeModal();
		form.reset();
	};

	return (
		<>
			<Modal
				opened={open}
				onClose={handleClose}
				title="Edit Recipe"
				closeOnClickOutside={false}
				closeOnEscape={false}
				size="xl"
			>
				<form onSubmit={form.onSubmit(handleSubmit)}>
					<Stack>
						<TextInput
							label="Recipe Name"
							key={form.key("name")}
							{...form.getInputProps("name")}
							withAsterisk
						/>
						<TextInput
							label="Author"
							key={form.key("author")}
							{...form.getInputProps("author")}
						/>
						<Textarea
							label="Description"
							key={form.key("description")}
							{...form.getInputProps("description")}
						/>
						<Select
							label="Difficulty"
							data={[
								{ value: "Easy", label: "Easy" },
								{ value: "Medium", label: "Medium" },
								{ value: "Hard", label: "Hard" },
							]}
							value={form.values.difficulty}
							onChange={(value) =>
								form.setFieldValue("difficulty", value || "Easy")
							}
							error={form.errors.difficulty}
						/>
						<TextInput
							label="Estimated Time"
							key={form.key("estimatedDuration")}
							{...form.getInputProps("estimatedDuration")}
						/>

						<Divider mt={20} />
						<Group>
							<Text pb={5} pt={5} size="lg">
								{"Ingredients"}
							</Text>
							<ActionIcon
								size="xs"
								onClick={() =>
									form.insertListItem("ingredients", {
										id:
											Math.min(
												...form
													.getValues()
													.ingredients.flatMap((ingredient) => ingredient.id),
												0,
											) - 1,
										recipeId: recipe.id,
										position: form.getValues().ingredients.length + 1,
										description: "",
									})
								}
							>
								<IconPlus />
							</ActionIcon>
						</Group>
						{form.getValues().ingredients.map((ingredient, index) => (
							<Grid key={ingredient.id} align="end">
								<Grid.Col span={{ base: 12, md: 2 }}>
									<NumberInput
										min={1}
										value={ingredient.position}
										onChange={(e) => {
											form.replaceListItem("ingredients", index, {
												...ingredient,
												position: e,
											});
											form.setValues({
												...form.getValues(),
												ingredients: form
													.getValues()
													.ingredients.toSorted(
														(a, b) => a.position - b.position,
													),
											});
										}}
										label="Position"
									/>
								</Grid.Col>
								<Grid.Col span={{ base: 12, md: 9 }}>
									<TextInput
										value={ingredient.description}
										onChange={(e) =>
											form.replaceListItem("ingredients", index, {
												...ingredient,
												description: e.target.value,
											})
										}
										label="Description"
									/>
								</Grid.Col>
								<Grid.Col
									span={{ base: 12, md: 1 }}
									style={{ textAlign: "center" }}
								>
									<ActionIcon
										onClick={() => form.removeListItem("ingredients", index)}
									>
										<IconTrash />
									</ActionIcon>
								</Grid.Col>
							</Grid>
						))}

						<Divider mt={20} />
						<Group>
							<Text pb={5} pt={5} size="lg">
								{"Instructions"}
							</Text>
							<ActionIcon
								size="xs"
								onClick={() =>
									form.insertListItem("instructions", {
										id:
											Math.min(
												...form
													.getValues()
													.instructions.flatMap(
														(instruction) => instruction.id,
													),
												0,
											) - 1,
										recipeId: recipe.id,
										position: form.getValues().instructions.length + 1,
										description: "",
									})
								}
							>
								<IconPlus />
							</ActionIcon>
						</Group>

						{form.getValues().instructions.map((instruction, index) => (
							<Grid key={instruction.id} align="end">
								<Grid.Col span={{ base: 12, md: 2 }}>
									<NumberInput
										min={1}
										value={instruction.position}
										onChange={(e) => {
											form.replaceListItem("instructions", index, {
												...instruction,
												position: e,
											});
											form.setValues({
												...form.getValues(),
												instructions: form
													.getValues()
													.instructions.toSorted(
														(a, b) => a.position - b.position,
													),
											});
										}}
										label="Position"
									/>
								</Grid.Col>
								<Grid.Col span={{ base: 12, md: 9 }}>
									<TextInput
										value={instruction.description}
										onChange={(e) =>
											form.replaceListItem("instructions", index, {
												...instruction,
												description: e.target.value,
											})
										}
										label="Description"
									/>
								</Grid.Col>
								<Grid.Col
									span={{ base: 12, md: 1 }}
									style={{ textAlign: "center" }}
								>
									<ActionIcon
										onClick={() => form.removeListItem("instructions", index)}
									>
										<IconTrash />
									</ActionIcon>
								</Grid.Col>
							</Grid>
						))}

						<Button type="submit" fullWidth mt="md">
							Save
						</Button>
					</Stack>
				</form>
			</Modal>
		</>
	);
}
