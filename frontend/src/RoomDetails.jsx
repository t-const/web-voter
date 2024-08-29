import { useNavigate, useParams } from "react-router-dom";
import useFetch from "./useFetch";
import UserDetails from "./UserDetails";
import { API_URL } from "./config";
import VoteGrid from "./VoteGrid";

const RoomDetails = () => {
	const { id } = useParams();
	const { data: room, error, isPending } = useFetch(`${API_URL}/room?id=` + id);
	const navigation = useNavigate();

	const handleClick = () => {
		fetch(`${API_URL}/room?id=` + room.id, {
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
					<VoteGrid title={"Votes"} votes={[0.5, 1, 2, 3, 5, 8, 'Coffee']}/>
					<button onClick={handleClick}>delete</button>
					{room.admin && <UserDetails id={room.admin}/>}
				</article>
			)}
		</div>
	);
}

export default RoomDetails;