import RoomList from "./RoomList";
import UserList from "./UserList";
import useFetch from "./useFetch";

const Home = () => {
	const { data: users, isPending, error } = useFetch('http://localhost:8000/users');
	const { data: rooms, isPending: isRoomPending, error: roomError } = useFetch('http://localhost:8000/rooms');

	return (
		<div className="home">
			{roomError && <div>{roomError}</div>}
			{isRoomPending && <div>Loading Rooms...</div>}
			{rooms && <RoomList rooms={rooms} title="Available Rooms" />}

			{error && <div>{error}</div>}
			{isPending && <div>Loading Connected Users...</div>}
			{users && <UserList users={users.filter(u => u.status === 'connected')} title="Connected users" />}
		</div>
	);
}

export default Home;