document.body.style.margin = '0';
document.body.style.overflow = 'hidden';

// TODO: Switch to OffscreenCanvas once it supports fillText
//       var offscreen = canvas.transferControlToOffscreen();
var canvas = document.getElementById('canvas');
var ctx = canvas.getContext("2d", {"alpha": false});
var action_socket = undefined;
var event_socket = undefined;
var binds = [
  {"html": "onmousedown", "args": ["clientX", "clientY", "button"]},
  {"html": "onmousemove", "args": ["clientX", "clientY"]},
  {"html": "onmouseup",   "args": ["clientX", "clientY", "button"]},
  {"html": "onwheel",     "args": ["deltaX", "deltaY"]},
  {"html": "onkeydown",   "args": ["code", "key"]},
  {"html": "onkeyup",     "args": ["code", "key"]},
  {"html": "oncontextmenu"},
];

function WindowResize(e) {
  canvas.width = innerWidth;
  canvas.height = innerHeight;
  //event_socket.send("resized " + innerWidth + " " + innerHeight);
};

window.onresize = WindowResize;
window.onresize();

function ActionMessage(e) {
  console.log("Received", e.data);
  var msg = JSON.parse(e.data);
  if (Array.isArray(msg)) {
    var command = msg[0];
      var args = msg.slice(1);
      if (command == 'measureText') {
	  var start = performance.now();
	  var w = ctx.measureText(args[0]).width;
	  action_socket.send(w);
	  var end = performance.now();
	  console.log("Time taken to measure text: " + (end - start));
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
function ActionReconnect() {
  ctx.clearRect(0,0,canvas.width, canvas.height);
  ctx.save();
  ctx.fillStyle = 'white';
  ctx.textAlign = 'center';
  ctx.font = '20px sans-serif';
  ctx.translate(canvas.width/2, canvas.height/2);
  ctx.fillText('Stopped', 0, 0);
  ctx.restore();
  setTimeout(ActionConnect, 1000);
};
function ActionClose() {
  Reconnect();
};
function ActionOpen(e) {
  action_socket.onerror = undefined;
  action_socket.onclose = ActionClose;
};
function ActionConnect() {
  action_socket = new WebSocket("ws://localhost:" + action_port + "/");
  action_socket.onmessage = ActionMessage;
  action_socket.onopen = ActionOpen;
  action_socket.onerror = ActionReconnect;
};

ActionConnect();

function EventBind(bind) {
  window[bind.html] = function(e) {
    if (typeof bind.args != "undefined") {
      var o = [bind.html];
      for (var i in bind.args) {
        o.push(e[bind.args[i]]);
      }
      event_socketg.send(JSON.stringify(o));
    }
    e.preventDefault();
    return true;
  }
};
function EventReconnect() {
  setTimeout(EventConnect, 1000);
};
function EventClose() {
  binds.forEach(function(bind) { window[bind.html] = undefined; });
  EventReconnect();
};
function EventOpen(e) {
  event_socket.onerror = undefined;
  binds.forEach(EventBind);
  event_socket.onclose = EventClose;
};
function EventConnect() {
  action_socket = new WebSocket("ws://localhost:" + event_port + "/");
  action_socket.onopen = EventOpen;
  action_socket.onerror = EventReconnect;
};

EventConnect();
