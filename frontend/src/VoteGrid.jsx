import VoteCard from "./VoteCard";

const VoteGrid = ({ title, votes }) => {

  return (<div className="vote-grid">
    <h2>{title}</h2>
    {votes.map(v => (
      <div>
        <VoteCard content={v} />
      </div>
    ))}
  </div>);
}

export default VoteGrid;