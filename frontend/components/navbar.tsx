"use client";

import {
	Navbar as NextUINavbar,
	NavbarContent,
	NavbarMenu,
	NavbarMenuToggle,
	NavbarBrand,
	NavbarItem,
	NavbarMenuItem,
} from "@nextui-org/navbar";
import { Button } from "@nextui-org/button";
import { Link } from "@nextui-org/link";

import { link as linkStyles } from "@nextui-org/theme";

import { siteConfig } from "@/config/site";
import NextLink from "next/link";
import clsx from "clsx";

import { ThemeSwitch } from "@/components/theme-switch";
import { GithubIcon } from "@/components/icons";

import { Logo } from "@/components/icons";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export const Navbar = () => {
	let token;
	let userId; 

	useEffect(() => {
		token = localStorage.getItem("token");
		userId = localStorage.getItem("user");
	}, [])

	const handleSignout = () => {
		localStorage.removeItem("user");
		localStorage.removeItem("token");

		router.push("/");
	};

	const router = useRouter();

	return (
		<NextUINavbar maxWidth="xl" position="sticky">
			<NavbarContent className="basis-1/5 sm:basis-full" justify="start">
				<NavbarBrand as="li" className="gap-3 max-w-fit">
					<NextLink className="flex justify-start items-center gap-1" href="/">
						<Logo />
						<p className="font-bold text-inherit">Elysium</p>
					</NextLink>
				</NavbarBrand>
				<ul className="hidden lg:flex gap-4 justify-start ml-2">
					{siteConfig.navItems.map((item) => (
						<NavbarItem key={item.href}>
							<NextLink
								className={clsx(
									linkStyles({ color: "foreground" }),
									"data-[active=true]:text-primary data-[active=true]:font-medium",
								)}
								color="foreground"
								href={item.href}
							>
								{item.label}
							</NextLink>
						</NavbarItem>
					))}
				</ul>
			</NavbarContent>

			<NavbarContent
				className="hidden sm:flex basis-1/5 sm:basis-full"
				justify="end"
			>
				<NavbarItem className="hidden sm:flex gap-2">
					<Link isExternal href={siteConfig.links.github} aria-label="Github">
						<GithubIcon className="text-default-500" />
					</Link>
					<ThemeSwitch />
				</NavbarItem>
				{!userId && !token && (
					<>
						<NavbarItem className="hidden md:flex">
							<Button
								onClick={() => router.push("/login")}
								className="text-sm font-normal text-default-600 bg-default-100"
								href={siteConfig.links.sponsor}
								variant="flat"
							>
								Login
							</Button>
						</NavbarItem>
						<NavbarItem className="hidden md:flex">
							<Button
								onClick={() => router.push("/register")}
								className="text-sm font-normal text-default-600 bg-default-100"
								href={siteConfig.links.sponsor}
								variant="flat"
							>
								Signup
							</Button>
						</NavbarItem>
					</>
				)}
				{(userId || token) && (
					<NavbarItem className="hidden md:flex">
						<Button
							onClick={() => handleSignout()}
							className="text-sm font-normal text-default-600 bg-default-100"
							href={siteConfig.links.sponsor}
							variant="flat"
						>
							Sigout
						</Button>
					</NavbarItem>
				)}
			</NavbarContent>

			<NavbarContent className="sm:hidden basis-1 pl-4" justify="end">
				<Link isExternal href={siteConfig.links.github} aria-label="Github">
					<GithubIcon className="text-default-500" />
				</Link>
				<ThemeSwitch />
				<NavbarMenuToggle />
			</NavbarContent>

			<NavbarMenu>
				<div className="mx-3 mt-2 flex flex-col gap-2">
					{siteConfig.navMenuItems.map((item, index) => (
						<NavbarMenuItem key={`${item}-${index}`}>
							<Link color="foreground" href={item.href} size="lg">
								{item.label}
							</Link>
						</NavbarMenuItem>
					))}
				</div>
			</NavbarMenu>
		</NextUINavbar>
	);
};
