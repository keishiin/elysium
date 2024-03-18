export default function SteanLayout({
	children,
}: {
	children: React.ReactNode;
}) {
	return (
		<section>
			<div>{children}</div>
		</section>
	);
}
