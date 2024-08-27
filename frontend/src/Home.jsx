import { useState } from "react";

const Home = () => {
	// Would be replaced with DB users for session
	const [users, setUsers] = useState([
		{ name: 'User1', body: 'Some desc', id: 1 },
		{ name: 'User2', body: 'Some desc', id: 2 },
		{ name: 'User3', body: 'Some desc', id: 3 },
	]);

	return (
		<div className="home">
			{users.map((u) => (
				<div className="user-preview" key={u.id}>
					<h2>{u.name}</h2>
					<p>Description: {u.body}</p>
				</div>
			))}
		</div>
	);
}

export default Home;