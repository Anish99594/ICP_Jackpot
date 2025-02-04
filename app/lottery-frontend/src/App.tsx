import { useState, useEffect } from 'react';
import './App.css'; // Regular CSS for styling

function LotteryApp() {
    const [lotteryState] = useState(null);
    const [ticketId, setTicketId] = useState('');
    const [winner, setWinner] = useState(null);
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        fetchLotteryState();
    }, []);

    const fetchLotteryState = async () => {
        // setLoading(true);
        // // Fetch lottery state from backend
        // const response = await fetch('/api/lottery-state');
        // const data = await response.json();
        // setLotteryState(data);
        // setLoading(false);
    };

    const purchaseTicket = async () => {
        setLoading(true);
        const response = await fetch('/api/purchase-ticket', { method: 'POST' });
        const data = await response.json();
        if (data.success) {
            setTicketId(data.ticketId);
            alert('Ticket purchased successfully!');
        } else {
            alert('Error: ' + data.message);
        }
        setLoading(false);
    };

    const closeLottery = async () => {
        setLoading(true);
        const response = await fetch('/api/close-lottery', { method: 'POST' });
        const data = await response.json();
        if (data.success) {
            setWinner(data.winner);
            alert('Lottery closed! Winner selected.');
        } else {
            alert('Error: ' + data.message);
        }
        setLoading(false);
    };

    const claimPrize = async () => {
        setLoading(true);
        const response = await fetch('/api/claim-prize', { method: 'POST' });
        const data = await response.json();
        alert(data.message);
        setLoading(false);
    };

    return (
        <div className="lottery-container">
            <h1>Lottery DApp</h1>
            {loading ? <p>Loading...</p> : null}
            <p>Lottery State: {lotteryState ? lotteryState : 'Loading...'}</p>
            <button onClick={purchaseTicket} disabled={loading}>Buy Ticket</button>
            <button onClick={closeLottery} disabled={loading}>Close Lottery</button>
            <button onClick={claimPrize} disabled={loading}>Claim Prize</button>
            {ticketId && <p>Your Ticket ID: {ticketId}</p>}
            {winner && <p>Winner: {winner}</p>}
        </div>
    );
}

export default LotteryApp;
