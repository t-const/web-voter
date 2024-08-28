import { useParams } from "react-router-dom";
import useFetch from "./useFetch";

const UserDetails = ({id: propId}) => {
	const { id: paramId } = useParams();
	const id = propId || paramId;
	const { data: user, error, isPending } = useFetch('http://localhost:8000/user?id=' + id);

	return (
		<div className="user-details">
			{isPending && <div>Loading...</div>}
			{error && <div>{error}</div>}
			{user && (
				<article>
					<h2>{user.name}</h2>
					<div>{"Status: " + user.status}</div>
					<div>{user.body}</div>
				</article>
			)}
		</div>
	);
}

export default UserDetails;