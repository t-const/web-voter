import { useNavigate, useParams } from "react-router-dom";
import useFetch from "./useFetch";

const RoomDetails = () => {
	const { id } = useParams();
	const { data: room, error, isPending } = useFetch('http://localhost:8000/room?id=' + id);
	const navigation = useNavigate();

	const handleClick = () => {
		fetch('http://localhost:8000/room?id=' + room.id, {
			method: 'DELETE'
		}).then(() => {
			navigation('/');
		});
	}

	return (
		<div className="room-details">
			{isPending && <div>Loading...</div>}
			{error && <div>{error}</div>}
			{room && (
				<article>
					<h2>{room.name}</h2>
					<div>{room.body}</div>
					<button onClick={handleClick}>delete</button>
				</article>
			)}
		</div>
	);
}

export default RoomDetails;