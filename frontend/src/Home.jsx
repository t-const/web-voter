import { useState, useEffect } from "react";
import UserList from "./UserList";
import useFetch from "./useFetch";

const Home = () => {
	const { data: users, isPending, error } = useFetch('http://localhost:8000/users');

	return (
		<div className="home">
			{error && <div>{error}</div>}
			{isPending && <div>Loading...</div>}
			{users && <UserList users={users.filter(u => u.status === 'connected')} title="Connected users" />}
		</div>
	);
}

export default Home;