import {
	ActionIcon,
	AppShell,
	DEFAULT_THEME,
	Group,
	type MantineTheme,
	NavLink,
	ScrollArea,
	Stack,
	Text,
	createTheme,
	mergeMantineTheme,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { IconChevronsLeft, IconMenu3 } from "@tabler/icons-react";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { Link, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools";
import { useEffect, useState } from "react";
import { DarkModeSelector } from "./components/dark-mode-selector";
import { ThemeSelector } from "./components/theme-selector";

export interface AppProps {
	theme: MantineTheme;
	onThemeChanged: (theme: MantineTheme) => void;
}

export function App(props: AppProps) {
	const [mobileOpened, { toggle: toggleMobile, close: closeMobile }] =
		useDisclosure();
	const [desktopOpened, { toggle: toggleDesktop }] = useDisclosure(true);

	const closeNav = () => {
		if (mobileOpened) {
			closeMobile();
		}
	};

	const storedTheme = localStorage.getItem("recipe-keeper-theme") ?? "blue";
	const [color, setColor] = useState(storedTheme);

	useEffect(() => {
		const customTheme = createTheme({
			primaryColor: color,
		});

		const merged = mergeMantineTheme(DEFAULT_THEME, customTheme);
		props.onThemeChanged(merged);
	}, [color, props.onThemeChanged]);

	const updateTheme = (theme: string) => {
		setColor(theme);
		localStorage.setItem("recipe-keeper-theme", theme);
	};

	return (
		<AppShell
			header={{ height: 48 }}
			navbar={{
				width: 300,
				breakpoint: "sm",
				collapsed: { mobile: !mobileOpened, desktop: !desktopOpened },
			}}
			padding="md"
		>
			<AppShell.Header>
				<Group h="100%" px="md" justify="space-between">
					<Group>
						<ActionIcon hiddenFrom="sm" onClick={toggleMobile}>
							{mobileOpened ? <IconChevronsLeft /> : <IconMenu3 />}
						</ActionIcon>
						<ActionIcon visibleFrom="sm" onClick={toggleDesktop}>
							{desktopOpened ? <IconChevronsLeft /> : <IconMenu3 />}
						</ActionIcon>

						<Text size="xl">Recipe Keeper</Text>
					</Group>
					<Group>
						<DarkModeSelector />
						<ThemeSelector theme={color} onThemeChanged={updateTheme} />
					</Group>
				</Group>
			</AppShell.Header>
			<AppShell.Navbar p="md">
				<ScrollArea h="100%">
					<Stack p="sm" gap="xs">
						<NavLink
							component={Link}
							to="/"
							label="Search"
							onClick={closeNav}
						/>
						<NavLink
							component={Link}
							to="/recipes/new"
							label="New Recipe"
							onClick={closeNav}
						/>
					</Stack>
				</ScrollArea>
			</AppShell.Navbar>
			<AppShell.Main>
				<ScrollArea
					offsetScrollbars
					scrollHideDelay={0}
					scrollbarSize={6}
					h="calc(100vh - var(--app-shell-header-height, 0px) - 32px)"
				>
					<Outlet />
					<TanStackRouterDevtools />
					<ReactQueryDevtools />
				</ScrollArea>
			</AppShell.Main>
		</AppShell>
	);
}
