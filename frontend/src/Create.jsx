import { useState } from "react";
import {useNavigate} from 'react-router-dom';

const Create = () => {
	const [name, setName] = useState('Room creation');
	const [body, setBody] = useState('');
	const [admin, setAdmin] = useState('User hey');
	const [isPending, setIsPending] = useState(false)
	const navigate = useNavigate();

	const handleSubmit = (e) => {
		e.preventDefault();
		const room = { name, body, admin };

		setIsPending(true);

		fetch('http://localhost:8000/rooms/', {
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
				<select
					value={admin}
					onChange={(e) => setAdmin(e.target.value)}>
					<option value="User hey">User hey</option>
					<option value="User nay">User nay</option>
				</select>
				{!isPending && <button>Add Room</button> }
				{isPending && <button disabled>Adding Room...</button> }
			</form>
		</div>
	);
}

export default Create;