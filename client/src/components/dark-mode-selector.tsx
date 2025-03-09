import { ActionIcon, useMantineColorScheme } from "@mantine/core";
import { IconMoonFilled, IconSunFilled } from "@tabler/icons-react";

export function DarkModeSelector() {
	const { colorScheme, toggleColorScheme } = useMantineColorScheme();

	return (
		<ActionIcon onClick={toggleColorScheme}>
			{colorScheme === "light" ? <IconMoonFilled /> : <IconSunFilled />}
		</ActionIcon>
	);
}
