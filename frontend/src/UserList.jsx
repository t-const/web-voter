import { Link } from "react-router-dom";

const UserList = ({ users, title }) => {

	return (
		<div className="user-list">
			<h2>{title}</h2>
			{users.map((u) => (
				<div className="user-preview" key={u.id}>
					<Link to={`/users/${u.id}`}>
						<h2>{u.name}</h2>
						<p>Description: {u.body}</p>
					</Link>
				</div>
			))}
		</div>
	);
}

export default UserList;