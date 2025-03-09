import {
	ActionIcon,
	Popover,
	SimpleGrid,
	ThemeIcon,
	Tooltip,
	useMantineTheme,
} from "@mantine/core";
import { IconColorPicker } from "@tabler/icons-react";
import { useState } from "react";

export interface ThemeSelectorProps {
	theme: string;
	onThemeChanged: (theme: string) => void;
}

export function ThemeSelector(props: ThemeSelectorProps) {
	const [currentTheme, setCurrentTheme] = useState(props.theme);
	const updateTheme = (theme: string) => {
		setCurrentTheme(theme);
		props.onThemeChanged(theme);
	};

	const colors = [
		"dark",
		"gray",
		"red",
		"pink",
		"grape",
		"violet",
		"indigo",
		"blue",
		"cyan",
		"green",
		"lime",
		"yellow",
		"orange",
		"teal",
	];

	const theme = useMantineTheme();

	return (
		<Popover position="bottom-end" shadow="md">
			<Popover.Target>
				<Tooltip label="Color Picker" color={theme.primaryColor}>
					<ActionIcon>
						<IconColorPicker />
					</ActionIcon>
				</Tooltip>
			</Popover.Target>
			<Popover.Dropdown>
				<SimpleGrid cols={4}>
					{colors.map((color) => (
						<Tooltip color={color} key={color} label={color}>
							<ThemeIcon
								key={color}
								color={color}
								onClick={() => updateTheme(color)}
								style={{
									cursor: "pointer",
								}}
							>
								{color === currentTheme ? (
									<span
										style={{
											position: "absolute",
											top: "50%",
											left: "50%",
											transform: "translate(-50%, -50%)",
											color: "white",
											fontSize: "18px",
											fontWeight: "bold",
											textShadow: "0 0 5px rgba(0, 0, 0, 0.5)",
										}}
									>
										✓
									</span>
								) : null}
							</ThemeIcon>
						</Tooltip>
					))}
				</SimpleGrid>
			</Popover.Dropdown>
		</Popover>
	);
}
