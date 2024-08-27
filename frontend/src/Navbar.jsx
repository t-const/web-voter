import { Link } from "react-router-dom";

const Navbar = () => {
	return (
		<nav className="navbar">
			<h1>Web Voter</h1>
			<div className="link">
				<Link to="/">Home</Link>
				<Link to="/create">New Room</Link>
			</div>
		</nav>
	);
}

export default Navbar;