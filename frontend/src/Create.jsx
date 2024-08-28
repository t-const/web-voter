import { useEffect, useState } from "react";
import {useNavigate} from 'react-router-dom';
import useFetch from "./useFetch";

const Create = () => {
	const [name, setName] = useState('Room creation');
	const [body, setBody] = useState('');
	const [adminName, setAdminName] = useState('');
	const [admin, setAdmin] = useState('');
	const [isPending, setIsPending] = useState(false)
	const {data: allUsers, isPending: isUsersFetchPending, error} = useFetch('http://localhost:8000/users');
	const navigate = useNavigate();

	useEffect(() => {
		console.log(allUsers)
		setAdminName(allUsers?.[0]?.name || '');
		setAdmin(allUsers?.[0]?.id || '');
	}, [allUsers]);

	useEffect(() => {
		console.log(admin)
	}, [admin])

	const handleSubmit = (e) => {
		e.preventDefault();
		const room = { name, body, admin };
		console.log(room);

		setIsPending(true);

		fetch('http://localhost:8000/room', {
			method: 'POST',
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify(room)
		}).then(() => {
			console.log('Added new room');
			setIsPending(false);
		});

		navigate('/');
	};

	return (
		<div className="create">
			<h2>Add a new room</h2>
			<form onSubmit={handleSubmit}>
				<label>Room name:</label>
				<input type="text"
					required
					value={name}
					onChange={(e) => setName(e.target.value)} />
				<label>Room description:</label>
				<textarea
					required
					value={body}
					onChange={(e) => setBody(e.target.value)}>
				</textarea>
				<label>Room  admin:</label>
				{isUsersFetchPending && <div>Loading possible admins...</div>}
				{error && <div>{error}</div>}
				{!error && !isUsersFetchPending && <select
					value={adminName}
					onChange={(e) => {
						setAdminName(e.target.label);
						setAdmin(e.target.value);
						}}>
					{allUsers.map(u => (
						<option value={u.id}>
							{u.name}
						</option>
					))}
					</select>
				}
				{!isPending && <button>Add Room</button> }
				{isPending && <button disabled>Adding Room...</button> }
			</form>
		</div>
	);
}

export default Create;