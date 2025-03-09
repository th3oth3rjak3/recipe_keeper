import {
	ActionIcon,
	Card,
	CardSection,
	Checkbox,
	CloseButton,
	Group,
	Stack,
	TextInput,
	Tooltip,
	useMantineTheme,
} from "@mantine/core";
import { useForm } from "@mantine/form";
import { IconSearch } from "@tabler/icons-react";
import { valibotResolver } from "mantine-form-valibot-resolver";

import * as v from "valibot";

export type SearchParameters = v.InferOutput<typeof SearchSchema>;

const SearchSchema = v.object({
	query: v.pipe(v.string()),
	includeIngredients: v.boolean(),
	includeInstructions: v.boolean(),
});

export interface SearchRecipesComponentProps {
	onSearch: (params: SearchParameters) => void;
}

export default function SearchRecipesComponent(
	props: SearchRecipesComponentProps,
) {
	const searchForm = useForm({
		mode: "controlled",
		initialValues: {
			query: "",
			includeIngredients: false,
			includeInstructions: false,
		},
		validate: valibotResolver(SearchSchema),
	});

	const onSubmit = (values: SearchParameters) => {
		props.onSearch(values);
	};

	const theme = useMantineTheme();

	return (
		<Card maw={500} w="75%" miw={400} withBorder>
			<CardSection>
				<form onSubmit={searchForm.onSubmit(onSubmit)}>
					<Stack>
						<Group justify="space-between" pt="md" pr="md" pl="md">
							<TextInput
								flex={1}
								placeholder="Search Recipes"
								rightSectionPointerEvents="all"
								rightSection={
									<CloseButton
										aria-label="Clear input"
										onClick={() =>
											searchForm.setValues({
												...searchForm.values,
												query: "",
											})
										}
										style={{
											display: searchForm.values.query ? undefined : "none",
										}}
									/>
								}
								key={searchForm.key("query")}
								{...searchForm.getInputProps("query")}
							/>
							<Tooltip color={theme.primaryColor} label="Search">
								<ActionIcon type="submit" size="lg">
									<IconSearch />
								</ActionIcon>
							</Tooltip>
						</Group>
						<Group justify="center" flex={1} mb="md">
							<Checkbox
								label="Include Ingredients"
								key={searchForm.key("includeIngredients")}
								{...searchForm.getInputProps("includeIngredients", {
									type: "checkbox",
								})}
							/>
							<Checkbox
								label="Include Instructions"
								key={searchForm.key("includeInstructions")}
								{...searchForm.getInputProps("includeInstructions", {
									type: "checkbox",
								})}
							/>
						</Group>
					</Stack>
				</form>
			</CardSection>
		</Card>
	);
}
