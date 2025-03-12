import * as v from "valibot";

export const RecipeBaseSchema = v.object({
    id: v.number(),
    name: v.string(),
    description: v.nullable(v.string()),
    author: v.nullable(v.string()),
    difficulty: v.nullable(v.string()),
    estimatedDuration: v.nullable(v.string())
});

export type RecipeBase = v.InferOutput<typeof RecipeBaseSchema>;


export const IngredientSchema = v.object({
    id: v.number(),
    recipeId: v.number(),
    position: v.number(),
    description: v.string(),
});

export type Ingredient = v.InferOutput<typeof IngredientSchema>;

export const InstructionSchema = v.object({
    id: v.number(),
    recipeId: v.number(),
    position: v.number(),
    description: v.string(),
});

export type Instruction = v.InferOutput<typeof InstructionSchema>;



export const RecipeSchema = v.object({
    id: v.number(),
    name: v.string(),
    description: v.nullable(v.string()),
    author: v.nullable(v.string()),
    difficulty: v.nullable(v.string()),
    estimatedDuration: v.nullable(v.string()),
    ingredients: v.array(IngredientSchema),
    instructions: v.array(InstructionSchema),
});

export type Recipe = v.InferOutput<typeof RecipeSchema>;

export const UpdateIngredientSchema = v.object({
    id: v.nullable(v.number()),
    recipeId: v.number(),
    position: v.number(),
    description: v.string(),
});

export type UpdateIngredient = v.InferOutput<typeof UpdateIngredientSchema>;

export const UpdateInstructionSchema = v.object({
    id: v.nullable(v.number()),
    recipeId: v.number(),
    position: v.number(),
    description: v.string(),
});

export type UpdateInstruction = v.InferOutput<typeof UpdateInstructionSchema>;



export const UpdateRecipeSchema = v.object({
    id: v.number(),
    name: v.string(),
    description: v.nullable(v.string()),
    author: v.nullable(v.string()),
    difficulty: v.nullable(v.string()),
    estimatedDuration: v.nullable(v.string()),
    ingredients: v.array(UpdateIngredientSchema),
    instructions: v.array(UpdateInstructionSchema),
});

export type UpdateRecipe = v.InferOutput<typeof UpdateRecipeSchema>;

