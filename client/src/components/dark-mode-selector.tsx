import {
	ActionIcon,
	Tooltip,
	useMantineColorScheme,
	useMantineTheme,
} from "@mantine/core";
import { IconMoonFilled, IconSunFilled } from "@tabler/icons-react";
import { useMemo } from "react";

export function DarkModeSelector() {
	const { colorScheme, toggleColorScheme } = useMantineColorScheme();

	const theme = useMantineTheme();
	const label = useMemo(
		() => (colorScheme === "dark" ? "Light Mode" : "Dark Mode"),
		[colorScheme],
	);

	return (
		<Tooltip color={theme.primaryColor} label={label}>
			<ActionIcon onClick={toggleColorScheme}>
				{colorScheme === "light" ? <IconMoonFilled /> : <IconSunFilled />}
			</ActionIcon>
		</Tooltip>
	);
}
