import { Link } from "react-router-dom";

const RoomList = ({ rooms, title }) => {

	return (
		<div className="room-list">
			<h2>{title}</h2>
			{rooms.map((r) => (
				<div className="room-preview" key={r.id}>
					<Link to={`/rooms/${r.id}`}>
						<h2>{r.name}</h2>
						<p>Description: {r.body}</p>
					</Link>
				</div>
			))}
		</div>
	);
}

export default RoomList;