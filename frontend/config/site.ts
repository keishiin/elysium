export type SiteConfig = typeof siteConfig;

export const siteConfig = {
	name: "Elysium",
	description: "Make beautiful websites regardless of your design experience.",
	navItems: [
		{
			label: "Home",
			href: "/",
		},
		{
			label: "Profile",
			href: "/profile",
		},
		{
			label: "User",
			href: "/user",
		},
		{
			label: "Steam",
			href: "/steam",
		},
	],
	navMenuItems: [
		{
			label: "Home",
			href: "/",
		},
		{
			label: "Profile",
			href: "/profile",
		},
		{
			label: "User",
			href: "/user",
		},
		{
			label: "Steam",
			href: "/steam",
		},
	],
	links: {
		github: "https://github.com/keishiin/elysium/tree/react-to-nextjs",
		sponsor: "https://patreon.com/jrgarciadev",
	},
};
