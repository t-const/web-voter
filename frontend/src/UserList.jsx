const UserList = ({users, title, handleDelete}) => {

	return (
		<div className="user-list">
			<h2>{title}</h2>
			{users.map((u) => (
				<div className="user-preview" key={u.id}>
					<h2>{u.name}</h2>
					<p>Description: {u.body}</p>
					<button onClick={() => handleDelete(u.id)}>
						Delete user
					</button>
				</div>
			))}
		</div>
	);
}

export default UserList;