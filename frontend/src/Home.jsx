import { useState, useEffect } from "react";
import UserList from "./UserList";

const Home = () => {
	// Would be replaced with DB users for session
	const [users, setUsers] = useState([
		{ name: 'User1', body: 'Some desc', id: 1, status: 'connected' },
		{ name: 'User2', body: 'Some desc', id: 2, status: 'connected' },
		{ name: 'User3', body: 'Some desc', id: 3, status: 'disconnected' },
		{ name: 'User4', body: 'Some desc', id: 4, status: 'connected' }
	]);

	const deleteUser = (id) => {
		const newUsers = users.filter(u => u.id !== id);
		setUsers(newUsers)
	}

	useEffect(() => {
		console.log('use effect ran');
		console.log(users)
	});

	return (
		<div className="home">
			<UserList users={users.filter(u => u.status === 'connected')} title="Connected users" handleDelete={deleteUser} />
		</div>
	);
}

export default Home;