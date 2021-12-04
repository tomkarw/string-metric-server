const chat = document.getElementById('chat');
const send = document.getElementById('send');
const string1 = document.getElementById('string1');
const string2 = document.getElementById('string2');
const uri = 'ws://' + location.host + '/ws';
const ws = new WebSocket(uri);

console.log("this gets executed");

ws.onopen = function() {
    chat.innerHTML = '<p><em>Connected!</em></p>';
};

ws.onmessage = function(msg) {
    const line = document.createElement('p');
    const data = JSON.parse(msg.data);
    console.log(data);
    for (const key in data) {
        line.innerText += `${key}: ${data[key]} `;
    }
    chat.appendChild(line);
};

ws.onclose = function() {
    chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
};

send.onclick = function() {
    const msg = {
        'string1': string1.value,
        'string2': string2.value,
    }

    ws.send(JSON.stringify(msg));

    string1.value = '';
    string2.value = '';
};
