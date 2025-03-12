import "@mantine/core/styles.css";
import "@mantine/dates/styles.css";
import "@mantine/notifications/styles.css";

import {
	DEFAULT_THEME,
	MantineProvider,
	localStorageColorSchemeManager,
} from "@mantine/core";
import { ModalsProvider } from "@mantine/modals";
import { Notifications } from "@mantine/notifications";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { createRootRoute } from "@tanstack/react-router";
import { useState } from "react";
import { App } from "../app";
export const Route = createRootRoute({
	component: RootComponent,
});

export type ColorScheme = "light" | "dark";

function RootComponent() {
	const [theme, setTheme] = useState(DEFAULT_THEME);

	const queryClient = new QueryClient();

	const colorSchemeManager = localStorageColorSchemeManager({
		key: "recipe-keeper-mode",
	});

	return (
		<QueryClientProvider client={queryClient}>
			<MantineProvider colorSchemeManager={colorSchemeManager} theme={theme}>
				<ModalsProvider>
					<Notifications />
					<App theme={theme} onThemeChanged={setTheme} />
				</ModalsProvider>
			</MantineProvider>
		</QueryClientProvider>
	);
}
