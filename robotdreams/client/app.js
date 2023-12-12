document.getElementById('messageForm').addEventListener('submit', async function (event) {
    event.preventDefault();
    const messageInput = document.getElementById('messageInput');
    const responseDiv = document.getElementById('response');

    try {
        const response = await fetch('/send', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ message: messageInput.value }),
        });

        const result = await response.json();
        responseDiv.textContent = result.message;
    } catch (error) {
        console.error('Error sending message:', error);
    }
});