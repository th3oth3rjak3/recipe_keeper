import {
	ActionIcon,
	Button,
	Divider,
	Grid,
	Group,
	NumberInput,
	ScrollArea,
	Select,
	Text,
	TextInput,
	Textarea,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { IconPlus, IconTrash } from "@tabler/icons-react";
import { useMutation } from "@tanstack/react-query";
import { createFileRoute, useNavigate } from "@tanstack/react-router";
import ky from "ky";
import { valibotResolver } from "mantine-form-valibot-resolver";
import { v4 as uuidv4 } from "uuid";
import { parse } from "valibot";
import {
	type CreateRecipe,
	CreateRecipeSchema,
	RecipeSchema,
} from "../../models/recipes";

export const Route = createFileRoute("/recipes/new")({
	component: RouteComponent,
});

function RouteComponent() {
	const form = useForm<CreateRecipe>({
		validate: valibotResolver(CreateRecipeSchema),
		initialValues: {
			name: "",
			author: "",
			description: "",
			difficulty: "Easy",
			estimatedDuration: "",
			ingredients: [],
			instructions: [],
		},
	});

	const navigate = useNavigate();

	const { mutate } = useMutation({
		mutationKey: ["create-recipe"],
		mutationFn: async (recipe: CreateRecipe) => {
			return await ky
				.post("/api/recipes", {
					json: recipe,
				})
				.json();
		},
		onSuccess: async (value: unknown) => {
			const recipe = parse(RecipeSchema, value);
			navigate({ to: `/recipes/${recipe.id}` });
		},
	});

	const handleSubmit = (recipe: CreateRecipe) => {
		parse(CreateRecipeSchema, recipe);
		mutate(recipe);
	};

	return (
		<form onSubmit={form.onSubmit(handleSubmit)}>
			<ScrollArea ml={50} mr={50} scrollbarSize={6}>
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
								id: uuidv4(),
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
						<Grid.Col span={2}>
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
											.ingredients.toSorted((a, b) => a.position - b.position),
									});
								}}
								label="Position"
							/>
						</Grid.Col>
						<Grid.Col span={9}>
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
						<Grid.Col span={1} style={{ textAlign: "center" }}>
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
								id: uuidv4(),
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
						<Grid.Col span={2}>
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
											.instructions.toSorted((a, b) => a.position - b.position),
									});
								}}
								label="Position"
							/>
						</Grid.Col>
						<Grid.Col span={9}>
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
						<Grid.Col span={1} style={{ textAlign: "center" }}>
							<ActionIcon
								onClick={() => form.removeListItem("instructions", index)}
							>
								<IconTrash />
							</ActionIcon>
						</Grid.Col>
					</Grid>
				))}
				<Group justify="center">
					<Button type="submit" fullWidth mt="md">
						Save
					</Button>
				</Group>
			</ScrollArea>
		</form>
	);
}
