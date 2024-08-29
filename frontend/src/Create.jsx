import { useEffect, useState } from "react";
import { useNavigate } from 'react-router-dom';
import useFetch from "./useFetch";
import { API_URL } from "./config"

const Create = () => {
	const [name, setName] = useState('Room creation');
	const [body, setBody] = useState('');
	const [admin, setAdmin] = useState({ name: '', id: '' });
	const [isPending, setIsPending] = useState(false)
	const { data: allUsers, isPending: isUsersFetchPending, error } = useFetch(`${API_URL}/users`);
	const navigate = useNavigate();

	useEffect(() => {
		if (allUsers && allUsers.length > 0) {
			setAdmin({ name: allUsers[0].name, id: allUsers[0].id });
		}
	}, [allUsers]);

	const handleSubmit = (e) => {
		e.preventDefault();
		const room = { name, body, admin: admin.id };

		setIsPending(true);

		fetch(`${API_URL}/room`, {
			method: 'POST',
			headers: { "Content-Type": "application/json" },
			body: JSON.stringify(room)
		}).then((response) => {
			if (!response.ok) {
				throw new Error("Failed to create new room");
			}
			return response.json();
		}).then((data) => {
			console.log('Added new room');
			setIsPending(false);
			navigate('/');
		})
			.catch(err => {
				console.error('Error:', err);
				setIsPending(false);
			});
	};

	const handleSelect = (e) => {
		const selectedId = e.target.value;
    const selectedName = e.target.options[e.target.selectedIndex].text;
    setAdmin({ name: selectedName, id: selectedId });
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
					value={admin.name}
					onChange={handleSelect}>
					{allUsers.map(u => (
						<option value={u.id} key={u.id}>
							{u.name}
						</option>
					))}
				</select>
				}
				{!isPending && <button>Add Room</button>}
				{isPending && <button disabled>Adding Room...</button>}
			</form>
		</div>
	);
}

export default Create;