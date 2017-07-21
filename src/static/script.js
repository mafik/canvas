document.body.style.margin = '0';
document.body.style.overflow = 'hidden';

// TODO: Switch to OffscreenCanvas once it supports fillText
//       var offscreen = canvas.transferControlToOffscreen();
var canvas = document.getElementById('canvas');
var ctx = canvas.getContext("2d", {"alpha": false});
var socket = undefined;
var binds = [
  {"html": "onmousedown", "args": ["clientX", "clientY", "button"]},
  {"html": "onmousemove", "args": ["clientX", "clientY"]},
  {"html": "onmouseup",   "args": ["clientX", "clientY", "button"]},
  {"html": "onwheel",     "args": ["deltaX", "deltaY"]},
  {"html": "onkeydown",   "args": ["code", "key"]},
  {"html": "onkeyup",     "args": ["code", "key"]},
  {"html": "oncontextmenu"},
];

function SocketMessage(e) {
  console.log("Received", e.data);
  var msg = JSON.parse(e.data);
  if (Array.isArray(msg)) {
    var command = msg[0];
      var args = msg.slice(1);
      if (command == 'measureText') {
	  console.log('text measured');
	  var w = ctx.measureText(args[0]).width;
	  socket.send(JSON.stringify(["textWidth", w]));
      }

    else if (typeof ctx[command] === 'function') {
      ctx[command].apply(ctx, args);
    } else {
      ctx[command] = args[0];
    }
  } else {
    console.error("Message is not an array", msg);
  }
};

function Reconnect() {
  ctx.clearRect(0,0,canvas.width, canvas.height);
  ctx.save();
  ctx.fillStyle = 'white';
  ctx.textAlign = 'center';
  ctx.font = '20px sans-serif';
  ctx.translate(canvas.width/2, canvas.height/2);
  ctx.fillText('Stopped', 0, 0);
  ctx.restore();
  setTimeout(Connect, 1000);
};

function Connect() {
  socket = new WebSocket("ws://localhost:8081/");
  socket.onmessage = SocketMessage;
  socket.onopen = SocketOpen;
  socket.onerror = Reconnect;
};

function SocketClose() {
  window.onresize = undefined;
  binds.forEach(function(bind) { window[bind.html] = undefined; });
  Reconnect();
};

function WindowResize(e) {
  socket.send(JSON.stringify(["size",innerWidth,innerHeight]));
  canvas.width = innerWidth;
  canvas.height = innerHeight;
  // TODO: redraw the screen
};

function Bind(bind) {
  window[bind.html] = function(e) {
    if (typeof bind.args != "undefined") {
      var o = [bind.html];
      for (var i in bind.args) {
        o.push(e[bind.args[i]]);
      }
      socket.send(JSON.stringify(o));
    }
    e.preventDefault();
    return true;
  }
};

function SocketOpen(e) {
  socket.onerror = undefined;
  window.onresize = WindowResize;
  window.onresize();
  binds.forEach(Bind);
  socket.onclose = SocketClose;
};

Connect();
