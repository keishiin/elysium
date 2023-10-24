function Navbar() {
	return (
		<div className="bg-blue-500">
			<nav className="relative py-2 pl-3 pr-4 flex justify-between items-center bg-white">
				<a className="text-3xl font-bold leading-none" href="#">
					<span className="h-10">
						webapp
					</span>
				</a>
				<div className="lg:hidden">
					<button className="navbar-burger flex items-center text-blue-600 p-3">
						<svg className="block h-4 w-4 fill-current" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
							<title>Mobile menu</title>
							<path d="M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z"></path>
						</svg>
					</button>
				</div>
				<ul className="hidden absolute top-1/2 left-1/2 transform -translate-y-1/2 -translate-x-1/2 lg:flex lg:mx-auto lg:flex lg:items-center lg:w-auto lg:space-x-6">
					<li>
						<a className="py-2 pl-3 pr-4 text-black hover:text-blue-500" href="#">
							Home
						</a>
					</li>
					<li>
						<a className="py-2 pl-3 pr-4 text-black hover:text-gray-500" href="#">
							About Us
						</a>
					</li>
					<li>
						<a className="py-2 pl-3 pr-4 text-black hover:text-gray-500" href="#">
							Services
						</a>
					</li>
					<li>
						<a className="py-2 pl-3 pr-4 text-black hover:text-gray-500" href="#">
							Pricing
						</a>
					</li>
					<li>
						<a className="py-2 pl-3 pr-4 text-black hover:text-gray-500" href="#">
							Contact
						</a>
					</li>
				</ul>
				<a className="hidden lg:inline-block lg:ml-auto lg:mr-3 py-2 px-6 bg-gray-50 hover:bg-gray-100 text-sm text-gray-900 font-bold  rounded-xl transition duration-200" href="#">
					Sign In
				</a>
				<a className="hidden lg:inline-block py-2 px-6 bg-blue-500 hover:bg-blue-600 text-sm text-white font-bold rounded-xl transition duration-200" href="#">
					Sign up
				</a>
			</nav>
			<div className="navbar-menu relative z-50 hidden">
				<div className="navbar-backdrop fixed inset-0 bg-gray-800 opacity-25"></div>
				<nav className="fixed top-0 left-0 bottom-0 flex flex-col w-5/6 max-w-sm py-6 px-6 bg-white border-r overflow-y-auto">
					<div className="flex items-center mb-8">
						<button className="navbar-close">
							<svg className="h-6 w-6 text-gray-400 cursor-pointer hover:text-gray-500" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
							</svg>
						</button>
					</div>
					<div>
						<ul>
							<li className="mb-1">
								<a className="block p-4 text-sm font-semibold text-gray-400 hover:bg-blue-50 hover:text-blue-600 rounded" href="#">
									Home
								</a>
							</li>
							<li className="mb-1">
								<a className="block p-4 text-sm font-semibold text-gray-400 hover:bg-blue-50 hover:text-blue-600 rounded" href="#">
									About Us
								</a>
							</li>
							<li className="mb-1">
								<a className="block p-4 text-sm font-semibold text-gray-400 hover:bg-blue-50 hover:text-blue-600 rounded" href="#">
									Services
								</a>
							</li>
							<li className="mb-1">
								<a className="block p-4 text-sm font-semibold text-gray-400 hover:bg-blue-50 hover:text-blue-600 rounded" href="#">
									Pricing
								</a>
							</li>
							<li className="mb-1">
								<a className="block p-4 text-sm font-semibold text-gray-400 hover:bg-blue-50 hover:text-blue-600 rounded" href="#">
									Contact
								</a>
							</li>
						</ul>
					</div>
					<div className="mt-auto">
						<div className="pt-6">
							<a className="block px-4 py-3 mb-3 leading-loose text-xs text-center font-semibold leading-none bg-gray-50 hover:bg-gray-100 rounded-xl" href="#">
								Sign in
							</a>
							<a className="block px-4 py-3 mb-2 leading-loose text-xs text-center text-white font-semibold bg-blue-600 hover:bg-blue-700  rounded-xl" href="#">
								Sign Up
							</a>
						</div>
					</div>
				</nav>
			</div>
		</div>
	)
}

export default Navbar; 
