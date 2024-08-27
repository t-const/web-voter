import { useState, useEffect } from "react";

const useFetch = (url) => {
	const [data, setData] = useState(null);
	const [isPending, setIsPending] = useState(true);
	const [error, setError] = useState(null);

	useEffect(() => {
		setIsPending(true);
		setTimeout(() => {
			fetch(url) // Post request
				.then(res => {
					if (!res.ok) {
						throw Error('Could not fetch the data for that resource');
					}
					return res.json(); // return as json object for subsequent calls
				})
				.then((data) => { // handle json object
					setData(data);
					setIsPending(false);
					setError(null);
				})
				.catch(err => {
					setIsPending(false);
					setError(err.message);
				});
		}, 1000);
	}, [url]);

	return {data, isPending, error};
}

export default useFetch;